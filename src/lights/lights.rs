use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn with(position: Tuple, intensity: Color) -> PointLight {
        assert!(position.is_point());
        PointLight {
            position,
            intensity,
        }
    }
}
