use crate::color::{color, Color};
use crate::intersection::{Intersection, Intersections, PreparedComputation};
use crate::lights::PointLight;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::sphere::Sphere;
use crate::tuple::{point, Tuple};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: PointLight,
}

impl World {
    pub fn with(objects: Vec<Sphere>, light_source: PointLight) -> World {
        World {
            objects,
            light_source,
        }
    }

    pub fn default_world() -> World {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), Color::white());

        let mut material = Material::with_color(color(0.8, 1.0, 0.6));
        material.diffuse = 0.7;
        material.specular = 0.2;

        let s1 = Sphere::with_material(material);
        let s2 = Sphere::unit().set_transform(Matrix::identity().scale(0.5, 0.5, 0.5));

        Self::with(vec![s1, s2], light)
    }

    pub fn intersect_world(&self, ray: &Ray) -> Intersections {
        let mut xs: Vec<Intersection> = Vec::new();

        for i in 0..self.objects.len() {
            xs.append(&mut self.objects[i].intersects(ray).xs);
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections { xs }
    }

    pub fn shade_hit(&self, computations: PreparedComputation) -> Color {
        computations.object.material.lighting(
            &self.light_source,
            computations.over_point,
            computations.eye_vector,
            computations.normal_vector,
            self.is_shadowed(computations.over_point),
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersection = self.intersect_world(ray).xs.into_iter().find(|i| i.t > 0.0);

        match intersection {
            Some(i) => self.shade_hit(i.prepare_computations(ray)),
            None => Color::black(),
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light_source.position - point;
        let direction = v.normalize();
        let distance = v.magnitude();

        let shadow_ray = Ray::with(point, direction);

        match self
            .intersect_world(&shadow_ray)
            .xs
            .into_iter()
            .find(|i| i.t > 0.0)
        {
            Some(hit) => hit.t < distance,
            None => false,
        }
    }
}
