extern crate core;

pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod lights;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod perlin_noise;
pub mod rays;
pub mod scenes;
pub mod shape;
pub mod transformation;
pub mod tuple;
pub mod world;

use crate::camera::Camera;
use crate::color::{black, color, rgb, white};
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::pattern::Pattern;
use crate::scenes::hexagon::{hexagon_scene};
use crate::shape::Shape;
use crate::transformation::view_transformation;
use crate::tuple::{point, vector};
use crate::world::World;
use std::f64::consts::FRAC_PI_3;
use std::process::Command;

pub fn main_lib() {
    let mut world = World::with_light(PointLight::default());
    world = hexagon_scene(world);

    let (horizontal, vertical) = (640, 360);
    // let (horizontal, vertical) = (1280, 720);
    // let (horizontal, vertical) = (1920, 1080);

    let camera = Camera::new(horizontal, vertical, FRAC_PI_3).set_transform(view_transformation(
        point(0.0, 2.8, -5.0),
        point(0.0, 1.5, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    camera.render(world).save_to_file("src/main.ppm").unwrap();

    println!("{:?}", Command::new("open").arg("./src/main.ppm").output());
}
