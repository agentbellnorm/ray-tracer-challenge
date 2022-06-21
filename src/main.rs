extern crate core;

use crate::camera::Camera;
use crate::color::{black, color, rgb, white};
use crate::io::save_to_file;
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::pattern::Pattern;
use crate::scenes::{maggies_world, shiny_scene};
use crate::shapes::Shape;
use crate::transformation::view_transformation;
use crate::tuple::{point, vector};
use crate::world::World;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_8};
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
        maggies_world(),
        PointLight::with(point(-10.0, 10.0, -10.0), white()),
    );

    let camera = Camera::new(1920, 1080, FRAC_PI_3).set_transform(view_transformation(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    camera.render(world).save_to_file("src/main.ppm");

    println!("{:?}", Command::new("open").arg("./src/main.ppm").output());
}
