use crate::canvas::Canvas;
use crate::color::color;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::point;
use crate::world::World;
use std::time::Instant;

pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f32,
    pub transform: Matrix,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f32) -> Camera {
        let half_view = f32::tan(field_of_view / 2.0);
        let aspect = (hsize as f32) / (vsize as f32);

        let half_width: f32;
        let half_height: f32;

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
            pixel_size: (half_width * 2.0) / (hsize as f32),
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let x_offset = ((px as f32) + 0.5) * self.pixel_size;
        let y_offset = ((py as f32) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inv_transform = self.transform.inverse(); // TODO: seems like this can be done once?

        let pixel = point(world_x, world_y, -1.0) * &inv_transform;
        let origin = point(0.0, 0.0, 0.0) * &inv_transform;
        let direction = (pixel - origin).normalize();

        Ray::with(origin, direction)
    }

    pub fn set_transform(mut self, transform: Matrix) -> Camera {
        self.transform = transform;
        self
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize, color(0.0, 0.0, 0.0));

        let start_time = Instant::now();

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image = image.write_pixel(x, y, color);
            }
        }

        let duration = start_time.elapsed().as_millis();

        println!("Rendered {} pixels", self.hsize * self.vsize);
        println!("Total duration: {} ms", duration);
        println!(
            "ms per pixel: {}",
            duration as f32 / (self.hsize * self.vsize) as f32
        );

        image
    }
}