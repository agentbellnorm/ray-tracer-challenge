use crate::material::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::shapes::shape::Shape;
use crate::tuple::{point, Tuple};

pub fn sphere_normal_at(object_point: Tuple) -> Tuple {
    object_point - point(0.0, 0.0, 0.0)
}

pub fn sphere_intersects(transformed_ray: &Ray) -> Vec<f64> {
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
