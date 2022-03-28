#[cfg(test)]
mod intersection_test {
    use crate::intersection::{Intersection, Intersections};
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector, EPSILON};

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

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::unit();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::unit();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::unit();
        let i = Intersection::new(1.0, &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        // inverted!
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::with_transform(Matrix::identity().translate(0.0, 0.0, 1.0));
        let intersection = Intersection::new(5.0, &shape);

        let comps = intersection.prepare_computations(&r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
