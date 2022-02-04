#[cfg(test)]
mod translation {
    use crate::translation::{rotation_x, rotation_y, rotation_z, scaling, translation};
    use crate::tuple::{point, vector};
    use std::f32::consts::PI;

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

    #[test]
    fn scaling_matrix_applied_to_point() {
        let scaling = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);

        assert_eq!(p * scaling, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let scaling = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(v * scaling, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_with_invers_of_scaling_matrix() {
        let s = scaling(2.0, 3.0, 4.0);
        let inv_s = s.inverse();
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(v * inv_s, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_by_scaling_with_negative() {
        let s = scaling(-1.0, 1.0, 1.0);
        let p = vector(2.0, 3.0, 4.0);

        assert_eq!(p * s, vector(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            p * half_quarter,
            point(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(p * full_quarter, point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert_eq!(
            p * inv,
            point(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0)
        )
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            p * half_quarter,
            point(f32::sqrt(2.0) / 2.0, 0.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(p * full_quarter, point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            p * half_quarter,
            point(-f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(p * full_quarter, point(-1.0, 0.0, 0.0));
    }
}
