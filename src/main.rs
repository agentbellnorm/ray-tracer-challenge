mod canvas;
mod color;
mod io;
mod tuple;

fn main() {
    println!("{:?}", tuple::point(1.0, 2.0, 3.4));
    println!("{:?}", tuple::vector(1.4, 2.8, 3.4));
}
