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
