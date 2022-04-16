#[cfg(test)]
mod pattern_test {
    use crate::color::{black, color, white};
    use crate::matrix::Matrix;
    use crate::pattern::Pattern;
    use crate::tuple::point;
    use crate::Shape;

    #[test]
    fn creating_stripe_pattern() {
        let _striped = Pattern::striped(white(), black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let striped = Pattern::striped(white(), black());
        assert_eq!(striped.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(striped.color_at(point(0.0, 1.0, 0.0)), white());
        assert_eq!(striped.color_at(point(0.0, 2.0, 0.0)), white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let striped = Pattern::striped(white(), black());
        assert_eq!(striped.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(striped.color_at(point(0.0, 0.0, 1.0)), white());
        assert_eq!(striped.color_at(point(0.0, 0.0, 2.0)), white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let striped = Pattern::striped(white(), black());
        assert_eq!(striped.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(striped.color_at(point(0.9, 0.0, 1.0)), white());
        assert_eq!(striped.color_at(point(1.0, 0.0, 2.0)), black());
        assert_eq!(striped.color_at(point(-0.1, 0.0, 2.0)), black());
        assert_eq!(striped.color_at(point(-1.0, 0.0, 2.0)), black());
        assert_eq!(striped.color_at(point(-1.1, 0.0, 2.0)), white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let o = Shape::sphere_from_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let p = Pattern::striped(white(), black());

        let c = p.color_at_object(&o, point(1.5, 0.0, 0.0));

        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let o = Shape::sphere_default();
        let p = Pattern::striped(white(), black())
            .with_transformation(Matrix::identity().scale(2.0, 2.0, 2.0));

        let c = p.color_at_object(&o, point(1.5, 0.0, 0.0));

        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let o = Shape::sphere_from_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let p = Pattern::striped(white(), black())
            .with_transformation(Matrix::identity().translate(0.5, 0.0, 0.0));

        let c = p.color_at_object(&o, point(2.5, 0.0, 0.0));

        assert_eq!(c, white())
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let p = Pattern::gradient(white(), black());
        assert_eq!(p.color_at(point(0.00, 0.0, 0.0)), white());
        assert_eq!(p.color_at(point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
        assert_eq!(p.color_at(point(0.50, 0.0, 0.0)), color(0.5, 0.5, 0.5));
        assert_eq!(p.color_at(point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_x_and_z() {
        let p = Pattern::ring(white(), black());
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(p.color_at(point(1.0, 0.0, 0.0)), black());
        assert_eq!(p.color_at(point(0.0, 0.0, 1.0)), black());
        assert_eq!(p.color_at(point(0.708, 0.0, 709.0)), black());
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p = Pattern::checkers(white(), black());
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(p.color_at(point(0.99, 0.0, 0.0)), white());
        assert_eq!(p.color_at(point(1.01, 0.0, 0.0)), black());
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let p = Pattern::checkers(white(), black());
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(p.color_at(point(0.0, 0.99, 0.0)), white());
        assert_eq!(p.color_at(point(0.0, 1.01, 0.0)), black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p = Pattern::checkers(white(), black());
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(p.color_at(point(0.0, 0.0, 0.99)), white());
        assert_eq!(p.color_at(point(0.0, 0.0, 1.01)), black());
    }
}
