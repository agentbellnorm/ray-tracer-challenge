#[cfg(test)]
mod intersection_test {
    use crate::intersection::Intersection;
    use crate::rays::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new();
        let i = Intersection::new(3.5, &sphere);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &sphere);
    }
}
