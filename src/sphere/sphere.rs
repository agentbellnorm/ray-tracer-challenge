use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::sphere::shape::Shape;
use crate::sphere::ShapeInit;
use crate::tuple::{point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub transformation: Matrix,
    pub material: Material,
}

impl ShapeInit for Sphere {
    fn new() -> Self {
        Sphere {
            material: Material::new(),
            transformation: Matrix::identity(),
        }
    }

    fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    fn from_material(material: Material) -> Self {
        Sphere {
            transformation: Matrix::identity(),
            material,
        }
    }

    fn with_transform(mut self, transform: Matrix) -> Self {
        self.transformation = transform;
        self
    }

    fn from_transform(transformation: Matrix) -> Self {
        Sphere {
            transformation,
            material: Material::new(),
        }
    }
}

impl Shape for Sphere {
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let object_point = world_point * &self.transformation.inverse();
        let object_normal = object_point - point(0.0, 0.0, 0.0);

        let mut world_normal = object_normal * &self.transformation.inverse().transpose();
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    fn intersects(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.transformation.inverse());

        let sphere_to_ray = transformed_ray.origin - point(0.0, 0.0, 0.0);

        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = transformed_ray.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections { xs: Vec::new() };
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Intersections {
            xs: vec![Intersection::new(t1, self), Intersection::new(t2, self)],
        }
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}
