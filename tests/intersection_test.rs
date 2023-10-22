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
        let sphere_id = 0;
        let i = Intersection::new(3.5, sphere_id);
        assert!(i.object_id.eq(&sphere_id));
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let i1 = Intersection {
            t: 1.0,
            object_id: 1,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: 2.0,
            object_id: 2,
            u: None,
            v: None,
        };
        let xs = Intersections { xs: vec![i1, i2] };

        assert_eq!(xs.hit().unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let i1 = Intersection {
            t: -1.0,
            object_id: 1,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: 1.0,
            object_id: 2,
            u: None,
            v: None,
        };
        let xs = Intersections { xs: vec![i1, i2] };

        assert_eq!(xs.hit().unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_negative() {
        let i1 = Intersection {
            t: -2.0,
            object_id: 1,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: -1.0,
            object_id: 2,
            u: None,
            v: None,
        };
        let xs = Intersections { xs: vec![i1, i2] };

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() {
        let i1 = Intersection {
            t: 5.0,
            object_id: 1,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: 7.0,
            object_id: 2,
            u: None,
            v: None,
        };
        let i3 = Intersection {
            t: -3.0,
            object_id: 3,
            u: None,
            v: None,
        };
        let i4 = Intersection {
            t: 2.0,
            object_id: 4,
            u: None,
            v: None,
        };
        let xs = Intersections {
            xs: vec![i1, i2, i3, i4],
        };

        assert_eq!(xs.hit().unwrap(), i4);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default();
        let world = World::default().with_objects(vec![shape]);
        let i = Intersection::new(4.0, 0);

        let comps = i.prepare_computations(&world, &r, &Intersections::from(vec![i.clone()]));

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object_id);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default();
        let world = World::default().with_objects(vec![shape]);
        let i = Intersection::new(4.0, 0);

        let comps = i.prepare_computations(&world, &r, &Intersections::from(vec![i.clone()]));

        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere_default();
        let world = World::default().with_objects(vec![shape]);
        let i = Intersection::new(1.0, 0);

        let comps = i.prepare_computations(&world, &r, &Intersections::from(vec![i.clone()]));

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
        let world = World::default().with_objects(vec![shape]);
        let intersection = Intersection::new(5.0, 0);

        let comps = intersection.prepare_computations(
            &world,
            &r,
            &Intersections::from(vec![intersection.clone()]),
        );

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Shape::plane_default();
        let world = World::default().with_objects(vec![shape]);
        let ray = Ray::with(
            point(0.0, 1.0, -1.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let intersection = Intersection::new(SQRT_2, 0);

        let comps = intersection.prepare_computations(
            &world,
            &ray,
            &Intersections::from(vec![intersection.clone()]),
        );

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
        let a_id = 0;

        let mut b =
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let b_id = 1;

        let mut c =
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let c_id = 2;

        let r = Ray::with(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));

        let world = World::default().with_objects(vec![a, b, c]);

        let xs = Intersections {
            xs: vec![
                Intersection::new(2.0, a_id),
                Intersection::new(2.75, b_id),
                Intersection::new(3.25, c_id),
                Intersection::new(4.75, b_id),
                Intersection::new(5.25, c_id),
                Intersection::new(6.0, a_id),
            ],
        };

        let comps = xs.get(index).prepare_computations(&world, &r, &xs);

        assert_eq!(comps.n1, n1);
        assert_eq!(comps.n2, n2);
    }

    #[test]
    fn under_point_is_offset_below_the_surface() {
        let r = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));
        let shape =
            Shape::sphere_glass().with_transform(Matrix::identity().translate(0.0, 0.0, 1.0));
        let world = World::default().with_objects(vec![shape]);
        let i = Intersection::new(5.0, 0);
        let xs = Intersections::from(vec![i.clone()]);

        let comps = i.prepare_computations(&world, &r, &xs);

        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z)
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = World::test_world();
        let first_test_world_item_id = 0;
        let r = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));
        let xs = Intersections::from(vec![
            Intersection::new(4.0, first_test_world_item_id),
            Intersection::new(6.0, first_test_world_item_id),
        ]);

        let comps = xs.get(0).prepare_computations(&w, &r, &xs);

        assert_eq!(w.refracted_color(&comps, 5), black());
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = World::test_world();
        w.objects.get_mut(0).unwrap().shape.material.transparency = 1.0;
        w.objects
            .get_mut(0)
            .unwrap()
            .shape
            .material
            .refractive_index = 1.5;
        let first_world_shape_id = 0;
        let r = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));
        let xs = Intersections::from(vec![Intersection::new(4.0, 0), Intersection::new(6.0, 0)]);
        let comps = xs
            .get(first_world_shape_id)
            .prepare_computations(&w, &r, &xs);

        assert_eq!(w.refracted_color(&comps, 0), black());
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::test_world();
        w.objects.get_mut(0).unwrap().shape.material.transparency = 1.0;
        w.objects
            .get_mut(0)
            .unwrap()
            .shape
            .material
            .refractive_index = 1.5;
        let test_world_first_item_id = 0;
        let r = Ray::with(point(0.0, 0.0, SQRT_2 / 2.0), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-SQRT_2 / 2.0, test_world_first_item_id),
            Intersection::new(SQRT_2 / 2.0, test_world_first_item_id),
        ]);
        let comps = xs.get(1).prepare_computations(&w, &r, &xs);

        assert_eq!(w.refracted_color(&comps, 5), black());
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::test_world();
        w.objects.get_mut(0).unwrap().shape.material.ambient = 1.0;
        w.objects.get_mut(0).unwrap().shape.material.pattern = Some(Pattern::test());

        w.objects.get_mut(1).unwrap().shape.material.transparency = 1.0;
        w.objects
            .get_mut(1)
            .unwrap()
            .shape
            .material
            .refractive_index = 1.5;

        // from the order of creation in test_world
        let a_id = 0;
        let b_id = 1;

        let r = Ray::with(point(0.0, 0.0, 0.1), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-0.9899, a_id),
            Intersection::new(-0.4899, b_id),
            Intersection::new(0.4899, b_id),
            Intersection::new(0.9899, a_id),
        ]);

        let comps = xs.get(2).prepare_computations(&w, &r, &xs);

        assert_eq!(
            w.refracted_color(&comps, 5),
            color(0.0, 0.998874, 0.0472189)
        )
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::test_world();

        let mut floor_material = Material::default();
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;
        let floor = Shape::plane_from_material(floor_material)
            .with_transform(Matrix::identity().translate(0.0, -1.0, 0.0));

        let mut ball_material = Material::from_color(color(1.0, 0.0, 0.0));
        ball_material.ambient = 0.5;
        let ball = Shape::sphere_from_material(ball_material)
            .with_transform(Matrix::identity().translate(0.0, -3.5, -0.5));

        let floor_id = w.add_shape(floor.clone());
        w.add_shape(ball);

        let ray = Ray::with(point_i(0, 0, -3), vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0));
        let xs = Intersections::from(vec![Intersection::new(SQRT_2, floor_id)]);

        let comps = xs.get(0).prepare_computations(&w, &ray, &xs);
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = Shape::sphere_glass();
        let world = World::default().with_objects(vec![shape]);
        let ray = Ray::with(point(0.0, 0.0, SQRT_2 / 2.0), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![
            Intersection::new(-SQRT_2 / 2.0, 0),
            Intersection::new(SQRT_2 / 2.0, 0),
        ]);

        let comps = xs.get(1).prepare_computations(&world, &ray, &xs);

        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Shape::sphere_glass();
        let world = World::default().with_objects(vec![shape]);
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector_i(0, 1, 0));
        let xs = Intersections::from(vec![Intersection::new(-1.0, 0), Intersection::new(1.0, 0)]);

        let comps = xs.get(1).prepare_computations(&world, &ray, &xs);

        let reflectance = comps.schlick();
        assert!(is_equal_float(reflectance, 0.04));
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = Shape::sphere_glass();
        let world = World::default().with_objects(vec![shape]);
        let ray = Ray::with(point(0.0, 0.99, -2.0), vector_i(0, 0, 1));
        let xs = Intersections::from(vec![Intersection::new(1.8589, 0)]);

        let comps = xs.get(0).prepare_computations(&world, &ray, &xs);

        let reflectance = comps.schlick();
        assert!(is_equal_float(reflectance, 0.48873));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::test_world();
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

        let floor_id = w.add_shape(floor.clone());
        w.add_shape(ball);

        let xs = Intersections::from(vec![Intersection::new(SQRT_2, floor_id)]);

        let comps = xs.get(0).prepare_computations(&w, &ray, &xs);
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}
