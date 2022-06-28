use crate::color::{black, color, white, Color};
use crate::intersection::{Intersection, Intersections, PreparedComputation};
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrix::{is_equal_float, Matrix};
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tuple::{point, Tuple};
use std::vec;

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

        let reflected = self.reflected_color(computations, remaining);
        let refracted = self.refracted_color(computations, remaining);

        let material = computations.object.material;

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = computations.schlick();

            return surface_color + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface_color + reflected + refracted
    }

    pub fn color_at(&self, ray: &Ray, remaining: i32) -> Color {
        let intersections = self.intersect_world(ray).xs;
        let positive_intersection = intersections.iter().find(|i| i.t > 0.0);

        match positive_intersection {
            Some(intersection) => self.shade_hit(
                &intersection
                    .prepare_computations(ray, &Intersections::from(intersections.clone())),
                remaining,
            ),
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

    pub fn refracted_color(&self, comps: &PreparedComputation, remaining: i32) -> Color {
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(&comps.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        let is_total_internal_reflection = sin2_t > 1.0;

        if comps.is_opaque() || remaining == 0 || is_total_internal_reflection {
            return black();
        }

        // cos(theta_t) via trig identity
        let cos_t = f64::sqrt(1.0 - sin2_t);

        // direction of refracted ray
        let direction =
            comps.normal_vector * (n_ratio * cos_i - cos_t) - comps.eye_vector * n_ratio;

        let refract_ray = Ray::with(comps.under_point, direction);

        self.color_at(&refract_ray, remaining - 1) * comps.object.material.transparency
    }

    pub fn has_object(&self, o: &Shape) -> bool {
        self.objects.contains(o)
    }

    pub fn add_object(mut self, o: Shape) -> Self {
        self.objects.push(o);
        self
    }
}
