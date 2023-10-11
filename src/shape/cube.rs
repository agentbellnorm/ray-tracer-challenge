use crate::intersection::{Intersection, Intersections};
use crate::rays::Ray;
use crate::shape::bounds::Bounds;
use crate::tuple::Tuple;
use crate::vector;

fn check_axis(origin: f64, direction: f64, axis_min: f64, axis_max: f64) -> (f64, f64) {
    let tmin_numerator = axis_min - origin;
    let tmax_numerator = axis_max - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    if tmin > tmax {
        return (tmax, tmin);
    }

    (tmin, tmax)
}

pub fn cube_intersects(ray: &Ray, bounds: &Bounds, shape_id: usize) -> Intersections {
    let (xt_min, xt_max) = check_axis(ray.origin.x, ray.direction.x, bounds.min.x, bounds.max.x);
    let (yt_min, yt_max) = check_axis(ray.origin.y, ray.direction.y, bounds.min.y, bounds.max.y);
    let (zt_min, zt_max) = check_axis(ray.origin.z, ray.direction.z, bounds.min.z, bounds.max.x);

    let t_min = xt_min.max(yt_min).max(zt_min);
    let t_max = xt_max.min(yt_max).min(zt_max);

    if t_min > t_max {
        return Intersections::empty();
    }

    Intersections::from(vec![
        Intersection::new(t_min, shape_id),
        Intersection::new(t_max, shape_id),
    ])
}

pub fn cube_normal_at(point: Tuple) -> Tuple {
    let max_c = point.x.abs().max(point.y.abs().max(point.z.abs()));

    if max_c == point.x.abs() {
        return vector(point.x, 0.0, 0.0);
    } else if max_c == point.y.abs() {
        return vector(0.0, point.y, 0.0);
    }

    vector(0.0, 0.0, point.z)
}

#[cfg(test)]
mod cube_test {
    use crate::rays::Ray;
    use crate::tuple::Tuple;
    use crate::tuple::{point, point_i, vector, vector_i};
    use crate::{Shape, World};
    use crate::intersection::Intersection;
    use parameterized::{ide, parameterized};

    ide!();

    #[parameterized(
    scenario= {     "+x",                   "-x",                   "+y",                   "-y",                   "+z",                   "-z",                   "inside"                },
    origin = {      point(5.0, 0.5, 0.0),   point(-5.0, 0.5, 0.0),  point(0.5, 5.0, 0.0),   point(0.5, -5.0, 0.0),  point(0.5, 0.0, 5.0),   point(0.5, 0.0, -5.0),  point(0.0, 0.5, 0.0)    },
    direction = {   vector_i(-1, 0, 0),     vector_i(1, 0, 0),      vector_i(0, -1, 0),     vector_i(0, 1, 0),      vector_i(0, 0, -1),     vector_i(0, 0, 1),      vector_i(0, 0, 1)       },
    t1 = {          4.0,                    4.0,                    4.0,                    4.0,                    4.0,                    4.0,                    -1.0                    },
    t2 = {          6.0,                    6.0,                    6.0,                    6.0,                    6.0,                    6.0,                    1.0                     }
    )]
    pub fn ray_intersects_cube(scenario: &str, origin: Tuple, direction: Tuple, t1: f64, t2: f64) {
        let cube = Shape::cube_default();
        let world = World::default().with_objects(vec![cube]);
        let ray = Ray::with(origin, direction);

        let xs = world.get_shape(0).intersects(&world, &ray);

        assert_eq!(xs.len(), 2, "{}", scenario);
        assert_eq!(xs.get(0).t, t1, "{}", scenario);
        assert_eq!(xs.get(1).t, t2, "{}", scenario);
    }

    #[parameterized(
    origin = {      point_i(-2, 0, 0),              point_i(0, -2, 0),              point_i(0, 0, -2),                point_i(2, 0, 2),   point_i(0, 2, 2),   point_i(2, 2, 0)    },
    direction = {   vector(0.2673, 0.5345, 0.8018), vector(0.8018, 0.2673, 0.5345), vector(0.5345, 0.8018, 0.2673), vector_i(0, 0, -1), vector_i(0, -1, 0), vector_i(-1, 0, 0)  }
    )]
    pub fn ray_misses_cube(origin: Tuple, direction: Tuple) {
        let mut cube = Shape::cube_default();
        cube.id = Some(0);
        let ray = Ray::with(origin, direction);

        let xs = cube.intersects(&World::default(), &ray);

        assert_eq!(xs.len(), 0)
    }

    #[parameterized(
    point = {   point(1.0, 0.5, -0.8),  point(-1.0, -0.2, 0.9), point(-0.4, 1.0, -0.1), point(0.3, -1.0, -0.7), point(-0.6, 0.3, 1.0),  point(0.4, 0.4, -1.0),  point_i(1, 1, 1),   point_i(-1, -1, -1) },
    normal = {  vector_i(1, 0, 0),      vector_i(-1, 0, 0),     vector_i(0, 1, 0),      vector_i(0, -1, 0),     vector_i(0, 0, 1),      vector_i(0, 0, -1),     vector_i(1, 0, 0),  vector_i(-1, 0, 0)  },
    )]
    fn normal_on_surface_of_cube(point: Tuple, normal: Tuple) {
        assert_eq!(
            Shape::cube_default().normal_at(&World::default(), point, &Intersection::new(0.0, 0)),
            normal
        )
    }
}
