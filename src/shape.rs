pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::shape::cone::{cone_intersects, cone_normal_at};
use crate::shape::cube::{cube_intersects, cube_normal_at};
use crate::shape::cylinder::{cylinder_intersects, cylinder_normal_at};
use crate::shape::plane::{plane_intersects, plane_normal_at};
use crate::shape::sphere::{sphere_intersects, sphere_normal_at};
use crate::tuple::Tuple;

#[derive(PartialEq, Clone, Debug)]
pub enum ShapeType {
    Sphere,
    Plane,
    Cube,
    Cylinder(f64, f64, bool), // Cylinder(min_y, max_y, closed)
    Cone(f64, f64, bool),     // Cone(min_y, max_y, closed)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Shape {
    pub inverse_transformation: Matrix,
    pub material: Material,
    shape_type: ShapeType,
}

impl Shape {
    fn default(shape_type: ShapeType) -> Self {
        Shape {
            shape_type,
            material: Material::default(),
            inverse_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn plane_default() -> Self {
        Shape::default(ShapeType::Plane)
    }

    pub fn sphere_default() -> Self {
        Shape::default(ShapeType::Sphere)
    }

    pub fn cube_default() -> Self {
        Shape::default(ShapeType::Cube)
    }

    pub fn cylinder_default() -> Self {
        Shape::default(ShapeType::Cylinder(-f64::INFINITY, f64::INFINITY, false))
    }

    pub fn cylinder(min: f64, max: f64, closed: bool) -> Self {
        Shape::default(ShapeType::Cylinder(min, max, closed))
    }
    
    pub fn cone_default() -> Self {
        Shape::default(ShapeType::Cone(-f64::INFINITY, f64::INFINITY, false))
    }
    
    pub fn cone(y_min: f64, y_max: f64, closed: bool) -> Self {
        Shape::default(ShapeType::Cone(y_min, y_max, closed))
    }

    pub fn sphere_from_material(material: Material) -> Self {
        Shape::sphere_default().with_material(material)
    }

    pub fn sphere_from_transform(transform: Matrix) -> Self {
        Shape::sphere_default().with_transform(transform)
    }

    pub fn plane_from_material(material: Material) -> Self {
        Shape::plane_default().with_material(material)
    }

    pub fn sphere_glass() -> Self {
        Shape::sphere_from_material(Material::glass())
    }

    pub fn sphere_chrome() -> Self {
        Shape::sphere_from_material(Material::chrome())
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let object_point = world_point * &self.inverse_transformation;

        let object_normal = match self.shape_type {
            ShapeType::Sphere => sphere_normal_at(object_point),
            ShapeType::Plane => plane_normal_at(object_point),
            ShapeType::Cube => cube_normal_at(object_point),
            ShapeType::Cylinder(y_min, y_max, _) => cylinder_normal_at(object_point, y_min, y_max),
            ShapeType::Cone(y_min, y_max, _) => cone_normal_at(object_point, y_min, y_max),
        };

        let mut world_normal = object_normal * &self.inverse_transformation.transpose();
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn intersects(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.inverse_transformation);

        let v = match self.shape_type {
            ShapeType::Sphere => sphere_intersects(&transformed_ray),
            ShapeType::Plane => plane_intersects(&transformed_ray),
            ShapeType::Cube => cube_intersects(&transformed_ray),
            ShapeType::Cylinder(y_min, y_max, closed) => {
                cylinder_intersects(&transformed_ray, y_min, y_max, closed)
            }
            ShapeType::Cone(y_min, y_max, closed) => {
                cone_intersects(&transformed_ray, y_min, y_max, closed)
            }
        };

        Intersections {
            xs: v.into_iter().map(|t| Intersection::new(t, self)).collect(),
        }
    }

    pub fn with_transform(mut self, transformation: Matrix) -> Self {
        self.inverse_transformation = transformation.inverse();
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}
