use crate::rays::Ray;

pub fn cube_intersects(ray: &Ray) -> Vec<f64> {
    vec![]
}

#[cfg(test)]
mod cube_test {
    use crate::rays::Ray;
    use crate::shape::cube::cube_intersects;
    use crate::tuple::Tuple;
    use crate::tuple::{point, vector_i};
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
        let ray = Ray::with(origin, direction);

        let xs = cube_intersects(&ray);

        assert_eq!(xs.len(), 2, "{}", scenario);
        assert_eq!(xs[0], t1, "{}", scenario);
        assert_eq!(xs[1], t2, "{}", scenario);
    }
}
