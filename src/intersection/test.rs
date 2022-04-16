#[cfg(test)]
mod intersection_test {
    use crate::intersection::{Intersection, Intersections};
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::tuple::{point, vector, EPSILON};
    use crate::{color, white, Material, PointLight, Shape, World};
    use std::detect::__is_feature_detected::sha;
    use std::f64::consts::SQRT_2;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Shape::sphere_default();
        let i = Intersection::new(3.5, &sphere);
        assert!(i.object.eq(&sphere));
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Shape::sphere_default();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Shape::sphere_default();
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
        let s = Shape::sphere_default();
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
        let s = Shape::sphere_default();
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
        let shape = Shape::sphere_default();
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
        let shape = Shape::sphere_default();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default();
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
        let shape = Shape::sphere_from_transform(Matrix::identity().translate(0.0, 0.0, 1.0));
        let intersection = Intersection::new(5.0, &shape);

        let comps = intersection.prepare_computations(&r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Shape::plane_default();
        let ray = Ray::with(
            point(0.0, 1.0, -1.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let intersection = Intersection::new(SQRT_2, &shape);

        let comps = intersection.prepare_computations(&ray);

        assert_eq!(
            comps.reflection_vector,
            vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        )
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut world = World::default_world();
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = world.objects.get_mut(1).unwrap();
        shape.material.ambient = 1.0;
    }
}
