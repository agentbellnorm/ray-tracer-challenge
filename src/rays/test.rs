#[cfg(test)]
mod template_test {
    use crate::rays::{Ray, Sphere};
    use crate::tuple::{point, vector};

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
        let sphere = Sphere::unit();

        let xs = sphere.intersects(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 4.0);
        assert_eq!(xs.get(1).t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::with(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let xs = sphere.intersects(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 5.0);
        assert_eq!(xs.get(1).t, 5.0);
    }

    #[test]
    fn ray_misses_square() {
        let ray = Ray::with(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let xs = sphere.intersects(ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let xs = sphere.intersects(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, -1.0);
        assert_eq!(xs.get(1).t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::with(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let xs = sphere.intersects(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, -6.0);
        assert_eq!(xs.get(1).t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_intersection() {
        let ray = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let xs = sphere.intersects(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).object, &sphere);
        assert_eq!(xs.get(1).object, &sphere);
    }
}
