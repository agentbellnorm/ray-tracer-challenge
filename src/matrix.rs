use crate::tuple::{Tuple, EPSILON};
use std::ops::Mul;

type Storage = [f64; 16];

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    storage: Storage,
    size: usize,
}

const IDENTITY: [f64; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

impl Matrix {
    pub fn from_vec(v: Storage, size: usize) -> Matrix {
        Matrix { storage: v, size }
    }

    // for tests
    pub fn from_values(from: Vec<Vec<f64>>) -> Matrix {
        let n_cols = from.first().unwrap().len();

        let mut storage = [0.0; 16];

        let flat = from.into_iter().flatten().collect::<Vec<f64>>();

        storage[..flat.len()].clone_from_slice(&flat[..]);

        Matrix {
            storage,
            size: n_cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.storage[self.i(row, col)]
    }

    fn i(&self, row: usize, col: usize) -> usize {
        (self.size * row) + col
    }

    pub fn identity() -> Matrix {
        Matrix {
            size: 4,
            storage: IDENTITY,
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut transposed = [0.0; 16];

        for i in 0..self.size {
            for j in 0..self.size {
                transposed[self.i(i, j)] = self.storage[i + j * self.size];
            }
        }

        Matrix {
            size: self.size,
            storage: transposed,
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            return self.get(0, 0) * self.get(1, 1) - self.get(1, 0) * self.get(0, 1);
        }

        let mut determinant = 0.0;

        for col in 0..self.size {
            determinant += self.get(0, col) * self.cofactor(0, col);
        }

        determinant
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut submatrix = [0.0; 16];

        let mut new_index = 0;

        for r in 0..self.size {
            for c in 0..self.size {
                if r == row || c == col {
                    continue;
                }

                submatrix[new_index] = self.get(r, c);
                new_index += 1;
            }
        }

        Matrix {
            size: self.size - 1,
            storage: submatrix,
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            return self.minor(row, col);
        }

        -self.minor(row, col)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        let mut inverse = [0.0; 16];

        let determinant = self.determinant();

        for row in 0..self.size {
            for col in 0..self.size {
                // transpose by swapping row and col in self.i call
                inverse[self.i(col, row)] = self.cofactor(row, col) / determinant;
            }
        }

        Matrix {
            storage: inverse,
            size: self.size,
        }
    }

    fn set(mut self, row: usize, col: usize, v: f64) -> Matrix {
        let i = self.i(row, col);
        self.storage[i] = v;
        self
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Matrix {
        &Matrix::identity().set(0, 3, x).set(1, 3, y).set(2, 3, z) * self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Matrix {
        &Matrix::identity().set(0, 0, x).set(1, 1, y).set(2, 2, z) * self
    }

    pub fn rotate_x(&self, rad: f64) -> Matrix {
        &Matrix::identity()
            .set(1, 1, f64::cos(rad))
            .set(1, 2, -f64::sin(rad))
            .set(2, 1, f64::sin(rad))
            .set(2, 2, f64::cos(rad))
            * self
    }

    pub fn rotate_y(&self, rad: f64) -> Matrix {
        &Matrix::identity()
            .set(0, 0, f64::cos(rad))
            .set(0, 2, f64::sin(rad))
            .set(2, 0, -f64::sin(rad))
            .set(2, 2, f64::cos(rad))
            * self
    }

    pub fn rotate_z(&self, rad: f64) -> Matrix {
        &Matrix::identity()
            .set(0, 0, f64::cos(rad))
            .set(0, 1, -f64::sin(rad))
            .set(1, 0, f64::sin(rad))
            .set(1, 1, f64::cos(rad))
            * self
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        &Matrix::identity()
            .set(0, 1, xy)
            .set(0, 2, xz)
            .set(1, 0, yx)
            .set(1, 2, yz)
            .set(2, 0, zx)
            .set(2, 1, zy)
            * self
    }

    pub fn apply(&self, other: &Matrix) -> Matrix {
        other * self
    }
}

pub fn is_equal_float(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.storage.len() != other.storage.len() || self.size != other.size {
            return false;
        }

        for i in 0..self.storage.len() {
            if !is_equal_float(self.storage[i], other.storage[i]) {
                return false;
            }
        }

        true
    }
}

impl<'a> Mul for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, 4);
        assert_eq!(rhs.size, 4);

        let mut result_vals = [0.0; 16];

        for row in 0..self.size {
            for col in 0..self.size {
                result_vals[self.i(row, col)] = self.get(row, 0) * rhs.get(0, col)
                    + self.get(row, 1) * rhs.get(1, col)
                    + self.get(row, 2) * rhs.get(2, col)
                    + self.get(row, 3) * rhs.get(3, col)
            }
        }

        Matrix {
            storage: result_vals,
            size: self.size,
        }
    }
}

impl Mul<&Matrix> for Tuple {
    type Output = Tuple;

    fn mul(self, matrix: &Matrix) -> Tuple {
        let mut result_vals = Vec::with_capacity(4);

        for row in 0..matrix.size {
            result_vals.push(
                matrix.get(row, 0) * self.x
                    + matrix.get(row, 1) * self.y
                    + matrix.get(row, 2) * self.z
                    + matrix.get(row, 3) * self.w,
            )
        }

        Tuple {
            x: result_vals[0],
            y: result_vals[1],
            z: result_vals[2],
            w: result_vals[3],
        }
    }
}
