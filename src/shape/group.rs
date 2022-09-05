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
        if let ShapeType::Group(children) = group.shape_type {
            assert!(children.is_empty())
        } else {
            panic!("group was not group")
        }
    }

    #[test]
    fn shape_has_parent_attribute() {
        let shape = Shape::sphere_glass();
        let world = World::default().add_shape(shape);

        assert!(world.get_shape(world.current_index()).parent == None)
    }

    #[test]
    fn add_child_to_group() {
        let mut world = World::default();
        let group = Shape::group();
        let shape = Shape::sphere_glass();

        world = world.with_group_and_children(group, vec![shape]);

        let group = &world.objects[0];
        match &group.shape.shape_type {
            ShapeType::Group(children) => {
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
        let world = World::default();
        let group = Shape::group();

        let ray = Ray::with(point_i(0, 0, 0), vector_i(0, 0, 1));

        let xs = group.intersects(&world, &ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let mut world = World::default();
        let group = Shape::group();
        let s1 = Shape::sphere_chrome();
        let s2 = Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, -3.0));
        let s3 =
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));

        world = world.with_group_and_children(group, vec![s1, s2, s3]);

        let ray = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));

        let xs = world.intersect_world(&ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs.get(0).object, 2);
        assert_eq!(xs.get(1).object, 2);
        assert_eq!(xs.get(2).object, 1);
        assert_eq!(xs.get(3).object, 1);
    }

    #[test]
    fn intersecting_transformed_group() {
        let group = Shape::group().with_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let sphere =
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let world = World::default().with_group_and_children(group, vec![sphere]);
        let ray = Ray::with(point_i(10, 0, -10), vector_i(0, 0, 1));

        let xs = world.intersect_world(&ray);

        assert_eq!(xs.len(), 2);
    }
}