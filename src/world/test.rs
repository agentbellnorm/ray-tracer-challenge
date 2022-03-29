#[cfg(test)]
mod world_test {
    use crate::color::{color, Color};
    use crate::intersection::Intersection;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};
    use crate::world::World;

    #[test]
    fn default_world() {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), Color::white());

        let mut material = Material::with_color(color(0.8, 1.0, 0.6));
        material.diffuse = 0.7;
        material.specular = 0.2;

        let s1 = Sphere::with_material(material);
        let s2 = Sphere::unit().set_transform(Matrix::identity().scale(0.5, 0.5, 0.5));

        let default_world = World::default_world();

        assert_eq!(default_world.light_source, light);
        assert!(default_world
            .objects
            .clone()
            .into_iter()
            .find(|s| s.eq(&s1))
            .is_some());
        assert!(default_world
            .objects
            .into_iter()
            .find(|s| s.eq(&s2))
            .is_some());
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
        let shape = w.objects.get(0).unwrap();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default_world();
        w.light_source = PointLight::with(point(0.0, 0.25, 0.0), Color::white());
        let r = Ray::with(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects.get(1).unwrap();
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let s1 = Sphere::unit();
        let s2 = Sphere::unit().set_transform(Matrix::identity().translate(0.0, 0.0, 10.0));
        let w = World::with(
            vec![s1, s2.clone()],
            PointLight::with(point(0.0, 0.0, -10.0), Color::white()),
        );
        let i = Intersection::new(4.0, &s2);
        let r = Ray::with(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::default_world();
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));

        let c = w.color_at(&r);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default_world();
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let c = w.color_at(&r);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default_world();
        let r = Ray::with(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));

        let outer = w.objects.get_mut(0).unwrap();
        outer.material.ambient = 1.0;

        let inner = w.objects.get_mut(1).unwrap();
        inner.material.ambient = 1.0;
        let inner_color = inner.material.color;

        let c = w.color_at(&r);

        assert_eq!(c, inner_color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_position_and_light() {
        let world = World::default_world();
        let p = point(0.0, 10.0, 0.0);

        assert_eq!(world.is_shadowed(p), false);
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let world = World::default_world();
        let p = point(10.0, -10.0, 10.0);

        assert_eq!(world.is_shadowed(p), true);
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let world = World::default_world();
        let p = point(-20.0, 20.0, -20.0);

        assert_eq!(world.is_shadowed(p), false);
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let world = World::default_world();
        let p = point(-2.0, 2.0, -2.0);

        assert_eq!(world.is_shadowed(p), false);
    }
}
