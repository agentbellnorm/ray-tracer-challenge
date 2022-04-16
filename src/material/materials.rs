use crate::color::{black, white, Color};
use crate::lights::PointLight;
use crate::pattern::Pattern;
use crate::shapes::Shape;
use crate::tuple::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn with_color(color: Color) -> Material {
        let mut m = Material::new();
        m.color = color;
        m
    }

    pub fn from_pattern(pattern: Pattern) -> Material {
        let mut m = Material::new();
        m.pattern = Some(pattern);
        m
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Material {
        self.pattern = Some(pattern);
        self
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
        in_shadow: bool,
    ) -> Color {
        let ambient: Color;
        let diffuse: Color;
        let specular: Color;

        let black = black();
        let color = match self.pattern {
            Some(pattern) => pattern.color_at_object(object, point),
            None => self.color,
        };

        // combine surface color with lights color/intensity
        let effective_color = color * light.intensity;

        // direction of light source
        let light_vector = (light.position - point).normalize();

        // ambient contribution
        ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        // light_dot_normal is cosine of angle between light vector and normal vector.
        let light_dot_normal = light_vector.dot(&normal_vector);

        if light_dot_normal < 0.0 {
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