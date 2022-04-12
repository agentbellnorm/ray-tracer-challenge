#[cfg(test)]
mod pattern_test {
    use crate::color::Color;
    use crate::pattern::stripe_pattern;
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
}
