use crate::color::Color;
use crate::matrix::Matrix;
use crate::shapes::Shape;
use crate::tuple::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    Striped {
        a: Color,
        b: Color,
        transformation: Matrix,
    },
}

impl Pattern {
    pub fn color_at(self, point: Tuple) -> Color {
        match self {
            Pattern::Striped { a, b, .. } => match point.x.floor() as i64 % 2 {
                0 => a,
                _ => b,
            },
        }
    }

    pub fn with_transformation(self, transformation: Matrix) -> Self {
        match self {
            Pattern::Striped { a, b, .. } => Pattern::Striped {
                a,
                b,
                transformation,
            },
        }
    }

    pub fn color_at_object(self, object: &Shape, point: Tuple) -> Color {
        let object_space = point * &object.get_transformation().inverse();
        let pattern_space = object_space * &self.get_transformation().inverse();
        self.color_at(pattern_space)
    }

    pub fn get_transformation(&self) -> &Matrix {
        match self {
            Pattern::Striped { transformation, .. } => transformation,
        }
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern::Striped {
        a,
        b,
        transformation: Matrix::identity(),
    }
}
