use crate::intersection::{Intersection, Intersections};
use crate::matrix::is_zero_float;
use crate::rays::Ray;
use crate::tuple::{Tuple, EPSILON};
use crate::vector;

fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    x.powi(2) + z.powi(2) <= radius.powi(2)
}

fn intersect_caps(
    y_min: f64,
    y_max: f64,
    closed: bool,
    ray: &Ray,
    mut xs: Intersections,
    shape_id: usize,
) -> Intersections {
    if !closed || is_zero_float(ray.direction.y) {
        return xs;
    }

    let t = (y_min - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, y_min) {
        xs.push(Intersection::new(t, shape_id));
    }

    let t = (y_max - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, y_max) {
        xs.push(Intersection::new(t, shape_id));
    }

    xs
}

pub fn cone_intersects(
    ray: &Ray,
    y_min: f64,
    y_max: f64,
    closed: bool,
    shape_id: usize,
) -> Intersections {
    let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
    let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
        + 2.0 * ray.origin.z * ray.direction.z;
    let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

    let mut xs: Intersections = Intersections::from(Vec::with_capacity(4));

    if is_zero_float(a) && is_zero_float(b) {
        return intersect_caps(y_min, y_max, closed, ray, xs, shape_id);
    }

    if is_zero_float(a) && !is_zero_float(b) {
        xs.push(Intersection::new(-c / (2.0 * b), shape_id));
    }

    let disc = b.powi(2) - 4.0 * a * c;

    if disc < 0.0 {
        return xs;
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);

    let y0 = ray.origin.y + t0 * ray.direction.y;
    if y_min < y0 && y0 < y_max {
        xs.push(Intersection::new(t0, shape_id));
    }

    let y1 = ray.origin.y + t1 * ray.direction.y;
    if y_min < y1 && y1 < y_max {
        xs.push(Intersection::new(t1, shape_id));
    }

    intersect_caps(y_min, y_max, closed, ray, xs, shape_id)
}

pub fn cone_normal_at(point: Tuple, y_min: f64, y_max: f64) -> Tuple {
    let dist = point.x.powi(2) + point.z.powi(2);

    if dist < 1.0 && point.y >= y_max - EPSILON {
        return vector(0.0, 1.0, 0.0);
    }

    if dist < 1.0 && point.y <= y_min + EPSILON {
        return vector(0.0, -1.0, 0.0);
    }

    let mut y = (point.x.powi(2) + point.z.powi(2)).sqrt();

    if point.y > 0.0 {
        y = -y;
    }
    vector(point.x, y, point.z)
}

#[cfg(test)]
mod cone_test {
    use crate::matrix::is_equal_float;
    use crate::rays::Ray;
    use crate::shape::cone::cone_normal_at;
    use crate::tuple::{point, point_i, vector, vector_i, Tuple};
    use crate::{Shape, World};
    use parameterized::parameterized;
    use std::f64::consts::SQRT_2;

    #[parameterized(
    origin = {      point_i(0, 0, -5),   point_i(0, 0, -5),  point_i(1, 1, -5)       },
    direction = {   vector_i(0, 0, 1),  vector_i(1, 1, 1),  vector(-0.5, -1.0, 1.0) },
    t0 = {          5.0,                8.66025,            4.55006                 },
    t1 = {          5.0,                8.66025,            49.44994                }
    )]
    fn intersecting_cone_with_ray(origin: Tuple, direction: Tuple, t0: f64, t1: f64) {
        let world = World::default().with_objects(vec![Shape::cone_default()]);
        let ray = Ray::with(origin, direction.normalize());

        let xs = world.get_shape(0).intersects(&world, &ray);

        assert_eq!(xs.len(), 2);
        assert!(is_equal_float(xs.get(0).t, t0));
        assert!(is_equal_float(xs.get(1).t, t1));
    }

    #[test]
    fn intersecting_cone_with_ray_parallel_to_one_of_its_halves() {
        let cone = Shape::cone_default();
        let world = World::default().with_objects(vec![cone]);
        let ray = Ray::with(point_i(0, 0, -1), vector_i(0, 1, 1).normalize());

        let xs = world.get_shape(0).intersects(&world, &ray);

        assert_eq!(xs.len(), 1);
        assert!(is_equal_float(xs.get(0).t, 0.35355));
    }

    #[parameterized(
    origin = {      point_i(0, 0, -5),  point(0.0, 0.0, -0.25), point(0.0, 0.0, -0.25)  },
    direction = {   vector_i(0, 1, 0),  vector_i(0, 1, 1),      vector_i(0, 1, 0)       },
    count = {       0,                  2,                      4                       }
    )]
    fn intersecting_a_cones_end_caps(origin: Tuple, direction: Tuple, count: usize) {
        let cone = Shape::cone(-0.5, 0.5, true);
        let world = World::default().with_objects(vec![cone]);
        let ray = Ray::with(origin, direction.normalize());

        let xs = world.get_shape(0).intersects(&world, &ray);

        assert_eq!(xs.len(), count)
    }

    #[parameterized(
    point = {   point_i(0, 0, 0),   point_i(1, 1, 1),           point_i(-1, -1, 0)  },
    normal = {  vector_i(0, 0, 0),  vector(1.0, -SQRT_2, 1.0),  vector_i(-1, 1, 0)  }
    )]
    fn normal_vector_on_cone(point: Tuple, normal: Tuple) {
        assert_eq!(cone_normal_at(point, -f64::INFINITY, f64::INFINITY), normal)
    }
}
