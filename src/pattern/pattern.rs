use crate::color::{white, Color};
use crate::matrix::Matrix;
use crate::shapes::Shape;
use crate::tuple::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
    transformation: Matrix,
    pattern_type: PatternType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PatternType {
    Striped,
    Gradient,
    Ring,
    Checkers,
}

impl Pattern {
    pub fn color_at(self, point: Tuple) -> Color {
        match self.pattern_type {
            PatternType::Striped => match point.x.floor() as i64 % 2 {
                0 => self.a,
                _ => self.b,
            },
            PatternType::Gradient => self.a + (self.b - self.a) * (point.x - point.x.floor()),
            PatternType::Ring => {
                match (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i64 % 2 {
                    0 => self.a,
                    _ => self.b,
                }
            }
            PatternType::Checkers => {
                match (point.x.abs() + point.y.abs() + point.z.abs()) as i64 % 2 {
                    0 => self.a,
                    _ => self.b,
                }
            }
        }
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self
    }

    pub fn color_at_object(self, object: &Shape, point: Tuple) -> Color {
        let object_space = point * &object.get_transformation().inverse();
        let pattern_space = object_space * &self.transformation.inverse();
        self.color_at(pattern_space)
    }

    pub fn striped(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            pattern_type: PatternType::Striped,
        }
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            pattern_type: PatternType::Gradient,
        }
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            pattern_type: PatternType::Ring,
        }
    }

    pub fn checkers(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            pattern_type: PatternType::Checkers,
        }
    }
}
