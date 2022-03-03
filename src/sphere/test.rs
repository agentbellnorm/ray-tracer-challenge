#[cfg(test)]
mod sphere_test {
    use crate::canvas::Canvas;
    use crate::color::color;
    use crate::io::save_to_file;
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};
    use std::f32::consts::PI;

    #[test]
    fn default_transformation() {
        let s = Sphere::unit();
        assert_eq!(s.transformation, Matrix::identity());
    }

    #[test]
    fn change_transformation() {
        let mut s = Sphere::unit();
        let t = Matrix::identity().translate(2.0, 3.0, 4.0);

        s = s.set_transform(t.clone());
        assert_eq!(s.transformation, t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::unit();

        s = s.set_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 3.0);
        assert_eq!(xs.get(1).t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::unit();

        s = s.set_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn draw_circle() {
        let size = 100;
        let mut canvas = Canvas::new(size, size, color(0.0, 0.0, 0.0));

        let ray_origin = point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / (size as f32);
        let half = wall_size / 2.0;

        let mut sphere = Sphere::unit();
        sphere = sphere.set_transform(Matrix::identity().scale(0.5, 1.0, 1.0).rotate_z(PI / 4.0));

        for y in 0..size {
            let world_y = half - pixel_size * (y as f32);
            for x in 0..size {
                let world_x = -half + pixel_size * (x as f32);
                let position = point(world_x, world_y, wall_z);
                let ray = Ray::with(ray_origin, (position - ray_origin).normalize());

                if sphere.intersects(ray).hit().is_some() {
                    canvas = canvas.write_pixel(x, y, color(255.0, 0.0, 0.0));
                }
            }
        }

        let res = save_to_file("src/sphere/cirkel.ppm", canvas.to_ppm());

        assert!(res.is_ok());
    }
}
