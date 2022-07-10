use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::shapes;
use crate::shapes::plane::{plane_intersects, plane_normal_at};
use crate::shapes::sphere_normal_at;
use crate::tuple::Tuple;

#[derive(PartialEq, Clone, Debug)]
pub enum ShapeType {
    Sphere,
    Plane,
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
            material: Material::new(),
            inverse_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn plane_default() -> Self {
        Shape::default(ShapeType::Plane)
    }

    pub fn sphere_default() -> Self {
        Shape::default(ShapeType::Sphere)
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
        };

        let mut world_normal = object_normal * &self.inverse_transformation.transpose();
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn intersects(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.inverse_transformation);

        let v = match self.shape_type {
            ShapeType::Sphere => shapes::sphere_intersects(&transformed_ray),
            ShapeType::Plane => plane_intersects(&transformed_ray),
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
