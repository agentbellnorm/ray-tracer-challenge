use crate::{rgb, white, Material, Matrix, Pattern, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn shiny_scene() -> Vec<Shape> {
    let mut ground_materal = Material::from_color(rgb(255, 182, 193));
    ground_materal.diffuse = 0.9;
    ground_materal.specular = 0.1;
    let ground = Shape::plane_from_material(ground_materal);

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

    vec![ground, sky]
}
