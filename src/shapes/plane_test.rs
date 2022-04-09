#[cfg(test)]
mod plane_test {
    use crate::rays::Ray;
    use crate::shapes::plane::{plane_default, plane_normal_at};
    use crate::tuple::{point, vector};

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
        let p = plane_default();
        let r = Ray::with(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersects(&r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = plane_default();
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersects(&r);

        assert!(xs.is_empty())
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = plane_default();
        let r = Ray::with(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersects(&r);

        assert_eq!(xs.len(), 1);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = plane_default();
        let r = Ray::with(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersects(&r);

        assert_eq!(xs.len(), 1);
    }
}