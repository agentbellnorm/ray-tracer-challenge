#[cfg(test)]
mod color_test {
    use crate::color::color;

    #[test]
    pub fn create() {
        let c = color(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    pub fn equality() {
        assert_eq!(color(-0.5, 0.4, 1.7), color(-0.5, 0.4, 1.7));

        assert_ne!(color(-0.6, 0.4, 1.7), color(-0.5, 0.4, 1.7));
    }

    #[test]
    pub fn addition() {
        assert_eq!(
            color(0.9, 0.6, 0.75) + color(0.7, 0.1, 0.25),
            color(1.6, 0.7, 1.0)
        )
    }

    #[test]
    pub fn subtraction() {
        assert_eq!(
            color(0.9, 0.6, 0.75) - color(0.7, 0.1, 0.25),
            color(0.2, 0.5, 0.5)
        )
    }

    #[test]
    pub fn negation() {
        assert_eq!(-color(0.2, 0.5, -0.5), color(-0.2, -0.5, 0.5));
    }

    #[test]
    pub fn multiplication_by_scalar() {
        assert_eq!(color(0.2, 0.3, 0.4) * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    pub fn multiplication() {
        assert_eq!(
            color(1.0, 0.2, 0.4) * color(0.9, 1.0, 0.1),
            color(0.9, 0.2, 0.04)
        );
    }
}
