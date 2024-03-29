use crate::color::{black, rgb, white, Color};
use crate::lights::PointLight;
use crate::pattern::Pattern;
use crate::shape::Shape;
use crate::tuple::Tuple;
use crate::World;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

impl Material {
    fn new() -> Material {
        Material {
            color: white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            reflective: 0.0,
            shininess: 200.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }

    pub fn from_color(color: Color) -> Material {
        Material {
            color,
            ..Material::default()
        }
    }

    pub fn from_pattern(pattern: Pattern) -> Material {
        Material {
            pattern: Some(pattern),
            ..Material::default()
        }
    }

    pub fn glass() -> Self {
        Material {
            color: white(),
            ambient: 0.0,
            diffuse: 0.0,
            transparency: 1.0,
            refractive_index: 1.5,
            shininess: 300.0,
            specular: 0.9,
            reflective: 1.0,
            pattern: None,
        }
    }

    pub fn chrome() -> Self {
        Material {
            color: white(),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 1.0,
            shininess: 400.0,
            reflective: 1.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }

    pub fn darker_chrome() -> Self {
        Material {
            color: rgb(169, 169, 169),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 1.0,
            shininess: 400.0,
            reflective: 0.3,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }

    pub fn air() -> Self {
        Material {
            color: white(),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.9,
            shininess: 300.0,
            reflective: 0.9,
            transparency: 0.9,
            refractive_index: 1.0000034,
            pattern: None,
        }
    }

    pub fn wrapper() -> Self {
        Material {
            color: rgb(255, 0, 0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 1.0,
            reflective: 0.0,
            transparency: 0.8,
            refractive_index: 1.0000034,
            pattern: None,
        }
    }

    pub fn pastel(color: Color) -> Material {
        Material {
            color,
            shininess: 1.0,
            specular: 0.0,
            ambient: 0.3,
            ..Material::default()
        }
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
        in_shadow: bool,
        world: &World,
    ) -> Color {
        let diffuse: Color;
        let specular: Color;

        let black = black();
        let color = match self.pattern {
            Some(pattern) => pattern.color_at_object(world, object, point),
            None => self.color,
        };

        // combine surface color with lights color/intensity
        let effective_color = color * light.intensity;

        // direction of light source
        let light_vector = (light.position - point).normalize();

        // ambient contribution
        let ambient = effective_color * self.ambient;

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
