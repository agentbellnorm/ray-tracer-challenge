use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    let mut t = Matrix::identity();
    t = t.set(0, 3, x);
    t = t.set(1, 3, y);
    t = t.set(2, 3, z);

    t
}
