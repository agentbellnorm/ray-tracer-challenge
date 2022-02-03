#[cfg(test)]
mod translation {
    use crate::translation::translation;
    use crate::tuple::{point, vector};

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);

        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(p * transform, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_with_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();

        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(p * inv, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);

        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(v * transform, v);
    }
}
