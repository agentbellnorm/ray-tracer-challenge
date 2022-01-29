use crate::tuple::EPSILON;

#[derive(Debug, Clone)]
pub struct Matrix {
    storage: Vec<f32>,
    n_cols: usize,
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
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.storage[self.i(row, col)]
    }

    fn i(&self, row: usize, col: usize) -> usize {
        (self.n_cols * row) + col
    }
}

fn is_equal_float(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
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
