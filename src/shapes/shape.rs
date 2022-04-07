use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::shapes;
use crate::shapes::sphere_normal_at;
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
        assert!(world_point.is_point());

        let transformation = self.get_transformation();
        let object_point = world_point * &transformation.inverse();

        let object_normal = match self {
            Shape::Sphere { .. } => sphere_normal_at(object_point),
        };

        let mut world_normal = object_normal * &transformation.inverse().transpose();
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn intersects(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.get_transformation().inverse());

        let v = match self {
            Shape::Sphere { .. } => shapes::intersects(&transformed_ray),
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
