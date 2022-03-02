#[cfg(test)]
mod intersection_test {
    use crate::intersection::{Intersection, Intersections};
    use crate::sphere::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::unit();
        let i = Intersection::new(3.5, &sphere);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &sphere);
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Sphere::unit();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Sphere::unit();
        let i1 = Intersection {
            t: -1.0,
            object: &s,
        };
        let i2 = Intersection { t: 1.0, object: &s };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_negative() {
        let s = Sphere::unit();
        let i1 = Intersection {
            t: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &s,
        };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() {
        let s = Sphere::unit();
        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i4);
    }
}
