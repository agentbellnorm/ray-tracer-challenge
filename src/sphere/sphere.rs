use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::{point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    location: Tuple,
    r: f32,
    pub transformation: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn unit() -> Sphere {
        Sphere {
            location: point(0.0, 0.0, 0.0),
            r: 1.0,
            transformation: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn intersects(&self, ray: Ray) -> Intersections {
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

    pub fn set_transform(mut self, transform: Matrix) -> Sphere {
        self.transformation = transform;
        self
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let object_point = world_point * &self.transformation.inverse();
        let object_normal = object_point - point(0.0, 0.0, 0.0);

        let mut world_normal = object_normal * &self.transformation.inverse().transpose();
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}
