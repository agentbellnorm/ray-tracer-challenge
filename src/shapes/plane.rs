use crate::rays::Ray;
use crate::tuple::{vector, Tuple, EPSILON};

pub fn plane_intersects(ray: &Ray) -> Vec<f64> {
    if f64::abs(ray.direction.y) < EPSILON {
        return vec![];
    }

    vec![-ray.origin.y / ray.direction.y]
}

pub fn plane_normal_at(_object_point: Tuple) -> Tuple {
    vector(0.0, 1.0, 0.0)
}

// pub fn plane_from_transform(transformation: Matrix) -> Shape {
//     Shape::Plane {
//         transformation,
//         material: Material::new(),
//     }
// }
