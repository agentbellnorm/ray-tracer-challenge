#[cfg(test)]
mod group_test {
    use crate::rays::Ray;
    use crate::shape::ShapeType;
    use crate::tuple::{point_i, vector_i};
    use crate::{Matrix, Shape, World};

    #[test]
    fn creating_new_group() {
        let group = Shape::group();

        assert_eq!(group.inverse_transformation, Matrix::identity().inverse());
        if let ShapeType::Group(children, _) = group.shape_type {
            assert!(children.is_empty())
        } else {
            panic!("group was not group")
        }
    }

    #[test]
    fn shape_has_parent_attribute() {
        let mut world = World::default();
        let shape = world.add_shape(Shape::sphere_glass());

        assert!(world.get_shape(shape).parent == None)
    }

    #[test]
    fn add_child_to_group() {
        let mut world = World::default();
        let group = world.add_shape(Shape::group());
        let shape = world.add_shape(Shape::sphere_glass());
        world.add_shape_to_group(group, shape);

        let group = &world.objects[0];
        match &group.shape.shape_type {
            ShapeType::Group(children, _) => {
                assert!(!children.is_empty());
                assert!(children
                    .into_iter()
                    .any(|child_id| world.get_shape(*child_id).shape_type == ShapeType::Sphere));
            }
            _ => panic!("group was not actually group"),
        }

        assert_eq!(world.get_shape(1).parent, Some(0))
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let mut world = World::default();
        let group = Shape::group();
        let group_id = world.add_shape(group);

        let ray = Ray::with(point_i(0, 0, 0), vector_i(0, 0, 1));

        let xs = world.get_shape(group_id).intersects(&world, &ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let mut world = World::default();
        let group = world.add_shape(Shape::group());
        let s1 = world.add_shape(Shape::sphere_chrome());
        let s2 = world.add_shape(
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, -3.0)),
        );
        let s3 = world.add_shape(
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0)),
        );

        world.add_shape_to_group(group, s1);
        world.add_shape_to_group(group, s2);
        world.add_shape_to_group(group, s3);

        let ray = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));

        let xs = world.intersect_world(&ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs.get(0).object_id, 2);
        assert_eq!(xs.get(1).object_id, 2);
        assert_eq!(xs.get(2).object_id, 1);
        assert_eq!(xs.get(3).object_id, 1);
    }

    #[test]
    fn intersecting_transformed_group() {
        let mut world = World::default();
        let group =
            world.add_shape(Shape::group().with_transform(Matrix::identity().scale(2.0, 2.0, 2.0)));
        let sphere = world.add_shape(
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0)),
        );
        world.add_shape_to_group(group, sphere);

        let ray = Ray::with(point_i(10, 0, -10), vector_i(0, 0, 1));

        let xs = world.intersect_world(&ray);

        assert_eq!(xs.len(), 2);
    }
}
