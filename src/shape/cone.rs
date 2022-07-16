use crate::rays::Ray;
use crate::tuple::Tuple;
use crate::vector;

pub fn cone_intersects(ray: &Ray, y_min: f64, y_max: f64, closed: bool) -> Vec<f64> {
    panic!("not implemented")
}

pub fn cone_normal_at(point: Tuple, y_min: f64, y_max: f64) -> Tuple {
    panic!("not implemented")
}

#[cfg(test)]
mod cone_test {
    use crate::rays::Ray;
    use crate::tuple::{point, point_i, vector, vector_i, Tuple};
    use crate::Shape;
    use parameterized::parameterized;
    use std::f64::consts::SQRT_2;

    #[parameterized(
    origin = {      point_i(0, 0, 5),   point_i(0, 0, -5),  point_i(1, 1, -5)       },
    direction = {   vector_i(0, 0, 1),  vector_i(1, 1, 1),  vector(-0.5, -1.0, 1.0) },
    t0 = {          5.0,                8.66025,            4.55006                 },
    t1 = {          5.0,                8.66025,            49.44994                }
    )]
    fn intersecting_cone_with_ray(origin: Tuple, direction: Tuple, t0: f64, t1: f64) {
        let cone = Shape::cone_default();
        let ray = Ray::with(origin, direction.normalize());

        let xs = cone.intersects(&ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, t0);
        assert_eq!(xs.get(1).t, t1);
    }

    #[test]
    fn intersecting_cone_with_ray_parallel_to_one_of_its_halves() {
        let cone = Shape::cone_default();
        let direction = vector_i(0, 1, 1).normalize();
        let ray = Ray::with(point_i(0, 0, -1), direction);

        let xs = cone.intersects(&ray);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs.get(0).t, 0.35355);
    }

    #[parameterized(
    origin = {      point_i(0, 0, -5),  point(0.0, 0.0, -0.25), point(0.0, 0.0, -0.25)  },
    direction = {   vector_i(0, 1, 0),  vector_i(0, 1, 1),      vector_i(0, 1, 0)       },
    count = {       0,                  2,                      4                       }
    )]
    fn intersecting_a_cones_end_caps(origin: Tuple, direction: Tuple, count: usize) {
        let cone = Shape::cone(-0.5, 0.5, true);
        let ray = Ray::with(origin, direction.normalize());

        let xs = cone.intersects(&ray);

        assert_eq!(xs.len(), count)
    }

    #[parameterized(
    point = {   point_i(0, 0, 0),   point_i(1, 1, 1),           point_i(-1, -1, 0)  },
    normal = {  vector_i(0, 0, 0),  vector(1.0, -SQRT_2, 1.0),  vector_i(-1, 1, 0)  }
    )]
    fn normal_vector_on_cone(point: Tuple, normal: Tuple) {
        assert_eq!(Shape::cone_default().normal_at(point), normal)
    }
}
