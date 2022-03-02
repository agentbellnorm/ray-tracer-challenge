use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn with(origin: Tuple, direction: Tuple) -> Ray {
        assert!(origin.is_point());
        assert!(direction.is_vector());

        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transformation: &Matrix) -> Ray {
        Ray {
            origin: self.origin * transformation,
            direction: self.direction * transformation,
        }
    }
}
