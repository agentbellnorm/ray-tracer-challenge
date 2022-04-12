use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern {
    Striped { a: Color, b: Color },
}

impl Pattern {
    pub fn color_at(self, point: Tuple) -> Color {
        match self {
            Pattern::Striped { a, b } => match point.x.floor() as i64 % 2 {
                0 => a,
                _ => b,
            },
        }
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern::Striped { a, b }
}
