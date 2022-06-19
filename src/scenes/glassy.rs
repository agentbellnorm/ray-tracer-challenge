use std::f64::consts::FRAC_PI_2;
use crate::{Material, Matrix, Pattern, rgb, Shape, white};

pub fn maggies_world() -> Vec<Shape> {
    let mut grass_material =
        Material::from_pattern(Pattern::gradient(rgb(255, 182, 193), rgb(124, 252, 0)));
    grass_material.diffuse = 0.9;
    grass_material.specular = 0.1;
    let grass = Shape::plane_from_material(grass_material);

    let mut sky_material = Material::from_pattern(
        Pattern::gradient(rgb(135, 206, 250), white()).with_transformation(Matrix::identity()),
    );
    sky_material.diffuse = 0.9;
    sky_material.specular = 0.1;
    let sky = Shape::plane_from_material(sky_material).with_transform(
        Matrix::identity()
            .scale(20.0, 1.0, 1.0)
            .rotate_x(FRAC_PI_2)
            .rotate_z(FRAC_PI_2)
            .translate(0.0, 0.0, 40.0),
    );