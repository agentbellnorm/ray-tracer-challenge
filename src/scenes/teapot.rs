use std::fs;

use crate::{obj_file::add_obj_file, world::World};



pub fn teapot(mut world: World) -> World {
    let content = fs::read_to_string("src/scenes/files/teapot.obj").unwrap();
    add_obj_file(&mut world, &content);

    world
}
