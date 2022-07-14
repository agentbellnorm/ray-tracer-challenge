use crate::rays::Ray;

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    if tmin > tmax {
        return (tmax, tmin);
    }

    (tmin, tmax)
}

pub fn cube_intersects(ray: &Ray) -> Vec<f64> {
    let (xt_min, xt_max) = check_axis(ray.origin.x, ray.direction.x);
    let (yt_min, yt_max) = check_axis(ray.origin.y, ray.direction.y);
    let (zt_min, zt_max) = check_axis(ray.origin.z, ray.direction.z);

    let t_min = xt_min.max(yt_min).max(zt_min);
    let t_max = xt_max.min(yt_max).min(zt_max);

    if t_min > t_max {
        return vec![];
    }

    vec![t_min, t_max]
}

#[cfg(test)]
mod cube_test {
    use crate::rays::Ray;
    use crate::tuple::Tuple;
    use crate::tuple::{point, point_i, vector, vector_i};
    use crate::Shape;
    use parameterized::parameterized;

    #[parameterized(
    scenario= {     "+x",                   "-x",                   "+y",                   "-y",                   "+z",                   "-z",                   "inside"                },
    origin = {      point(5.0, 0.5, 0.0),   point(-5.0, 0.5, 0.0),  point(0.5, 5.0, 0.0),   point(0.5, -5.0, 0.0),  point(0.5, 0.0, 5.0),   point(0.5, 0.0, -5.0),  point(0.0, 0.5, 0.0)    },
    direction = {   vector_i(-1, 0, 0),     vector_i(1, 0, 0),      vector_i(0, -1, 0),     vector_i(0, 1, 0),      vector_i(0, 0, -1),     vector_i(0, 0, 1),      vector_i(0, 0, 1)       },
    t1 = {          4.0,                    4.0,                    4.0,                    4.0,                    4.0,                    4.0,                    -1.0                    },
    t2 = {          6.0,                    6.0,                    6.0,                    6.0,                    6.0,                    6.0,                    1.0                     }
    )]
    pub fn ray_intersects_cube(scenario: &str, origin: Tuple, direction: Tuple, t1: f64, t2: f64) {
        let cube = Shape::cube_default();
        let ray = Ray::with(origin, direction);

        let xs = cube.intersects(&ray);

        assert_eq!(xs.len(), 2, "{}", scenario);
        assert_eq!(xs.get(0).t, t1, "{}", scenario);
        assert_eq!(xs.get(1).t, t2, "{}", scenario);
    }

    #[parameterized(
    origin = {      point_i(-2, 0, 0),              point_i(0, -2, 0),              point_i(0, 0, -2),                point_i(2, 0, 2),   point_i(0, 2, 2),   point_i(2, 2, 0)    },
    direction = {   vector(0.2673, 0.5345, 0.8018), vector(0.8018, 0.2673, 0.5345), vector(0.5345, 0.8018, 0.2673), vector_i(0, 0, -1), vector_i(0, -1, 0), vector_i(-1, 0, 0)  }
    )]
    pub fn ray_misses_cube(origin: Tuple, direction: Tuple) {
        let cube = Shape::cube_default();
        let ray = Ray::with(origin, direction);

        let xs = cube.intersects(&ray);

        assert_eq!(xs.len(), 0)
    }
}
