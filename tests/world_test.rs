#[cfg(test)]
mod world_test {
    use ray_tracer_challenge::color::{black, color, white};
    use ray_tracer_challenge::intersection::{Intersection, Intersections};
    use ray_tracer_challenge::lights::PointLight;
    use ray_tracer_challenge::material::Material;
    use ray_tracer_challenge::matrix::Matrix;
    use ray_tracer_challenge::rays::Ray;
    use ray_tracer_challenge::shape::Shape;
    use ray_tracer_challenge::tuple::{point, vector};
    use ray_tracer_challenge::world::World;
    use std::f64::consts::SQRT_2;

    #[test]
    fn default_world() {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), white());

        let mut material = Material::from_color(color(0.8, 1.0, 0.6));
        material.diffuse = 0.7;
        material.specular = 0.2;

        let s1 = Shape::sphere_from_material(material);
        let s2 = Shape::sphere_from_transform(Matrix::identity().scale(0.5, 0.5, 0.5));

        let default_world = World::default_world();

        assert_eq!(&default_world.light_source, &light);
        assert!(default_world.has_object(&s1));
        assert!(default_world.has_object(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let world = World::default_world();
        let ray = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let xs = world.intersect_world(&ray);

        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs.get(0).t);
        assert_eq!(4.5, xs.get(1).t);
        assert_eq!(5.5, xs.get(2).t);
        assert_eq!(6.0, xs.get(3).t);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default_world();
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shapes = w.objects.get(0).unwrap();
        let i = Intersection::new(4.0, shapes.clone());

        let comps = i.prepare_computations(&r, &Intersections::from(vec![i.clone()]));
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default_world();
        w.light_source = PointLight::with(point(0.0, 0.25, 0.0), white());
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shapes = w.objects.get(1).unwrap();
        let i = Intersection::new(0.5, shapes.clone());

        let comps = i.prepare_computations(&r, &Intersections::from(vec![i.clone()]));
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let s1 = Shape::sphere_default();
        let s2 = Shape::sphere_from_transform(Matrix::identity().translate(0.0, 0.0, 10.0));
        let w = World::with(
            vec![s1, s2.clone()],
            PointLight::with(point(0.0, 0.0, -10.0), white()),
        );
        let i = Intersection::new(4.0, s2.pack());
        let r = Ray::with(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));

        let comps = i.prepare_computations(&r, &Intersections::from(vec![i.clone()]));
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::default_world();
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));

        let c = w.color_at(&r, 5);

        assert_eq!(c, black());
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default_world();
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let c = w.color_at(&r, 5);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), white());

        let mut material = Material::from_color(color(0.8, 1.0, 0.6));
        material.diffuse = 0.7;
        material.specular = 0.2;
        material.ambient = 1.0;

        let outer = Shape::sphere_from_material(material);
        let inner = Shape::sphere_from_transform(Matrix::identity().scale(0.5, 0.5, 0.5))
            .with_material(material);
        let inner_color = inner.material.color;

        let w = World::with(vec![outer, inner], light);
        // above is default world with some tweaks

        let r = Ray::with(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));

        let c = w.color_at(&r, 5);

        assert_eq!(c, inner_color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_position_and_light() {
        let world = World::default_world();
        let p = point(0.0, 10.0, 0.0);

        assert!(!world.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let world = World::default_world();
        let p = point(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let world = World::default_world();
        let p = point(-20.0, 20.0, -20.0);

        assert!(!world.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let world = World::default_world();
        let p = point(-2.0, 2.0, -2.0);

        assert!(!world.is_shadowed(p));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut world = World::default_world();
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

        let second_obj = world.objects[1].clone();
        world.objects[1] = Shape {
            material: Material {
                ambient: 1.0,
                ..second_obj.material
            },
            ..second_obj.as_ref().clone()
        }
        .pack();

        let intersection = Intersection::new(1.0, world.objects.get(1).unwrap().clone());

        let comps = intersection
            .prepare_computations(&ray, &Intersections::from(vec![intersection.clone()]));

        assert_eq!(world.reflected_color(&comps, 5), black())
    }

    #[test]
    fn shade_hit_for_reflective_material() {
        let mut world = World::default_world();
        let mut material = Material::default();
        material.reflective = 0.5;
        let plane = Shape::plane_from_material(material)
            .with_transform(Matrix::identity().translate(0.0, -1.0, 0.0));
        world = world.add_object(plane);
        let ray = Ray::with(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = Intersection::new(SQRT_2, world.objects.get(2).unwrap().clone());

        let comps = i.prepare_computations(&ray, &Intersections::from(vec![i.clone()]));

        assert_eq!(
            world.shade_hit(&comps, 5).de_normalized(),
            color(0.87677, 0.92436, 0.82918).de_normalized()
        )
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let light = PointLight::with(point(0.0, 0.0, 0.0), white());
        let mut material = Material::default();
        material.reflective = 1.0;

        let lower = Shape::plane_from_material(material)
            .with_transform(Matrix::identity().translate(0.0, -1.0, 0.0));

        let upper = Shape::plane_from_material(material)
            .with_transform(Matrix::identity().translate(0.0, 1.0, 0.0));

        let world = World::with(vec![lower, upper], light);
        let ray = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));

        let c = world.color_at(&ray, 5);

        assert_eq!(color(11.4, 11.4, 11.4), c, "color at terminated")
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut world = World::default_world();
        let mut material = Material::default();
        material.reflective = 0.5;

        let plane = Shape::plane_from_material(material)
            .with_transform(Matrix::identity().translate(0.0, -1.0, 0.0));

        world = world.add_object(plane);

        let ray = Ray::with(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = Intersection::new(SQRT_2, world.objects.get(2).unwrap().clone());
        let comps = i.prepare_computations(&ray, &Intersections::from(vec![i.clone()]));

        assert_eq!(world.reflected_color(&comps, 0), black())
    }
}
