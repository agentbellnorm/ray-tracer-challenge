use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6};

use crate::{
    material::Material,
    matrix::Matrix,
    shape::{
        bounds::{bounds_to_cube, group_bounds},
        Shape, ShapeType,
    },
    world::World,
};

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
            .rotate_z(-FRAC_PI_2)
            .rotate_y(-FRAC_PI_6)
            .translate(0.0, 0.0, -1.0),
    )
}

pub fn hexagon_scene(mut world: World) -> World {
    let hexagon = world.add_shape(Shape::group());

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
    let hexagon_group = world.get_shape(hexagon);
    if let ShapeType::Group(_, bounds) = &hexagon_group.shape_type {
        let hexagon_bounds = bounds_to_cube(bounds).with_material(Material::wrapper());
        let box_id = world.add_shape(hexagon_bounds);
        world.add_shape_to_group(hexagon, box_id);
    }

    world
}
