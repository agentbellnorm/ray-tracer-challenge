use crate::color::{black, color, white, Color};
use crate::intersection::{Intersection, Intersections, PreparedComputation};
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrix::{is_equal_float, Matrix};
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tuple::{point, Tuple};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Shape>,
    pub light_source: PointLight,
}

impl World {
    pub fn with(objects: Vec<Shape>, light_source: PointLight) -> World {
        World {
            objects,
            light_source,
        }
    }

    pub fn default_world() -> World {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), white());

        let mut material = Material::from_color(color(0.8, 1.0, 0.6));
        material.diffuse = 0.7;
        material.specular = 0.2;

        let s1 = Shape::sphere_from_material(material);
        let s2 = Shape::sphere_from_transform(Matrix::identity().scale(0.5, 0.5, 0.5));

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

    pub fn shade_hit(&self, computations: &PreparedComputation, remaining: i32) -> Color {
        let is_in_shadow = self.is_shadowed(computations.over_point);
        let surface_color = computations.object.material.lighting(
            computations.object,
            &self.light_source,
            computations.over_point,
            computations.eye_vector,
            computations.normal_vector,
            is_in_shadow,
        );
        let reflected_color = self.reflected_color(computations, remaining);

        surface_color + reflected_color
    }

    pub fn color_at(&self, ray: &Ray, remaining: i32) -> Color {
        let intersection = self.intersect_world(ray).xs.into_iter().find(|i| i.t > 0.0);

        match intersection {
            Some(i) => self.shade_hit(&i.prepare_computations(ray), remaining),
            None => black(),
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

    pub fn reflected_color(&self, comps: &PreparedComputation, remaining: i32) -> Color {
        if is_equal_float(comps.object.material.reflective, 0.0) || remaining <= 0 {
            return black();
        }

        let reflect_ray = Ray::with(comps.over_point, comps.reflection_vector);

        self.color_at(&reflect_ray, remaining - 1) * comps.object.material.reflective
    }

    pub fn has_object(&self, o: &Shape) -> bool {
        self.objects.contains(o)
    }

    pub fn add_object(mut self, o: Shape) -> Self {
        self.objects.push(o);
        self
    }
}
