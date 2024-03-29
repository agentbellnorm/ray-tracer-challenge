use crate::intersection::{Intersection, Intersections};
use crate::matrix::{is_equal_float, is_zero_float};
use crate::rays::Ray;
use crate::tuple::{Tuple, EPSILON};
use crate::vector;

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    x.powi(2) + z.powi(2) <= 1.0
}

fn intersect_caps(
    y_min: f64,
    y_max: f64,
    closed: bool,
    ray: &Ray,
    mut xs: Intersections,
    shape_id: usize,
) -> Intersections {
    if !closed || is_equal_float(ray.direction.y, 0.0) {
        return xs;
    }

    let t = (y_min - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t) {
        xs.push(Intersection::new(t, shape_id));
    }

    let t = (y_max - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t) {
        xs.push(Intersection::new(t, shape_id));
    }

    xs
}

pub fn cylinder_intersects(
    ray: &Ray,
    y_min: f64,
    y_max: f64,
    closed: bool,
    shape_id: usize,
) -> Intersections {
    let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
    let mut intersections: Intersections = Intersections::from(Vec::with_capacity(2));

    if is_zero_float(a) {
        return intersect_caps(y_min, y_max, closed, ray, intersections, shape_id);
    }

    let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
    let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

    let disc = b.powi(2) - 4.0 * a * c;

    if disc < 0.0 {
        return Intersections::empty();
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);

    let y0 = ray.origin.y + t0 * ray.direction.y;
    if y_min < y0 && y0 < y_max {
        intersections.push(Intersection::new(t0, shape_id));
    }

    let y1 = ray.origin.y + t1 * ray.direction.y;
    if y_min < y1 && y1 < y_max {
        intersections.push(Intersection::new(t1, shape_id));
    }

    intersect_caps(y_min, y_max, closed, ray, intersections, shape_id)
}

pub fn cylinder_normal_at(point: Tuple, y_min: f64, y_max: f64) -> Tuple {
    let dist = point.x.powi(2) + point.z.powi(2);

    if dist < 1.0 && point.y >= y_max - EPSILON {
        return vector(0.0, 1.0, 0.0);
    }

    if dist < 1.0 && point.y <= y_min + EPSILON {
        return vector(0.0, -1.0, 0.0);
    }

    vector(point.x, 0.0, point.z)
}

#[cfg(test)]
mod cylinder_test {
    use crate::matrix::is_equal_float;
    use crate::rays::Ray;
    use crate::shape::ShapeType::Cylinder;
    use crate::tuple::{point, point_i, vector, vector_i, Tuple};
    use crate::intersection::Intersection;
    use crate::{Shape, World};
    use parameterized::parameterized;

    #[parameterized(
    origin = {      point_i(1, 0, 0),   point_i(0, 0, 0),   point_i(0, 0, -5)   },
    direction = {   vector_i(0, 1, 0),  vector_i(0, 1, 0),  vector_i(1, 1, 1)   }
    )]
    fn ray_misses_cylinder(origin: Tuple, direction: Tuple) {
        let cylinder = Shape::cylinder_default();
        let world = World::default().with_objects(vec![cylinder]);
        let ray = Ray::with(origin, direction.normalize());

        assert_eq!(world.get_shape(0).intersects(&world, &ray).len(), 0)
    }

    #[parameterized(
    origin = {      point_i(1, 0, -5),  point_i(0, 0, -5),  point(0.5, 0.0, -5.0)},
    direction = {   vector_i(0, 0, 1),  vector_i(0, 0, 1),  vector(0.1, 1.0, 1.0)},
    t0 = {          5.0,                4.0,                6.80798},
    t1 = {          5.0,                6.0,                7.08872},
    )]
    fn ray_strikes_cylinder(origin: Tuple, direction: Tuple, t0: f64, t1: f64) {
        let mut world = World::default();
        let cylinder = world.add_shape(Shape::cylinder_default());

        let ray = Ray::with(origin, direction.normalize());

        let xs = world.get_shape(cylinder).intersects(&World::default(), &ray);

        assert_eq!(xs.len(), 2);
        assert!(is_equal_float(xs.get(0).t, t0));
        assert!(is_equal_float(xs.get(1).t, t1));
    }

    #[parameterized(
    point = {   point_i(1, 0 ,0),   point_i(0, 5, -1),  point_i(0, -2, 1),  point_i(-1, 1, 0)},
    normal = {  vector_i(1, 0, 0),  vector_i(0, 0, -1), vector_i(0, 0, 1),  vector_i(-1, 0, 0)},
    )]
    fn normal_vector_on_cylinder(point: Tuple, normal: Tuple) {
        let cylinder = Shape::cylinder_default();

        assert_eq!(cylinder.normal_at(&World::default(), point, &Intersection::bogus()), normal);
    }

    #[test]
    fn default_bounds_of_cylinder() {
        let cylinder = Shape::cylinder_default();

        let (min, max) = match cylinder.shape_type {
            Cylinder(mi, ma, false) => (mi, ma),
            _ => panic!("cylinder is no cylinder"),
        };

        assert_eq!(min, -f64::INFINITY);
        assert_eq!(max, f64::INFINITY);
    }

    #[parameterized(
    point = {       point(0.0, 1.5, 0.0),   point_i(0, 3, -5),  point_i(0, 0, -5),  point_i(0, 2, -5),  point_i(0, 1, -5),  point(0.0, 1.5, -2.0)},
    direction = {   vector(0.1, 1.0, 0.0),  vector_i(0, 0, 1),  vector_i(0, 0, 1),  vector_i(0, 0, 1),  vector_i(0, 0, 1),  vector_i(0, 0, 1)},
    count = {       0,                      0,                  0,                  0,                  0,                  2}
    )]
    fn intersecting_constrained_cylinder(point: Tuple, direction: Tuple, count: usize) {
        let mut world = World::default();
        let cylinder = world.add_shape(Shape::cylinder(1.0, 2.0, false));
        let ray = Ray::with(point, direction.normalize());

        assert_eq!(world.get_shape(cylinder).intersects(&world, &ray).xs.len(), count)
    }

    #[test]
    fn default_closed_value_for_cylinder() {
        if let Cylinder(_, _, closed) = Shape::cylinder_default().shape_type {
            assert!(!closed);
        } else {
            panic!("cylinder is no cylinder");
        }
    }

    #[parameterized(
    point = {       point_i(0, 3, 0),   point_i(0, 3, -2),  point_i(0, 4, -2),  point_i(0, 0, -2),  point_i(0, -1, -2)  },
    direction = {   vector_i(0, -1, 0), vector_i(0, -1, 2), vector_i(0, -1, 1), vector_i(0, 1, 2),  vector_i(0, 1, 1)   },
    count = {       2,                  2,                  2,                  2,                  2                   }
    )]
    fn intersecting_caps_of_closed_cylinder(point: Tuple, direction: Tuple, count: usize) {
        let cylinder = Shape::cylinder(1.0, 2.0, true);
        let world = World::default().with_objects(vec![cylinder]);
        let ray = Ray::with(point, direction.normalize());

        assert_eq!(world.get_shape(0).intersects(&world, &ray).xs.len(), count);
    }

    #[parameterized(
    point = {   point_i(0, 1, 0),    point(0.5, 1.0, 0.0),  point(0.0, 1.0, 0.5),   point_i(0, 2, 0),   point(0.5, 2.0, 0.0),   point(0.0, 2.0, 0.5)},
    normal = {  vector_i(0, -1, 0), vector_i(0, -1, 0),     vector_i(0, -1, 0),     vector_i(0, 1, 0),  vector_i(0, 1, 0),      vector_i(0, 1, 0)}
    )]
    fn normal_vector_on_cylinder_end_caps(point: Tuple, normal: Tuple) {
        let cylinder = Shape::cylinder(1.0, 2.0, true);
        let world = World::default().with_objects(vec![cylinder]);

        assert_eq!(world.get_shape(0).normal_at(&world, point, &Intersection::bogus()), normal)
    }
}
