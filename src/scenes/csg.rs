use std::f64::consts::FRAC_PI_2;

use crate::{
    color::{self, black, rgb, white},
    material::Material,
    matrix::Matrix,
    pattern::Pattern,
    shape::{CsgType, Shape},
    world::World,
};

pub fn csg(mut world: World) -> World {
    let side = 6.0;
    let half_side = side / 2.0;
    let quarter_side = side / 4.0;

    // room
    world.add_shape(
        Shape::plane_default()
            .with_material(Material::from_pattern(Pattern::checkers(
                rgb(237, 234, 203),
                rgb(42, 39, 31),
            )))
            .with_transform(Matrix::identity().translate(0.0, -half_side, 0.0)),
    );

    world.add_shape(
        Shape::plane_default()
            .with_material(Material::from_pattern(Pattern::checkers(
                rgb(237, 234, 203),
                rgb(42, 39, 31),
            )))
            .with_transform(
                Matrix::identity()
                    .rotate_x(FRAC_PI_2)
                    .translate(0.0, 0.0, 11.0),
            ),
    );

    // right branch

    let cylinder_1 = world.add_shape(
        Shape::cylinder(-half_side, half_side, true)
            .with_material(Material::pastel(rgb(239, 17, 0))),
    );
    let cylinder_2 = world.add_shape(
        Shape::cylinder(-half_side, half_side, true)
            .with_transform(Matrix::identity().rotate_z(FRAC_PI_2))
            .with_material(Material::pastel(rgb(244, 121, 2))),
    );
    let csg_cyl_1_2 = world.create_csg(CsgType::UNION, cylinder_1, cylinder_2);

    let cylinder_3 = world.add_shape(
        Shape::cylinder(-half_side, half_side, true)
            .with_transform(Matrix::identity().rotate_x(FRAC_PI_2))
            .with_material(Material::pastel(rgb(227, 224, 8))),
    );

    let tripple_cross_csg = world.create_csg(CsgType::UNION, csg_cyl_1_2, cylinder_3);

    //left branch

    let cube_id = world.add_shape(
        Shape::cube_default()
            .with_transform(Matrix::identity().scale(quarter_side, quarter_side, quarter_side))
            .with_material(Material::chrome()),
    );
    let sphere_id = world.add_shape(
        Shape::sphere_default()
            .with_transform(Matrix::identity().scale(half_side, half_side, half_side))
            .with_material(Material::darker_chrome()),
    );

    let cube_sphere = world.create_csg(CsgType::INTERSECTION, sphere_id, cube_id);

    // root

    world.create_csg(CsgType::DIFFERENCE, cube_sphere, tripple_cross_csg);

    world
}
