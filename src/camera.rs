use crate::canvas::Canvas;
use crate::color::black;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::point;
use crate::world::World;
use std::time::Instant;

pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix,
    inverse_transform: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Camera {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect = (hsize as f64) / (vsize as f64);

        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            inverse_transform: Matrix::identity().inverse(),
            pixel_size: (half_width * 2.0) / (hsize as f64),
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let x_offset = ((px as f64) + 0.5) * self.pixel_size;
        let y_offset = ((py as f64) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inv_transform = self.inverse_transform;

        let pixel = &point(world_x, world_y, -1.0) * &inv_transform;
        let origin = &point(0.0, 0.0, 0.0) * &inv_transform;
        let direction = (pixel - origin).normalize();

        Ray::with(origin, direction)
    }

    pub fn set_transform(mut self, transform: Matrix) -> Camera {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
        self
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize, black());

        let n_pixels = self.vsize * self.hsize;

        let start_time = Instant::now();

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, 5);
                image = image.write_pixel(x, y, color);
            }
        }

        let duration = start_time.elapsed();

        println!("Rendered {} pixels", n_pixels);
        println!("Total duration: {}s", duration.as_secs());
        println!(
            "ms per pixel: {}",
            duration.as_millis() as f64 / n_pixels as f64
        );

        image
    }
}
