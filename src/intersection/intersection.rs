use crate::rays::Ray;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, EPSILON};

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

pub struct PreparedComputation<'a> {
    pub object: &'a Sphere,
    pub t: f32,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> PreparedComputation {
        let point = ray.position(self.t);
        let mut normal_vector = self.object.normal_at(point);
        let eye_vector = -ray.direction;
        let inside = normal_vector.dot(&eye_vector) < 0.0;

        normal_vector = match inside {
            true => -normal_vector,
            false => normal_vector,
        };

        let over_point = point + (normal_vector * EPSILON);

        PreparedComputation {
            point,
            over_point,
            eye_vector,
            inside,
            t: self.t,
            object: self.object,
            normal_vector,
        }
    }
}

pub struct Intersections<'a> {
    pub xs: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    // Sounds like doing sorting here can become a problem in the future, see p. 66
    pub fn hit(&self) -> Option<Intersection<'a>> {
        let mut sorted = self.xs.clone();
        sorted.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        sorted.into_iter().find(|inter| inter.t > 0.0)
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn get(&self, index: usize) -> &Intersection<'a> {
        &self.xs[index]
    }
}
