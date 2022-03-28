use crate::color::{color, Color};
use crate::lights::PointLight;
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn with_color(color: Color) -> Material {
        let mut m = Material::new();
        m.color = color;
        m
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
        in_shadow: bool,
    ) -> Color {
        let ambient: Color;
        let diffuse: Color;
        let specular: Color;

        let black = Color::black();

        // combine surface color with lights color/intensity
        let effective_color = self.color * light.intensity;

        // direction of light source
        let light_vector = (light.position - point).normalize();

        // ambient contribution
        ambient = effective_color * self.ambient;

        // light_dot_normal is cosine of angle between light vector and normal vector.
        let light_dot_normal = light_vector.dot(&normal_vector);

        if light_dot_normal < 0.0 || in_shadow {
            // light on other side of surface, or in shadow of other object.
            diffuse = black;
            specular = black;
        } else {
            diffuse = effective_color * (self.diffuse * light_dot_normal);

            // cosine of angle between reflection vector and eye_vector
            let reflect_vector = (-light_vector).reflect(&normal_vector);
            let reflect_dot_eye = reflect_vector.dot(&eye_vector);

            if reflect_dot_eye <= 0.0 {
                // light reflects away from the eye
                specular = black;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}
