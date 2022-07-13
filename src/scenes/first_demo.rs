use crate::scenes::Scene;
use crate::{black, color, rgb, white, Material, Matrix, Pattern, Shape};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_8};

pub fn maggies_nightmare() -> Scene {
    let mut floor_material = Material::from_pattern(
        Pattern::checkers(black(), white())
            .with_transformation(Matrix::identity().scale(0.8, 0.8, 0.8)),
    );
    floor_material.diffuse = 0.9;
    floor_material.specular = 0.1;
    floor_material.reflective = 0.3;
    let floor = Shape::plane_from_material(floor_material);

    // wall
    let mut wall_material = Material::from_pattern(
        Pattern::ring(color(1.0, 0.2, 0.2), white())
            .with_transformation(
                Matrix::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(2.0, 1.0, 1.0),
            )
            .with_noise(0.7),
    );
    wall_material.diffuse = 0.9;
    wall_material.specular = 0.1;
    let wall = Shape::plane_from_material(wall_material).with_transform(
        Matrix::identity()
            .rotate_z(FRAC_PI_2)
            .rotate_y(-FRAC_PI_8)
            .translate(2.5, 0.0, 0.0),
    );

    // large middle shapes
    let mut middle_material = Material::from_pattern(
        Pattern::striped(rgb(57, 128, 92), rgb(251, 221, 75))
            .with_transformation(
                Matrix::identity()
                    // .scale(0.4, 1.0, 1.0)
                    .rotate_z(FRAC_PI_4)
                    .rotate_x(FRAC_PI_4)
                    .rotate_y(FRAC_PI_3),
            )
            .with_noise(6.0),
    );
    middle_material.diffuse = 1.0;
    middle_material.specular = 0.5;
    let middle = Shape::sphere_from_material(middle_material)
        .with_transform(Matrix::identity().translate(-0.5, 1.0, 0.5));

    //smaller right shapes
    let mut right_material = Material::from_pattern(
        Pattern::striped(color(0.5, 1.0, 0.1), rgb(0, 0, 139)).with_transformation(
            Matrix::identity()
                .scale(0.2, 1.0, 1.0)
                .rotate_z(FRAC_PI_3)
                .rotate_x(FRAC_PI_3),
        ),
    );
    right_material.diffuse = 0.8;
    right_material.specular = 0.3;
    right_material.shininess = 300.0;
    let right = Shape::sphere_from_material(right_material).with_transform(
        Matrix::identity()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5),
    );

    // small left shapes
    let mut left_material = Material::from_pattern(
        Pattern::gradient(color(1.0, 0.8, 0.1), rgb(64, 224, 208)).with_transformation(
            Matrix::identity()
                .scale(2.0, 1.0, 1.0)
                .translate(1.0, 0.0, 0.0)
                .rotate_z(FRAC_PI_4),
        ),
    );
    left_material.diffuse = 0.8;
    left_material.specular = 0.3;
    let left = Shape::sphere_from_material(left_material).with_transform(
        Matrix::identity()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
    );

    let mut metal_material = Material::from_color(black());
    metal_material.reflective = 1.0;
    metal_material.specular = 0.3;
    metal_material.diffuse = 0.8;
    metal_material.shininess = 300.0;

    let dank = Shape::sphere_from_material(metal_material).with_transform(
        Matrix::identity()
            .scale(0.45, 0.45, 0.45)
            .translate(-2.5, 0.45, 0.75),
    );

    vec![floor, wall, middle, dank, right, left]
}
