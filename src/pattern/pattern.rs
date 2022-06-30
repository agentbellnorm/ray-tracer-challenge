use crate::color::Color;
use crate::matrix::{is_equal_float, Matrix};
use crate::pattern::noise3;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use crate::{black, color, point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
    transformation: Matrix,
    inverse_transformation: Matrix,
    pattern_type: PatternType,
    noise: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PatternType {
    Striped,
    Gradient,
    Ring,
    Checkers,
    Test,
}

impl Pattern {
    pub fn color_at(self, point: Tuple) -> Color {
        match self.pattern_type {
            PatternType::Striped => match point.x.floor() as i64 % 2 {
                0 => self.a,
                _ => self.b,
            },
            PatternType::Gradient => {
                let distance = self.b - self.a;
                let fraction = point.x - point.x.floor();
                self.a + distance * fraction
            }
            PatternType::Ring => {
                match (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i64 % 2 {
                    0 => self.a,
                    _ => self.b,
                }
            }
            PatternType::Checkers => {
                match (point.x.floor() + point.y.floor() + point.z.floor()) as i64 % 2 {
                    0 => self.a,
                    _ => self.b,
                }
            }
            PatternType::Test => color(point.x, point.y, point.z),
        }
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self.inverse_transformation = transformation.inverse();
        self
    }

    pub fn with_noise(mut self, noise: f64) -> Self {
        self.noise = noise;
        self
    }

    pub fn has_noise(&self) -> bool {
        !is_equal_float(self.noise, 0.0)
    }

    pub fn color_at_object(self, object: &Shape, p: Tuple) -> Color {
        let object_space = p * &object.inverse_transformation;
        let mut pattern_space = object_space * &self.inverse_transformation;

        if self.has_noise() {
            let factor = self.noise * noise3(pattern_space.x, pattern_space.y, pattern_space.z);
            pattern_space = point(
                pattern_space.x + factor,
                pattern_space.y + factor,
                pattern_space.z + factor,
            );
        }

        self.color_at(pattern_space)
    }

    pub fn striped(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity().inverse(),
            pattern_type: PatternType::Striped,
            noise: 0.0,
        }
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity().inverse(),
            pattern_type: PatternType::Gradient,
            noise: 0.0,
        }
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity().inverse(),
            pattern_type: PatternType::Ring,
            noise: 0.0,
        }
    }

    pub fn checkers(a: Color, b: Color) -> Self {
        Pattern {
            a,
            b,
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity().inverse(),
            pattern_type: PatternType::Checkers,
            noise: 0.0,
        }
    }

    pub fn test() -> Self {
        Pattern {
            a: black(),
            b: black(),
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity().inverse(),
            pattern_type: PatternType::Test,
            noise: 0.0,
        }
    }
}
