#[cfg(test)]
mod pattern_test {
    use crate::color::Color;
    use crate::matrix::Matrix;
    use crate::pattern::stripe_pattern;
    use crate::shapes::{sphere_default, sphere_from_transform};
    use crate::tuple::point;

    #[test]
    fn creating_stripe_pattern() {
        let _striped = stripe_pattern(Color::white(), Color::black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let striped = stripe_pattern(Color::white(), Color::black());
        assert_eq!(striped.color_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(striped.color_at(point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(striped.color_at(point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let striped = stripe_pattern(Color::white(), Color::black());
        assert_eq!(striped.color_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(striped.color_at(point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(striped.color_at(point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let striped = stripe_pattern(Color::white(), Color::black());
        assert_eq!(striped.color_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(striped.color_at(point(0.9, 0.0, 1.0)), Color::white());
        assert_eq!(striped.color_at(point(1.0, 0.0, 2.0)), Color::black());
        assert_eq!(striped.color_at(point(-0.1, 0.0, 2.0)), Color::black());
        assert_eq!(striped.color_at(point(-1.0, 0.0, 2.0)), Color::black());
        assert_eq!(striped.color_at(point(-1.1, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let o = sphere_from_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let p = stripe_pattern(Color::white(), Color::black());

        let c = p.color_at_object(&o, point(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let o = sphere_default();
        let p = stripe_pattern(Color::white(), Color::black())
            .with_transformation(Matrix::identity().scale(2.0, 2.0, 2.0));

        let c = p.color_at_object(&o, point(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let o = sphere_from_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let p = stripe_pattern(Color::white(), Color::black())
            .with_transformation(Matrix::identity().translate(0.5, 0.0, 0.0));

        let c = p.color_at_object(&o, point(2.5, 0.0, 0.0));

        assert_eq!(c, Color::white())
    }
}
