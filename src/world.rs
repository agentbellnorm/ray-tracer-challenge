use crate::color::{black, color, white, Color};
use crate::intersection::{Intersection, Intersections, PreparedComputation};
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrix::{is_equal_float, Matrix};
use crate::rays::Ray;
use crate::shape::{Shape, ShapeType};
use crate::tuple::{point, Tuple};
use std::borrow::BorrowMut;
use std::f64::consts::FRAC_PI_2;
use std::vec;

pub type ShapeId = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct WorldShape {
    pub shape: Shape,
    id: ShapeId,
}

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<WorldShape>,
    pub light_source: PointLight,
}

impl World {
    pub fn default() -> World {
        Self::with_light(PointLight::default())
    }

    pub fn with_light(light_source: PointLight) -> World {
        World {
            objects: vec![],
            light_source,
        }
    }

    pub fn with_objects(mut self, objects: Vec<Shape>) -> World {
        for obj in objects {
            self.add_shape(obj);
        }

        self
    }

    pub fn add_shape_to_group(&mut self, group_id: ShapeId, shape_id: ShapeId) -> usize {
        let mut shape = self.objects.get_mut(shape_id).unwrap();
        shape.shape.parent = Some(group_id);

        let mut group_members = match &self.get_shape(group_id).shape_type {
            ShapeType::Group(children) => children.clone(),
            _ => panic!("group id did not belong to a group"),
        };
        group_members.push(shape_id);

        self.objects.get_mut(group_id).unwrap().shape.shape_type = ShapeType::Group(group_members);

        shape_id
    }

    pub fn add_shape(&mut self, mut shape: Shape) -> usize {
        let shape_id = self.next_index();
        shape.id = Some(shape_id);
        let world_shape = WorldShape {
            shape,
            id: shape_id,
        };
        self.objects.push(world_shape);

        shape_id
    }

    pub fn test_world() -> World {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), white());

        let mut material = Material::from_color(color(0.8, 1.0, 0.6));
        material.diffuse = 0.7;
        material.specular = 0.2;

        let s1 = Shape::sphere_from_material(material);
        let s2 = Shape::sphere_from_transform(Matrix::identity().scale(0.5, 0.5, 0.5));

        Self::with_light(light).with_objects(vec![s1, s2])
    }

    pub fn test_world_with_group() -> World {
        let mut world = World::default();
        let g1 =
            world.add_shape(Shape::group().with_transform(Matrix::identity().rotate_y(FRAC_PI_2)));
        let g2 = world
            .add_shape(Shape::group().with_transform(Matrix::identity().scale(2.0, 2.0, 2.0)));
        let sphere = world.add_shape(
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0)),
        );

        world.add_shape_to_group(g1, g2);
        world.add_shape_to_group(g2, sphere);

        world
    }

    pub fn intersect_world(&self, ray: &Ray) -> Intersections {
        let mut xs: Vec<Intersection> = Vec::new();

        for world_shape in &self.objects {
            // shapes in groups are computed as part of the group, not by themselves
            if world_shape.shape.is_in_group() {
                continue;
            }

            xs.append(&mut world_shape.shape.intersects(self, ray).xs);
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections { xs }
    }

    pub fn shade_hit(&self, computations: &PreparedComputation, remaining: i32) -> Color {
        let is_in_shadow = self.is_shadowed(computations.over_point);
        let shape = self.get_shape(computations.object);
        let surface_color = shape.material.lighting(
            shape,
            &self.light_source,
            computations.over_point,
            computations.eye_vector,
            computations.normal_vector,
            is_in_shadow,
            self,
        );

        let reflected = self.reflected_color(computations, remaining);
        let refracted = self.refracted_color(computations, remaining);

        let material = shape.material;

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
                &intersection.prepare_computations(
                    self,
                    ray,
                    &Intersections::from(intersections.clone()),
                ),
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
        let shape = self.get_shape(comps.object);
        if is_equal_float(shape.material.reflective, 0.0) || remaining <= 0 {
            return black();
        }

        let reflect_ray = Ray::with(comps.over_point, comps.reflection_vector);

        self.color_at(&reflect_ray, remaining - 1) * shape.material.reflective
    }

    pub fn refracted_color(&self, comps: &PreparedComputation, remaining: i32) -> Color {
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(&comps.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        let shape = self.get_shape(comps.object);

        let is_total_internal_reflection = sin2_t > 1.0;

        if shape.is_opaque() || remaining == 0 || is_total_internal_reflection {
            return black();
        }

        // cos(theta_t) via trig identity
        let cos_t = f64::sqrt(1.0 - sin2_t);

        // direction of refracted ray
        let direction =
            comps.normal_vector * (n_ratio * cos_i - cos_t) - comps.eye_vector * n_ratio;

        let refract_ray = Ray::with(comps.under_point, direction);

        self.color_at(&refract_ray, remaining - 1) * shape.material.transparency
    }

    pub fn has_object(&self, object_id: ShapeId) -> bool {
        self.objects
            .iter()
            .any(|item| item.shape.id.unwrap().eq(&object_id))
    }

    pub fn current_index(&self) -> usize {
        self.objects.len() - 1
    }

    pub fn next_index(&self) -> usize {
        self.objects.len()
    }

    pub fn get_shape(&self, id: ShapeId) -> &Shape {
        &self.objects.get(id).unwrap().shape
    }
}
