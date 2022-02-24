use crate::rays::Sphere;

pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }
}
