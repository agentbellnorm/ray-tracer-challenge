use std::f64::consts::{FRAC_2_PI, FRAC_PI_6};

use crate::{matrix::Matrix, shape::Shape, world::World};

use super::Scene;

fn hexagon_corner() -> Shape {
    Shape::sphere_default().with_transform(
        Matrix::identity()
            .scale(0.25, 0.25, 0.25)
            .translate(0.0, 0.0, -1.0),
    )
}

fn hexagon_edge() -> Shape {
    Shape::cylinder(0.0, 1.0, false).with_transform(
        Matrix::identity()
            .scale(0.25, 1.0, 0.25)
            .rotate_z(-FRAC_2_PI)
            .rotate_y(-FRAC_PI_6)
            .translate(0.0, 0.0, -1.0),
    )
}

pub fn hexagon_scene(mut world: World) -> World {
    let hexagon = Shape::group();

    world = world.add_shape(hexagon);

    for i in [0..6] {
        let side = Shape::group();
        world = world.with_group_and_children(side, vec![hexagon_corner(), hexagon_edge()])
        
    }

    world
}
