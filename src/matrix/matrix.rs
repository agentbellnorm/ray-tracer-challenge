use crate::tuple::{Tuple, EPSILON};
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    storage: Vec<f32>,
    size: usize,
}

impl Matrix {
    pub fn from_vec(v: Vec<f32>, size: usize) -> Matrix {
        Matrix { storage: v, size }
    }

    pub fn from_values(from: Vec<Vec<f32>>) -> Matrix {
        let size = from.len();
        let n_cols = from.first().unwrap().len();

        Matrix {
            storage: from
                .into_iter()
                .fold(Vec::with_capacity(size * n_cols), |acc, v| {
                    [acc, v].concat()
                }),
            size: n_cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.storage[self.i(row, col)]
    }

    fn i(&self, row: usize, col: usize) -> usize {
        (self.size * row) + col
    }

    pub fn identity() -> Matrix {
        Matrix {
            size: 4,
            storage: vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut transposed = Vec::with_capacity(self.size * self.size);

        for i in 0..self.size {
            for j in 0..self.size {
                transposed.push(self.storage[i + j * self.size]);
            }
        }

        Matrix {
            size: self.size,
            storage: transposed,
        }
    }

    pub fn determinant(&self) -> f32 {
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
        let mut submatrix = Vec::with_capacity((self.size - 1) * (self.size - 1));

        for r in 0..self.size {
            for c in 0..self.size {
                if r == row || c == col {
                    continue;
                }

                submatrix.push(self.get(r, c));
            }
        }

        Matrix {
            size: self.size - 1,
            storage: submatrix,
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            return self.minor(row, col);
        }

        -self.minor(row, col)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        let mut inverse = vec![0.0; self.size * self.size];

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

    pub fn set(mut self, row: usize, col: usize, v: f32) -> Matrix {
        let i = self.i(row, col);
        self.storage[i] = v;
        self
    }
}

pub fn is_equal_float(a: f32, b: f32) -> bool {
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

        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a> Mul for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, 4);
        assert_eq!(rhs.size, 4);

        let mut result_vals = Vec::with_capacity(self.size * self.size);

        for row in 0..self.size {
            for col in 0..self.size {
                result_vals.push(
                    self.get(row, 0) * rhs.get(0, col)
                        + self.get(row, 1) * rhs.get(1, col)
                        + self.get(row, 2) * rhs.get(2, col)
                        + self.get(row, 3) * rhs.get(3, col),
                )
            }
        }

        Matrix {
            storage: result_vals,
            size: self.size,
        }
    }
}

impl Mul<Matrix> for Tuple {
    type Output = Tuple;

    fn mul(self, matrix: Matrix) -> Tuple {
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
