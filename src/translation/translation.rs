use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::identity().set(0, 3, x).set(1, 3, y).set(2, 3, z)
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::identity().set(0, 0, x).set(1, 1, y).set(2, 2, z)
}

pub fn rotation_x(rad: f32) -> Matrix {
    Matrix::identity()
        .set(1, 1, f32::cos(rad))
        .set(1, 2, -f32::sin(rad))
        .set(2, 1, f32::sin(rad))
        .set(2, 2, f32::cos(rad))
}

pub fn rotation_y(rad: f32) -> Matrix {
    Matrix::identity()
        .set(0, 0, f32::cos(rad))
        .set(0, 2, f32::sin(rad))
        .set(2, 0, -f32::sin(rad))
        .set(2, 2, f32::cos(rad))
}

pub fn rotation_z(rad: f32) -> Matrix {
    Matrix::identity()
        .set(0, 0, f32::cos(rad))
        .set(0, 1, -f32::sin(rad))
        .set(1, 0, f32::sin(rad))
        .set(1, 1, f32::cos(rad))
}
