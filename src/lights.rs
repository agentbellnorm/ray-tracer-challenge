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

#[test]
fn position_and_intensity() {
    use crate::{point, white};

    let position = point(0.0, 0.0, 0.0);
    let intensity = white();
    let point_light = PointLight::with(position, intensity);

    assert_eq!(point_light.position, position);
    assert_eq!(point_light.intensity, intensity);
}
