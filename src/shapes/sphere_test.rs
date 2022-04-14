#[cfg(test)]
mod sphere_test {
    use crate::canvas::Canvas;
    use crate::color::{black, color, white};
    use crate::io::save_to_file;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::shapes::{sphere_default, sphere_from_material, sphere_from_transform};
    use crate::tuple::{point, vector};
    use std::f64::consts::PI;

    #[test]
    fn default_transformation() {
        let s = sphere_default();
        assert_eq!(*s.get_transformation(), Matrix::identity());
    }

    #[test]
    fn change_transformation() {
        let t = Matrix::identity().translate(2.0, 3.0, 4.0);
        let s = sphere_from_transform(t.clone());

        assert_eq!(*s.get_transformation(), t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere_from_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 3.0);
        assert_eq!(xs.get(1).t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere_from_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let xs = s.intersects(&r);

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

        let sphere =
            sphere_from_transform(Matrix::identity().scale(0.5, 1.0, 1.0).rotate_z(PI / 4.0));

        for y in 0..size {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..size {
                let world_x = -half + pixel_size * (x as f64);
                let position = point(world_x, world_y, wall_z);
                let ray = Ray::with(ray_origin, (position - ray_origin).normalize());

                if sphere.intersects(&ray).hit().is_some() {
                    canvas = canvas.write_pixel(x, y, color(1.0, 0.0, 0.0));
                }
            }
        }

        let res = save_to_file("src/shapes/cirkel.ppm", canvas.to_ppm());

        assert!(res.is_ok());
    }

    #[test]
    fn normal_on_sphere_point_on_x() {
        let s = sphere_default();

        assert_eq!(s.normal_at(point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_point_on_y() {
        let s = sphere_default();

        assert_eq!(s.normal_at(point(0.0, 1.0, 0.0)), vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_point_on_z() {
        let s = sphere_default();

        assert_eq!(s.normal_at(point(0.0, 0.0, 1.0)), vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_nonaxial_point() {
        let s = sphere_default();

        assert_eq!(
            s.normal_at(point(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )),
            vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = sphere_default();

        let n = s.normal_at(point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = sphere_from_transform(Matrix::identity().translate(0.0, 1.0, 0.0));

        assert_eq!(
            s.normal_at(point(0.0, 1.70711, -std::f64::consts::FRAC_1_SQRT_2)),
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
        let s = sphere_from_transform(transform);

        assert_eq!(
            s.normal_at(point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)),
            vector(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn sphere_has_default_material() {
        let s = sphere_default();

        assert_eq!(*s.get_material(), Material::new());
    }

    #[test]
    fn sphere_can_have_material_assigned() {
        let mut s = sphere_default();
        let mut m = Material::new();

        m.ambient = 1.0;
        s = s.with_material(m.clone());

        assert_eq!(*s.get_material(), m);
    }

    #[test]
    fn draw_3d_sphere() {
        let size = 100;
        let mut canvas = Canvas::new(size, size, black());

        let ray_origin = point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / (size as f64);
        let half = wall_size / 2.0;

        let material = Material::with_color(color(1.0, 0.2, 1.0));
        let sphere = sphere_from_material(material)
            .with_transform(Matrix::identity().scale(1.0, 0.9, 1.0).rotate_z(-0.4));

        let light_position = point(-10.0, 0.0, -10.0);
        let light_color = white();
        let light = PointLight::with(light_position, light_color);

        for y in 0..size {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..size {
                let world_x = -half + pixel_size * (x as f64);
                let position = point(world_x, world_y, wall_z);
                let ray = Ray::with(ray_origin, (position - ray_origin).normalize());

                canvas = match sphere.intersects(&ray).hit() {
                    Some(hit) => {
                        let point_on_sphere = ray.position(hit.t);
                        let normal_on_sphere = hit.object.normal_at(point_on_sphere);
                        let eye = -ray.direction;
                        let color = hit.object.get_material().lighting(
                            &sphere_default(),
                            &light,
                            point_on_sphere,
                            eye,
                            normal_on_sphere,
                            false,
                        );
                        canvas.write_pixel(x, y, color)
                    }
                    None => canvas,
                }
            }
        }

        let res = save_to_file("src/shapes/shapes.ppm", canvas.to_ppm());

        assert!(res.is_ok());
    }
}
