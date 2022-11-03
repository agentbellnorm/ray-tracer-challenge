#[cfg(test)]
mod shape_test {
    use ray_tracer_challenge::matrix::Matrix;
    use ray_tracer_challenge::shape::{Shape, ShapeType};
    use ray_tracer_challenge::tuple::point_i;
    use std::f64::consts::FRAC_PI_2;

    fn converting_point_from_world_to_object_space() {
        let g1 = Shape::group()
            .with_transform(Matrix::identity().rotate_y(FRAC_PI_2))
            .to_rc();

        let g2_children = vec![
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0))
        ];
        let g2 = Shape::group_with_children(g2_children)
            .with_transform(Matrix::identity().scale(2.0, 2.0, 2.0));

        Shape::add_shape_to_group(&g1, g2);
        let g1_children = g1.get_children().unwrap();
        let g2_in_tree = g1_children.get(0).unwrap();
        assert!(matches!(g2_in_tree.shape_type, ShapeType::Group(_)));
        let g2_children = g2_in_tree.get_children().unwrap();
        let g2_first_child = g2_children.get(0).unwrap();
        assert!(matches!(g2_first_child.shape_type, ShapeType::Sphere));
        assert_eq!(
            point_i(0, 0, -1),
            g2_first_child.world_to_object(point_i(-2, 0, -10))
        );
    }
}
