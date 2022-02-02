use crate::tuple::{Tuple, EPSILON};
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    storage: Vec<f32>,
    n_cols: usize,
    n_rows: usize,
}

impl Matrix {
    pub fn from_values(from: Vec<Vec<f32>>) -> Matrix {
        let n_rows = from.len();
        let n_cols = from.first().unwrap().len();

        Matrix {
            storage: from
                .into_iter()
                .fold(Vec::with_capacity(n_rows * n_cols), |acc, v| {
                    [acc, v].concat()
                }),
            n_cols,
            n_rows,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.storage[self.i(row, col)]
    }

    fn i(&self, row: usize, col: usize) -> usize {
        (self.n_cols * row) + col
    }

    pub fn identity() -> Matrix {
        Matrix {
            n_rows: 4,
            n_cols: 4,
            storage: vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut transposed = Vec::with_capacity(self.n_rows * self.n_cols);

        for i in 0..self.n_cols {
            for j in 0..self.n_rows {
                transposed.push(self.storage[i + j * self.n_rows]);
            }
        }

        Matrix {
            n_rows: self.n_rows,
            n_cols: self.n_cols,
            storage: transposed,
        }
    }

    pub fn determinant(&self) -> f32 {
        assert_eq!(self.n_rows, 2);
        assert_eq!(self.n_cols, 2);

        self.get(0, 0) * self.get(1, 1) - self.get(1, 0) * self.get(0, 1)
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut submatrix = Vec::with_capacity((self.n_rows - 1) * (self.n_cols - 1));

        for r in 0..self.n_rows {
            for c in 0..self.n_cols {
                if r == row || c == col {
                    continue;
                }

                submatrix.push(self.get(r, c));
            }
        }

        Matrix {
            n_rows: self.n_rows - 1,
            n_cols: self.n_cols - 1,
            storage: submatrix,
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        assert_eq!(self.n_cols, 3);
        assert_eq!(self.n_rows, 3);

        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            return self.minor(row, col);
        }

        -self.minor(row, col)
    }
}

fn is_equal_float(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.storage.len() != other.storage.len()
            || self.n_cols != other.n_cols
            || self.n_rows != other.n_rows
        {
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

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.n_cols, 4);
        assert_eq!(self.n_rows, 4);
        assert_eq!(rhs.n_rows, 4);
        assert_eq!(rhs.n_rows, 4);

        let mut result_vals = Vec::with_capacity(self.n_cols * self.n_rows);

        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
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
            n_rows: self.n_rows,
            n_cols: self.n_cols,
        }
    }
}

impl Mul<Matrix> for Tuple {
    type Output = Tuple;

    fn mul(self, matrix: Matrix) -> Tuple {
        let mut result_vals = Vec::with_capacity(4);

        for row in 0..matrix.n_rows {
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
