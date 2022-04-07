use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::sphere::shape::Shape;
use crate::tuple::{point, Tuple};

pub fn normal_at(transformation: &Matrix, world_point: Tuple) -> Tuple {
    assert!(world_point.is_point());

    let object_point = world_point * &transformation.inverse();
    let object_normal = object_point - point(0.0, 0.0, 0.0);

    let mut world_normal = object_normal * &transformation.inverse().transpose();
    world_normal.w = 0.0;

    world_normal.normalize()
}

pub fn intersects(transformed_ray: &Ray) -> Vec<f64> {
    let sphere_to_ray = transformed_ray.origin - point(0.0, 0.0, 0.0);

    let a = transformed_ray.direction.dot(&transformed_ray.direction);
    let b = transformed_ray.direction.dot(&sphere_to_ray) * 2.0;
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return Vec::new();
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    vec![t1, t2]
}

pub fn sphere_default() -> Shape {
    Shape::Sphere {
        material: Material::new(),
        transformation: Matrix::identity(),
    }
}

pub fn sphere_from_material(material: Material) -> Shape {
    Shape::Sphere {
        material,
        transformation: Matrix::identity(),
    }
}

pub fn sphere_from_transform(transformation: Matrix) -> Shape {
    Shape::Sphere {
        transformation,
        material: Material::new(),
    }
}
