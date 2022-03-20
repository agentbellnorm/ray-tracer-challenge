#[cfg(test)]
mod lights_test {
    use crate::color::color;
    use crate::lights::PointLight;
    use crate::tuple::point;

    #[test]
    fn position_and_intensity() {
        let position = point(0.0, 0.0, 0.0);
        let intensity = color(1.0, 1.0, 1.0);
        let point_light = PointLight::with(position, intensity);

        assert_eq!(point_light.position, position);
        assert_eq!(point_light.intensity, intensity);
    }
}
