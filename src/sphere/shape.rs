use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::sphere;
use crate::tuple::Tuple;

#[derive(PartialEq, Clone, Debug)]
pub enum Shape {
    Sphere {
        transformation: Matrix,
        material: Material,
    },
}

impl Shape {
    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        match self {
            Shape::Sphere { transformation, .. } => sphere::normal_at(transformation, world_point),
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.get_transformation().inverse());

        let v = match self {
            Shape::Sphere { .. } => sphere::intersects(&transformed_ray),
        };

        Intersections {
            xs: v.into_iter().map(|t| Intersection::new(t, self)).collect(),
        }
    }

    pub fn get_transformation(&self) -> &Matrix {
        match self {
            Shape::Sphere { transformation, .. } => transformation,
        }
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Shape::Sphere { material, .. } => material,
        }
    }

    pub fn with_transform(self, transformation: Matrix) -> Self {
        match self {
            Shape::Sphere { material, .. } => Shape::Sphere {
                transformation,
                material,
            },
        }
    }

    pub fn with_material(self, material: Material) -> Self {
        match self {
            Shape::Sphere { transformation, .. } => Shape::Sphere {
                transformation,
                material,
            },
        }
    }
}
