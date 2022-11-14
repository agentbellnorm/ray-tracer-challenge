#[cfg(test)]
mod intersection_test {
    use parameterized::parameterized;
    use ray_tracer_challenge::color::{black, color};
    use ray_tracer_challenge::intersection::{Intersection, Intersections};
    use ray_tracer_challenge::material::Material;
    use ray_tracer_challenge::matrix::{is_equal_float, Matrix};
    use ray_tracer_challenge::pattern::Pattern;
    use ray_tracer_challenge::rays::Ray;
    use ray_tracer_challenge::shape::Shape;
    use ray_tracer_challenge::tuple::{point, point_i, vector, vector_i, EPSILON};
    use ray_tracer_challenge::world::World;
    use std::f64::consts::SQRT_2;
    use std::vec;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Shape::sphere_default();
        let i = Intersection::new(3.5, sphere.clone().pack());
        assert!(i.object.as_ref().eq(&sphere));
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Shape::sphere_default().pack();
        let i1 = Intersection {
            t: 1.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: 2.0,
            object: s.clone(),
        };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Shape::sphere_default().pack();
        let i1 = Intersection {
            t: -1.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: 1.0,
            object: s.clone(),
        };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_negative() {
        let s = Shape::sphere_default().pack();
        let i1 = Intersection {
            t: -2.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: -1.0,
            object: s.clone(),
        };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() {
        let s = Shape::sphere_default().pack();
        let i1 = Intersection {
            t: 5.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: 7.0,
            object: s.clone(),
        };
        let i3 = Intersection {
            t: -3.0,
            object: s.clone(),
        };
        let i4 = Intersection {
            t: 2.0,
            object: s.clone(),
        };
        let xs = Intersections {
            xs: vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()],
        };

        assert_eq!(xs.hit().unwrap(), i4);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default().pack();
        let i = Intersection::new(4.0, shape.clone());

        let comps = i.prepare_computations(&r, &Intersections::from(vec![i.clone()]));

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object.as_ref());
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default();
        let i = Intersection::new(4.0, shape.pack());

        let comps = i.prepare_computations(&r, &Intersections::from(vec![i.clone()]));

        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default();
        let i = Intersection::new(1.0, shape.pack());

        let comps = i.prepare_computations(&r, &Intersections::from(vec![i.clone()]));

        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        // inverted!
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_from_transform(Matrix::identity().translate(0.0, 0.0, 1.0));
        let intersection = Intersection::new(5.0, shape.pack());

        let comps =
            intersection.prepare_computations(&r, &Intersections::from(vec![intersection.clone()]));

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
        let intersection = Intersection::new(SQRT_2, shape.pack());

        let comps = intersection
            .prepare_computations(&ray, &Intersections::from(vec![intersection.clone()]));

        assert_eq!(
            comps.reflection_vector,
            vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        )
    }

    #[parameterized(
        index = {0, 1, 2, 3, 4, 5},
        n1 = {1.0, 1.5, 2.0, 2.5, 2.5, 1.5},
        n2 = {1.5, 2.0, 2.5, 2.5, 1.5, 1.0}
    )]
    fn finding_n1_and_n2_at_various_intersections(index: usize, n1: f64, n2: f64) {
        let mut a = Shape::sphere_glass().with_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let a_rc = a.pack();

        let mut b =
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let b_rc = b.pack();

        let mut c =
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let c_rc = c.pack();

        let r = Ray::with(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections {
            xs: vec![
                Intersection::new(2.0, a_rc.clone()),
                Intersection::new(2.75, b_rc.clone()),
                Intersection::new(3.25, c_rc.clone()),
                Intersection::new(4.75, b_rc.clone()),
                Intersection::new(5.25, c_rc.clone()),
                Intersection::new(6.0, a_rc.clone()),
            ],
        };

        let comps = xs.get(index).prepare_computations(&r, &xs);

        assert_eq!(comps.n1, n1);
        assert_eq!(comps.n2, n2);
    }

    #[test]
    fn under_point_is_offset_below_the_surface() {
        let r = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));
        let shape =
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, shape.pack());
        let xs = Intersections::from(vec![i.clone()]);

        let comps = i.prepare_computations(&r, &xs);

        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z)
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = World::default_world();
        let shape = w.objects.get(0).unwrap();
        let r = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));
        let xs = Intersections::from(vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape.clone()),
        ]);

        let comps = xs.get(0).prepare_computations(&r, &xs);

        assert_eq!(w.refracted_color(&comps, 5), black());
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = World::default_world();

        let first_object = w.objects[0].clone();
        w.objects[0] = Shape {
            material: Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..first_object.material
            },
            ..first_object.as_ref().clone()
        }
        .pack();

        let shape = w.objects.get(0).unwrap();
        let r = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));
        let xs = Intersections::from(vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape.clone()),
        ]);
        let comps = xs.get(0).prepare_computations(&r, &xs);

        assert_eq!(w.refracted_color(&comps, 0), black());
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default_world();

        let first_object = w.objects[0].clone();
        w.objects[0] = Shape {
            material: Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..first_object.material
            },
            ..first_object.as_ref().clone()
        }
        .pack();

        let shape = w.objects.get(0).unwrap();
        let r = Ray::with(point(0.0, 0.0, SQRT_2 / 2.0), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-SQRT_2 / 2.0, shape.clone()),
            Intersection::new(SQRT_2 / 2.0, shape.clone()),
        ]);
        let comps = xs.get(1).prepare_computations(&r, &xs);

        assert_eq!(w.refracted_color(&comps, 5), black());
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::default_world();
        // w.objects.get_mut(0).unwrap().material.ambient = 1.0;
        // w.objects.get_mut(0).unwrap().material.pattern = Some(Pattern::test());

        let first_object = w.objects[0].clone();
        w.objects[0] = Shape {
            material: Material {
                ambient: 1.0,
                pattern: Some(Pattern::test()),
                ..first_object.material
            },
            ..first_object.as_ref().clone()
        }
        .pack();

        let second_object = w.objects[1].clone();
        w.objects[1] = Shape {
            material: Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..second_object.material
            },
            ..second_object.as_ref().clone()
        }
        .pack();

        let a = w.objects.get(0).unwrap();
        let b = w.objects.get(1).unwrap();

        let r = Ray::with(point(0.0, 0.0, 0.1), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-0.9899, a.clone()),
            Intersection::new(-0.4899, b.clone()),
            Intersection::new(0.4899, b.clone()),
            Intersection::new(0.9899, a.clone()),
        ]);

        let comps = xs.get(2).prepare_computations(&r, &xs);

        assert_eq!(
            w.refracted_color(&comps, 5),
            color(0.0, 0.998874, 0.0472189)
        )
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::default_world();

        let mut floor_material = Material::default();
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;
        let floor = Shape::plane_from_material(floor_material)
            .with_transform(Matrix::identity().translate(0.0, -1.0, 0.0));

        let mut ball_material = Material::from_color(color(1.0, 0.0, 0.0));
        ball_material.ambient = 0.5;
        let ball = Shape::sphere_from_material(ball_material)
            .with_transform(Matrix::identity().translate(0.0, -3.5, -0.5));

        w = w.add_object(floor.clone());
        w = w.add_object(ball);

        let ray = Ray::with(point_i(0, 0, -3), vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0));
        let xs = Intersections::from(vec![Intersection::new(SQRT_2, floor.pack())]);

        let comps = xs.get(0).prepare_computations(&ray, &xs);
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = Shape::sphere_glass().pack();
        let ray = Ray::with(point(0.0, 0.0, SQRT_2 / 2.0), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-SQRT_2 / 2.0, shape.clone()),
            Intersection::new(SQRT_2 / 2.0, shape.clone()),
        ]);

        let comps = xs.get(1).prepare_computations(&ray, &xs);

        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Shape::sphere_glass().pack();
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-1.0, shape.clone()),
            Intersection::new(1.0, shape.clone()),
        ]);

        let comps = xs.get(1).prepare_computations(&ray, &xs);

        let reflectance = comps.schlick();
        assert!(is_equal_float(reflectance, 0.04));
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = Shape::sphere_glass();
        let ray = Ray::with(point(0.0, 0.99, -2.0), vector_i(0, 0, 1));
        let xs = Intersections::from(vec![Intersection::new(1.8589, shape.pack())]);

        let comps = xs.get(0).prepare_computations(&ray, &xs);

        let reflectance = comps.schlick();
        assert!(is_equal_float(reflectance, 0.48873));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::default_world();
        let ray = Ray::with(point_i(0, 0, -3), vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0));

        let mut floor_material = Material::default();
        floor_material.transparency = 0.5;
        floor_material.reflective = 0.5;
        floor_material.refractive_index = 1.5;
        let floor = Shape::plane_from_material(floor_material)
            .with_transform(Matrix::identity().translate(0.0, -1.0, 0.0));

        let mut ball_material = Material::from_color(color(1.0, 0.0, 0.0));
        ball_material.ambient = 0.5;
        let ball = Shape::sphere_from_material(ball_material)
            .with_transform(Matrix::identity().translate(0.0, -3.5, -0.5));

        w = w.add_object(floor.clone());
        w = w.add_object(ball);

        let xs = Intersections::from(vec![Intersection::new(SQRT_2, floor.pack())]);

        let comps = xs.get(0).prepare_computations(&ray, &xs);
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}
