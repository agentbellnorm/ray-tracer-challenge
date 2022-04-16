use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tuple::{Tuple, EPSILON};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

pub struct PreparedComputation<'a> {
    pub object: &'a Shape,
    pub t: f64,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflection_vector: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Shape) -> Intersection {
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

        let reflection_vector = ray.direction.reflect(&normal_vector);

        let over_point = point + (normal_vector * EPSILON);

        PreparedComputation {
            point,
            over_point,
            eye_vector,
            inside,
            t: self.t,
            object: self.object,
            normal_vector,
            reflection_vector,
        }
    }
}

pub struct Intersections<'a> {
    pub xs: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    // Sounds like doing sorting here can become a problem in the future, see p. 66
    pub fn hit(&self) -> Option<Intersection> {
        let mut sorted = self.xs.clone();
        sorted.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        sorted.into_iter().find(|inter| inter.t > 0.0)
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    pub fn get(&self, index: usize) -> &Intersection {
        &self.xs[index]
    }
}
