#[cfg(test)]
mod camera_test {
    use crate::camera::Camera;
    use crate::color::{color, Color};
    use crate::io::save_to_file;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::matrix::{is_equal_float, Matrix};
    use crate::sphere::{ShapeInit, Sphere};
    use crate::transformation::view_transformation;
    use crate::tuple::{point, vector};
    use crate::world::World;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI};

    #[test]
    fn constructing_a_camera() {
        let camera = Camera::new(160, 120, PI / 2.0);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.field_of_view, PI / 2.0);
        assert_eq!(camera.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, PI / 2.0);
        assert!(is_equal_float(camera.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let camera = Camera::new(125, 200, PI / 2.0);
        assert!(is_equal_float(camera.pixel_size, 0.01));
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, point(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(ray.origin, point(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera = camera.set_transform(
            Matrix::identity()
                .translate(0.0, -2.0, 5.0)
                .rotate_y(PI / 4.0),
        );

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, point(0.0, 2.0, -5.0));
        assert_eq!(
            ray.direction,
            vector(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let world = World::default_world();
        let mut camera = Camera::new(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        camera = camera.set_transform(view_transformation(from, to, up));

        let image = camera.render(world);
        assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn render_a_scene() {
        // floor
        let mut wall_material = Material::with_color(color(1.0, 0.9, 0.9));
        wall_material.specular = 0.0;
        let floor = Sphere::from_material(wall_material.clone())
            .with_transform(Matrix::identity().scale(10.0, 0.01, 10.0));

        // left wall
        let mut left_wall = Sphere::from_material(wall_material.clone()).with_transform(
            Matrix::identity()
                .scale(10.0, 0.01, 10.0)
                .rotate_x(FRAC_PI_2)
                .rotate_y(-FRAC_PI_4)
                .translate(0.0, 0.0, 5.0),
        );

        // right wall
        let mut right_wall = Sphere::from_material(wall_material.clone()).with_transform(
            Matrix::identity()
                .scale(10.0, 0.01, 10.0)
                .rotate_x(FRAC_PI_2)
                .rotate_y(FRAC_PI_4)
                .translate(0.0, 0.0, 5.0),
        );

        // large middle sphere
        let mut middle_material = Material::with_color(color(0.1, 1.0, 0.5));
        middle_material.diffuse = 0.7;
        middle_material.specular = 0.3;
        let mut middle = Sphere::from_material(middle_material)
            .with_transform(Matrix::identity().translate(-0.5, 1.0, 0.5));

        //smaller right sphere
        let mut right_material = Material::with_color(color(0.5, 1.0, 0.1));
        right_material.diffuse = 0.7;
        right_material.specular = 0.3;
        let mut right = Sphere::from_material(right_material).with_transform(
            Matrix::identity()
                .scale(0.5, 0.5, 0.5)
                .translate(1.5, 0.5, -0.5),
        );

        // small left sphere
        let mut left_material = Material::with_color(color(1.0, 0.8, 0.1));
        left_material.diffuse = 0.7;
        left_material.specular = 0.3;
        let mut left = Sphere::from_material(left_material).with_transform(
            Matrix::identity()
                .scale(0.33, 0.33, 0.33)
                .translate(-1.5, 0.33, -0.75),
        );

        let world = World::with(
            vec![
                Box::new(floor),
                Box::new(left_wall),
                Box::new(right_wall),
                Box::new(middle),
                Box::new(right),
                Box::new(left),
            ],
            PointLight::with(point(-10.0, 10.0, -10.0), Color::white()),
        );

        let mut camera = Camera::new(100, 50, FRAC_PI_3);
        camera = camera.set_transform(view_transformation(
            point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vector(0.0, 1.0, 0.0),
        ));

        let canvas = camera.render(world);
        let res = save_to_file("src/camera/first_scene.ppm", canvas.to_ppm());

        assert!(res.is_ok());
    }
}
