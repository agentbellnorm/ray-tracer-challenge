use crate::matrix::is_equal_float;
use crate::rays::Ray;
use crate::shape::Shape;
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
    pub under_point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflection_vector: Tuple,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> PreparedComputation<'a> {
    pub fn is_opaque(&self) -> bool {
        is_equal_float(self.object.material.transparency, 0.0)
    }

    // todo read "Reflections and Refractions in Ray Tracing"
    pub fn schlick(&self) -> f64 {
        // cosine angle between eye and normal vector
        let mut cos = self.eye_vector.dot(&self.normal_vector);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            // cosine(theta) using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Shape) -> Intersection {
        Intersection { t, object }
    }

    pub fn prepare_computations(
        &self,
        ray: &Ray,
        intersections: &Intersections,
    ) -> PreparedComputation {
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
        let under_point = point - (normal_vector * EPSILON);

        let (n1, n2) = self.get_refractive_indices(intersections);

        PreparedComputation {
            point,
            over_point,
            under_point,
            eye_vector,
            inside,
            t: self.t,
            object: self.object,
            normal_vector,
            reflection_vector,
            n1,
            n2,
        }
    }

    fn get_refractive_indices(&self, xs: &Intersections) -> (f64, f64) {
        let mut n1: Option<f64> = None;
        let mut n2: Option<f64> = None;

        let mut containers: Vec<&Shape> = Vec::new();

        for i in 0..xs.len() {
            let current_intersection = xs.get(i);
            let hit_is_current_intersection = self == current_intersection;

            if hit_is_current_intersection {
                if containers.is_empty() {
                    n1 = Some(1.0);
                } else {
                    n1 = Some(containers.last().unwrap().material.refractive_index);
                }
            }

            if containers
                .iter()
                .any(|intersection_obj| *intersection_obj == current_intersection.object)
            {
                containers
                    .retain(|intersection_obj| *intersection_obj != current_intersection.object)
            } else {
                containers.push(current_intersection.object)
            }

            if hit_is_current_intersection {
                if containers.is_empty() {
                    n2 = Some(1.0);
                } else {
                    n2 = Some(containers.last().unwrap().material.refractive_index);
                }
            }
        }

        (n1.unwrap(), n2.unwrap())
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

    pub fn from(xs: Vec<Intersection<'a>>) -> Self {
        Intersections { xs }
    }
}
