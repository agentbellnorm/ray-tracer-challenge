use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6, FRAC_PI_8};

use crate::{matrix::Matrix, shape::Shape, world::World};

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
            .rotate_z(-FRAC_PI_2)
            .rotate_y(-FRAC_PI_6)
            .translate(0.0, 0.0, -1.0),
    )
}

pub fn hexagon_scene(mut world: World) -> World {
    let hexagon =
        world.add_shape(Shape::group().with_transform(Matrix::identity().rotate_z(-FRAC_PI_8 / 3.0)));

    for i in 0..6 {
        let corner = world.add_shape(hexagon_corner());
        let edge = world.add_shape(hexagon_edge());

        let side = world.add_shape(
            Shape::group().with_transform(Matrix::identity().rotate_y(f64::from(i) * FRAC_PI_3)),
        );

        world.add_shape_to_group(side, corner);
        world.add_shape_to_group(side, edge);
        world.add_shape_to_group(hexagon, side);
    }

    world
}
