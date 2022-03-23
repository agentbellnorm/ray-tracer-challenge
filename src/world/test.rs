#[cfg(test)]
mod world_test {
    use crate::color::color;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};
    use crate::world::World;

    #[test]
    fn default_world() {
        let light = PointLight::with(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

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
}
