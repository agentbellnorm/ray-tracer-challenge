use crate::matrix::is_equal_float;
use crate::rays::Ray;
use crate::tuple::{vector_i, Tuple};

pub fn cylinder_intersects(ray: &Ray) -> Vec<f64> {
    let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

    if is_equal_float(a, 0.0) {
        return vec![];
    }

    let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
    let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

    let disc = b.powi(2) - 4.0 * a * c;

    if disc < 0.0 {
        return vec![];
    }

    vec![1.0]
}

pub fn cylinder_normal_at(point: Tuple) -> Tuple {
    vector_i(1, 1, 1)
}

#[cfg(test)]
mod cylinder_test {
    use crate::rays::Ray;
    use crate::tuple::{point_i, vector_i, Tuple};
    use crate::Shape;
    use parameterized::parameterized;

    #[parameterized(
    origin = {      point_i(1, 0, 0),   point_i(0, 0, 0),   point_i(0, 0, -5)   },
    direction = {   vector_i(0, 1, 0),  vector_i(0, 1, 0),  vector_i(1, 1, 1)   }
    )]
    fn ray_misses_cylinder(origin: Tuple, direction: Tuple) {
        let cylinder = Shape::cylinder_default();
        let ray = Ray::with(origin, direction.normalize());

        assert_eq!(cylinder.intersects(&ray).len(), 0)
    }
}
