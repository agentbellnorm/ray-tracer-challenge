#[cfg(test)]
mod template_test {
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::tuple::{point, vector};
    use crate::Shape;

    #[test]
    fn create_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let ray = Ray::with(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let ray = Ray::with(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

        assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Shape::sphere_default();

        let xs = sphere.intersects(&ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 4.0);
        assert_eq!(xs.get(1).t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::with(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Shape::sphere_default();

        let xs = sphere.intersects(&ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 5.0);
        assert_eq!(xs.get(1).t, 5.0);
    }

    #[test]
    fn ray_misses_square() {
        let ray = Ray::with(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Shape::sphere_default();

        let xs = sphere.intersects(&ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let sphere = Shape::sphere_default();

        let xs = sphere.intersects(&ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, -1.0);
        assert_eq!(xs.get(1).t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::with(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let sphere = Shape::sphere_default();

        let xs = sphere.intersects(&ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, -6.0);
        assert_eq!(xs.get(1).t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_intersection() {
        let ray = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Shape::sphere_default();

        let xs = sphere.intersects(&ray);

        assert_eq!(xs.len(), 2);
        assert!(xs.get(0).object.eq(&sphere));
        assert!(xs.get(1).object.eq(&sphere));
    }

    #[test]
    fn translate_a_ray() {
        let r = Ray::with(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix::identity().translate(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);

        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::with(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix::identity().scale(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);

        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}
