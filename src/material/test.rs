#[cfg(test)]
mod material_test {
    use crate::color::{black, color, white};
    use crate::lights::PointLight;
    use crate::material::Material;
    use crate::pattern::Pattern;
    use crate::tuple::{point, vector, Tuple};
    use crate::Shape;

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.color, white());
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
        let light = PointLight::with(point(0.0, 0.0, -10.0), white());

        let result = m.lighting(
            &Shape::sphere_default(),
            &light,
            position,
            eye_v,
            normal_v,
            false,
        );

        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let (m, position) = setup();
        let eye_v = vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, -10.0), white());

        let result = m.lighting(
            &Shape::sphere_default(),
            &light,
            position,
            eye_v,
            normal_v,
            false,
        );

        assert_eq!(result, white());
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let (m, position) = setup();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 10.0, -10.0), white());

        let result = m.lighting(
            &Shape::sphere_default(),
            &light,
            position,
            eye_v,
            normal_v,
            false,
        );

        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let (m, position) = setup();
        let eye_v = vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 10.0, -10.0), white());

        let result = m.lighting(
            &Shape::sphere_default(),
            &light,
            position,
            eye_v,
            normal_v,
            false,
        );

        assert_eq!(result, color(1.63639, 1.63639, 1.63639));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = setup();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, 10.0), white());

        let result = m.lighting(
            &Shape::sphere_default(),
            &light,
            position,
            eye_v,
            normal_v,
            false,
        );

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighing_with_surface_in_shadows() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, -10.0), white());
        let in_shadow = true;

        let result = m.lighting(
            &Shape::sphere_default(),
            &light,
            position,
            eyev,
            normalv,
            in_shadow,
        );

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let pattern = Pattern::striped(white(), black());
        let mut material = Material::from_pattern(pattern);
        material.ambient = 1.0;
        material.specular = 0.0;
        material.diffuse = 0.0;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::with(point(0.0, 0.0, -10.0), white());

        let c1 = material.lighting(
            &Shape::sphere_default(),
            &light,
            point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = material.lighting(
            &Shape::sphere_default(),
            &light,
            point(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );

        assert_eq!(c1, white());
        assert_eq!(c2, black());
    }

    #[test]
    fn reflectivity_for_default_material() {
        assert_eq!(Material::new().reflective, 0.0)
    }
}
