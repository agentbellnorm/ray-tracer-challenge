extern crate core;

use crate::camera::Camera;
use crate::color::{black, color, rgb, white};
use crate::io::save_to_file;
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::pattern::Pattern;
use crate::scenes::shiny_scene;
use crate::shapes::Shape;
use crate::transformation::view_transformation;
use crate::tuple::{point, vector};
use crate::world::World;
use std::f64::consts::FRAC_PI_3;
use std::process::Command;

mod camera;
mod canvas;
mod color;
mod intersection;
mod io;
mod lights;
mod material;
mod matrix;
mod pattern;
mod rays;
mod scenes;
mod shapes;
mod transformation;
mod tuple;
mod world;

fn main() {
    let world = World::with(
        shiny_scene(),
        PointLight::with(point(-10.0, 10.0, -10.0), white()),
    );

    // let (horizontal, vertical) = (640, 360);
    // let (horizontal, vertical) = (1280, 720);
    let (horizontal, vertical) = (1920, 1080);

    let camera = Camera::new(horizontal, vertical, FRAC_PI_3).set_transform(view_transformation(
        point(0.0, 2.8, -5.0),
        point(0.0, 1.5, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    camera.render(world).save_to_file("src/main.ppm").unwrap();

    println!("{:?}", Command::new("open").arg("./src/main.ppm").output());
}
