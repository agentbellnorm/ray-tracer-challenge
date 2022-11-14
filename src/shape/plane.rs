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

#[cfg(test)]
mod plane_test {
    use crate::rays::Ray;
    use crate::shape::plane::plane_normal_at;
    use crate::tuple::{point, vector};
    use crate::Shape;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let n1 = plane_normal_at(point(0.0, 0.0, 0.0));
        let n2 = plane_normal_at(point(10.0, 0.0, -10.0));
        let n3 = plane_normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Shape::plane_default();
        let r = Ray::with(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = Shape::intersects(p.pack(), &r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Shape::plane_default();
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = Shape::intersects(p.pack(), &r);

        assert!(xs.is_empty())
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Shape::plane_default();
        let r = Ray::with(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = Shape::intersects(p.pack(), &r);

        assert_eq!(xs.len(), 1);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Shape::plane_default();
        let r = Ray::with(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = Shape::intersects(p.pack(), &r);

        assert_eq!(xs.len(), 1);
    }
}
