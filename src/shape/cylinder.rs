use crate::matrix::is_equal_float;
use crate::rays::Ray;
use crate::tuple::Tuple;
use crate::vector;

pub fn cylinder_intersects(ray: &Ray, y_min: f64, y_max: f64) -> Vec<f64> {
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

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);

    let mut xs: Vec<f64> = Vec::with_capacity(2);

    let y0 = ray.origin.y + t0 * ray.direction.y;
    if y_min < y0 && y0 < y_max {
        xs.push(t0);
    }

    let y1 = ray.origin.y + t1 * ray.direction.y;
    if y_min < y1 && y1 < y_max {
        xs.push(t1);
    }

    xs
}

pub fn cylinder_normal_at(point: Tuple) -> Tuple {
    vector(point.x, 0.0, point.z)
}

#[cfg(test)]
mod cylinder_test {
    use crate::matrix::is_equal_float;
    use crate::rays::Ray;
    use crate::shape::ShapeType;
    use crate::tuple::{point, point_i, vector, vector_i, Tuple};
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

    #[parameterized(
    origin = {      point_i(1, 0, -5),  point_i(0, 0, -5),  point(0.5, 0.0, -5.0)},
    direction = {   vector_i(0, 0, 1),  vector_i(0, 0, 1),  vector(0.1, 1.0, 1.0)},
    t0 = {          5.0,                4.0,                6.80798},
    t1 = {          5.0,                6.0,                7.08872},
    )]
    fn ray_strikes_cylinder(origin: Tuple, direction: Tuple, t0: f64, t1: f64) {
        let cylinder = Shape::cylinder_default();
        let ray = Ray::with(origin, direction.normalize());

        let xs = cylinder.intersects(&ray);

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

        assert_eq!(cylinder.normal_at(point), normal);
    }

    #[test]
    fn default_bounds_of_cylinder() {
        let cylinder = Shape::cylinder_default();

        let (min, max) = match cylinder.shape_type {
            ShapeType::Cylinder(mi, ma) => (mi, ma),
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
        let cylinder = Shape::cylinder_bounded(1.0, 2.0);
        let ray = Ray::with(point, direction.normalize());

        assert_eq!(cylinder.intersects(&ray).xs.len(), count)
    }
}
