use crate::intersection::Intersections;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::Tuple;
use std::fmt::Debug;

pub trait Shape: Debug {
    fn normal_at(&self, point: Tuple) -> Tuple;
    fn intersects(&self, ray: &Ray) -> Intersections;
    fn get_transformation(&self) -> &Matrix;
    fn get_material(&self) -> &Material;
}

// should just be needed for tests
impl PartialEq for dyn Shape {
    fn eq(&self, that: &dyn Shape) -> bool {
        self.get_material().eq(that.get_material())
            && self.get_transformation().eq(that.get_transformation())
    }
}

pub trait ShapeInit: PartialEq {
    fn new() -> Self;
    fn with_material(self, material: Material) -> Self;
    fn from_material(material: Material) -> Self;
    fn with_transform(self, transform: Matrix) -> Self;
    fn from_transform(transform: Matrix) -> Self;
}
