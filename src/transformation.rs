use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn view_transformation(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    assert!(from.is_point());
    assert!(to.is_point());
    assert!(up.is_vector());

    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let orientation = Matrix::from_vec(
        [
            left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x,
            -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        4,
    );

    &orientation * &Matrix::identity().translate(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod transformation_test {
    use crate::matrix::Matrix;
    use crate::transformation::view_transformation;
    use crate::tuple::{point, vector};

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::identity().scale(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::identity().translate(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(
            t,
            Matrix::from_values(vec![
                vec![-0.50709, 0.50709, 0.67612, -2.36643],
                vec![0.76772, 0.60609, 0.12122, -2.82843],
                vec![-0.35857, 0.59761, -0.71714, 0.00000],
                vec![0.00000, 0.00000, 0.00000, 1.00000],
            ])
        );
    }
}
