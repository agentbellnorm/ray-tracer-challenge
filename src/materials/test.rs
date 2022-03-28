#[cfg(test)]
mod material_test {
    use crate::color::{color, Color};
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::tuple::{point, vector, Tuple};

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.color, color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    fn setup() -> (Material, Tuple) {
        (Material::new(), point(0.0, 0.0, 0.0))
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = setup();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eye_v, normal_v, false);

        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let (m, position) = setup();
        let eye_v = vector(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eye_v, normal_v, false);

        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let (m, position) = setup();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eye_v, normal_v, false);

        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let (m, position) = setup();
        let eye_v = vector(0.0, -f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eye_v, normal_v, false);

        assert_eq!(result, color(1.6363853, 1.6363853, 1.6363853));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = setup();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, 10.0), Color::white());

        let result = m.lighting(&light, position, eye_v, normal_v, false);

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighing_with_surface_in_shadows() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, -10.0), Color::white());
        let in_shadow = true;

        let result = m.lighting(&light, position, eyev, normalv, in_shadow);

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }
}
