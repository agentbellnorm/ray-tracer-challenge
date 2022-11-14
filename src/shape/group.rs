#[cfg(test)]
mod group_test {
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::shape::{Shape, ShapeType};
    use crate::tuple::{point_i, vector_i};

    #[test]
    pub fn creating_new_group() {
        let g = Shape::group();
        assert_eq!(g.inverse_transformation, Matrix::identity().inverse());
        assert!(!g.has_children())
    }

    #[test]
    pub fn shape_has_parent_attribute() {
        let g = Shape::group();
        assert!(g.get_parent().is_none())
    }

    #[test]
    fn add_child_to_group() {
        let s = Shape::sphere_default();
        let g = Shape::group_with_children(vec![s]);

        // {
        //     s.parent = Some(g.clone());
        //     let mut shape_type = &g.as_ref().shape_type;
        //     if let ShapeType::Group(mut children) = shape_type {
        //         children.push(Rc::new(RefCell::new(s)))
        //     };
        //     // g = Shape::add_shape_to_group(g, s);
        // }

        assert!(g.borrow().has_children());
        matches!(
            g.borrow()
                .get_children()
                .unwrap()
                .get(0)
                .unwrap()
                .borrow()
                .shape_type,
            ShapeType::Sphere
        );
        matches!(
            g.borrow()
                .get_children()
                .unwrap()
                .get(0)
                .unwrap()
                .borrow()
                .get_parent()
                .unwrap()
                .borrow()
                .shape_type,
            ShapeType::Group(_)
        );
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let g = Shape::group();
        let r = Ray::with(point_i(0, 0, 0), vector_i(0, 0, 1));

        let xs = Shape::intersects(g.pack(), &r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let s1 = Shape::sphere_default();
        let s2 =
            Shape::sphere_default().with_transform(Matrix::identity().translate(0.0, 0.0, -3.0));
        let s3 =
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let children = vec![s1.clone(), s2.clone(), s3.clone()];

        let group = Shape::group_with_children(children);
        let ray = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));

        let xs = Shape::intersects(group, &ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(&*xs.get(0).object.borrow(), &s2);
        assert_eq!(&*xs.get(1).object.borrow(), &s2);
        assert_eq!(&*xs.get(2).object.borrow(), &s1);
        assert_eq!(&*xs.get(3).object.borrow(), &s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let sphere =
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let group = Shape::group()
            .with_transform(Matrix::identity().scale(2.0, 2.0, 2.0))
            .pack();
        Shape::add_child_rc_to_group(group.clone(), sphere.pack());
        let ray = Ray::with(point_i(10, 0, -10), vector_i(0, 0, 1));

        assert!(group
            .borrow()
            .get_children()
            .unwrap()
            .get(0)
            .unwrap()
            .borrow()
            .parent
            .is_some());
        assert_eq!(Shape::intersects(group.clone(), &ray).xs.len(), 2);
    }
}
