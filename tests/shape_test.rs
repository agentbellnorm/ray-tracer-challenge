#[cfg(test)]
mod shape_test {
    use ray_tracer_challenge::matrix::Matrix;
    use ray_tracer_challenge::shape::{Shape, ShapeType};
    use ray_tracer_challenge::tuple::point_i;
    use std::borrow::BorrowMut;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn converting_point_from_world_to_object_space() {
        let g1 = Shape::group()
            .with_transform(Matrix::identity().rotate_y(FRAC_PI_2))
            .pack();

        let g2 = Shape::group()
            .with_transform(Matrix::identity().scale(2.0, 2.0, 2.0))
            .pack();

        let sphere = Shape::sphere_default()
            .with_transform(Matrix::identity().translate(5.0, 0.0, 0.0))
            .pack();

        Shape::add_child_rc_to_group(g2.clone(), sphere.clone());
        Shape::add_child_rc_to_group(g1.clone(), g2.clone());

        println!("wat");

        let root = g1.borrow();
        let mid = root.get_children().unwrap().get(0).unwrap().borrow();
        let leaf = mid.get_children().unwrap().get(0).unwrap().borrow();
        // assert!(matches!(
        //     g2_in_tree.as_ref().borrow().shape_type,
        //     ShapeType::Group(_)
        // ));
        //
        // let g2_children = g2_in_tree.as_ref().borrow().get_children().unwrap();
        // let g2_first_child = g2_children.get(0).unwrap().clone();
        // assert_eq!(leaf, &sphere);
        assert!(leaf.parent.is_some());
        assert_eq!(point_i(0, 0, -1), leaf.world_to_object(point_i(-2, 0, -10)));
    }
}
