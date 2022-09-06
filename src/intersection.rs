use crate::rays::Ray;
use crate::tuple::{Tuple, EPSILON};
use crate::world::ShapeId;
use crate::World;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Intersection {
    pub t: f64,
    pub object_id: ShapeId,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PreparedComputation {
    pub object: ShapeId,
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

impl PreparedComputation {
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

impl Intersection {
    pub fn new(t: f64, object_id: ShapeId) -> Intersection {
        Intersection { t, object_id }
    }

    pub fn prepare_computations(
        &self,
        world: &World,
        ray: &Ray,
        intersections: &Intersections,
    ) -> PreparedComputation {
        let point = ray.position(self.t);
        let mut normal_vector = world.get_shape(self.object_id).normal_at(world, point);
        let eye_vector = -ray.direction;
        let inside = normal_vector.dot(&eye_vector) < 0.0;

        normal_vector = match inside {
            true => -normal_vector,
            false => normal_vector,
        };

        let reflection_vector = ray.direction.reflect(&normal_vector);

        let over_point = point + (normal_vector * EPSILON);
        let under_point = point - (normal_vector * EPSILON);

        let (n1, n2) = self.get_refractive_indices(world, intersections);

        PreparedComputation {
            point,
            over_point,
            under_point,
            eye_vector,
            inside,
            t: self.t,
            object: self.object_id,
            normal_vector,
            reflection_vector,
            n1,
            n2,
        }
    }

    fn get_refractive_indices(&self, world: &World, xs: &Intersections) -> (f64, f64) {
        let mut n1: Option<f64> = None;
        let mut n2: Option<f64> = None;

        let mut containers: Vec<ShapeId> = Vec::new();

        for i in 0..xs.len() {
            let current_intersection = xs.get(i);
            let hit_is_current_intersection = self == current_intersection;

            if hit_is_current_intersection {
                if containers.is_empty() {
                    n1 = Some(1.0);
                } else {
                    n1 = Some(
                        world
                            .get_shape(*containers.last().unwrap())
                            .material
                            .refractive_index,
                    );
                }
            }

            if containers
                .iter()
                .any(|intersection_obj| *intersection_obj == current_intersection.object_id)
            {
                containers
                    .retain(|intersection_obj| *intersection_obj != current_intersection.object_id)
            } else {
                containers.push(current_intersection.object_id)
            }

            if hit_is_current_intersection {
                if containers.is_empty() {
                    n2 = Some(1.0);
                } else {
                    n2 = Some(
                        world
                            .get_shape(*containers.last().unwrap())
                            .material
                            .refractive_index,
                    );
                }
            }
        }

        (n1.unwrap(), n2.unwrap())
    }
}

pub struct Intersections {
    pub xs: Vec<Intersection>,
}

impl Intersections {
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

    pub fn from(xs: Vec<Intersection>) -> Self {
        Intersections { xs }
    }
}
