extern crate core;

mod canvas;
mod color;
mod intersection;
mod io;
mod lights;
mod materials;
mod matrix;
mod rays;
mod sphere;
mod transformation;
mod tuple;
mod world;

fn main() {
    println!("{:?}", tuple::point(1.0, 2.0, 3.4));
    println!("{:?}", tuple::vector(1.4, 2.8, 3.4));
}
