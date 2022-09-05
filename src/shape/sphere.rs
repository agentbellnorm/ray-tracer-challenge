use crate::rays::Ray;
use crate::tuple::{point, Tuple};

pub fn sphere_normal_at(object_point: Tuple) -> Tuple {
    object_point - point(0.0, 0.0, 0.0)
}

pub fn sphere_intersects(transformed_ray: &Ray) -> Vec<f64> {
    let sphere_to_ray = transformed_ray.origin - point(0.0, 0.0, 0.0);

    let a = transformed_ray.direction.dot(&transformed_ray.direction);
    let b = transformed_ray.direction.dot(&sphere_to_ray) * 2.0;
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return Vec::new();
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    vec![t1, t2]
}

#[cfg(test)]
mod sphere_test {
    use crate::canvas::Canvas;
    use crate::color::{black, color};
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::shape::Shape;
    use crate::tuple::{point, vector};
    use crate::World;
    use std::f64::consts::PI;

    #[test]
    fn default_transformation() {
        let s = Shape::sphere_default();
        assert_eq!(s.inverse_transformation, Matrix::identity().inverse());
    }

    #[test]
    fn change_transformation() {
        let t = Matrix::identity().translate(2.0, 3.0, 4.0);
        let s = Shape::sphere_from_transform(t);

        assert_eq!(s.inverse_transformation, t.inverse());
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere_from_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let xs = s.intersects(&World::default(), &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 3.0);
        assert_eq!(xs.get(1).t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere_from_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let xs = s.intersects(&World::default(), &r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn draw_circle() {
        let size = 100;
        let mut canvas = Canvas::new(size, size, black());

        let ray_origin = point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / (size as f64);
        let half = wall_size / 2.0;

        let sphere = Shape::sphere_from_transform(
            Matrix::identity().scale(0.5, 1.0, 1.0).rotate_z(PI / 4.0),
        );

        for y in 0..size {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..size {
                let world_x = -half + pixel_size * (x as f64);
                let position = point(world_x, world_y, wall_z);
                let ray = Ray::with(ray_origin, (position - ray_origin).normalize());

                if sphere.intersects(&World::default(), &ray).hit().is_some() {
                    canvas = canvas.write_pixel(x, y, color(1.0, 0.0, 0.0));
                }
            }
        }

        let res = canvas.save_to_file("tests/output/cirkel.ppm");

        assert!(res.is_ok());
    }

    #[test]
    fn normal_on_sphere_point_on_x() {
        let s = Shape::sphere_default();

        assert_eq!(
            s.normal_at(&World::default(), point(1.0, 0.0, 0.0)),
            vector(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn normal_on_sphere_point_on_y() {
        let s = Shape::sphere_default();

        assert_eq!(
            s.normal_at(&World::default(), point(0.0, 1.0, 0.0)),
            vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn normal_on_sphere_point_on_z() {
        let s = Shape::sphere_default();

        assert_eq!(
            s.normal_at(&World::default(), point(0.0, 0.0, 1.0)),
            vector(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn normal_on_sphere_nonaxial_point() {
        let s = Shape::sphere_default();

        assert_eq!(
            s.normal_at(
                &World::default(),
                point(
                    f64::sqrt(3.0) / 3.0,
                    f64::sqrt(3.0) / 3.0,
                    f64::sqrt(3.0) / 3.0
                )
            ),
            vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Shape::sphere_default();

        let n = s.normal_at(
            &World::default(),
            point(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
            ),
        );

        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Shape::sphere_from_transform(Matrix::identity().translate(0.0, 1.0, 0.0));

        assert_eq!(
            s.normal_at(
                &World::default(),
                point(0.0, 1.70711, -std::f64::consts::FRAC_1_SQRT_2)
            ),
            vector(
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2
            )
        );
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let transform = Matrix::identity().rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        let s = Shape::sphere_from_transform(transform);

        assert_eq!(
            s.normal_at(
                &World::default(),
                point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
            ),
            vector(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Shape::sphere_default();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn sphere_can_have_material_assigned() {
        let mut s = Shape::sphere_default();
        let mut m = Material::default();

        m.ambient = 1.0;
        s = s.with_material(m);

        assert_eq!(s.material, m);
    }
    // TODO: refactor to use world
    // #[test]
    // fn draw_3d_sphere() {
    //     let size = 100;
    //     let mut canvas = Canvas::new(size, size, black());
    //
    //     let ray_origin = point(0.0, 0.0, -5.0);
    //     let wall_z = 10.0;
    //     let wall_size = 7.0;
    //     let pixel_size = wall_size / (size as f64);
    //     let half = wall_size / 2.0;
    //
    //     let material = Material::from_color(color(1.0, 0.2, 1.0));
    //     let sphere = Shape::sphere_from_material(material)
    //         .with_transform(Matrix::identity().scale(1.0, 0.9, 1.0).rotate_z(-0.4));
    //
    //     let light_position = point(-10.0, 0.0, -10.0);
    //     let light_color = white();
    //     let light = PointLight::with(light_position, light_color);
    //
    //     for y in 0..size {
    //         let world_y = half - pixel_size * (y as f64);
    //         for x in 0..size {
    //             let world_x = -half + pixel_size * (x as f64);
    //             let position = point(world_x, world_y, wall_z);
    //             let ray = Ray::with(ray_origin, (position - ray_origin).normalize());
    //
    //             canvas = match sphere.intersects(&World::default(), &ray).hit() {
    //                 Some(hit) => {
    //                     let point_on_sphere = ray.position(hit.t);
    //                     let normal_on_sphere = hit.object.normal_at(point_on_sphere);
    //                     let eye = -ray.direction;
    //                     let color = hit.object.material.lighting(
    //                         &Shape::sphere_default(),
    //                         &light,
    //                         point_on_sphere,
    //                         eye,
    //                         normal_on_sphere,
    //                         false,
    //                     );
    //                     canvas.write_pixel(x, y, color)
    //                 }
    //                 None => canvas,
    //             }
    //         }
    //     }
    //
    //     let res = canvas.save_to_file("tests/output/shape.ppm");
    //
    //     assert!(res.is_ok());
    // }

    #[test]
    fn helper_for_producing_sphere_with_glassy_material() {
        let s = Shape::sphere_glass();

        // not storing the actual transformation, so comparing the inverse instead
        assert_eq!(s.inverse_transformation, Matrix::identity().inverse());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
