use crate::shape::Scene;
use crate::{rgb, white, Material, Matrix, Pattern, Shape};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_8};

fn daisy() -> Scene<'static> {
    let daisy_position = Matrix::identity()
        .translate(1.5, 1.5, 0.7)
        .rotate_y(FRAC_PI_8)
        .rotate_x(FRAC_PI_8);

    let mut center_material = Material::from_color(rgb(255, 215, 0));
    center_material.diffuse = 0.8;
    center_material.specular = 0.3;
    center_material.shininess = 300.0;
    let center = Shape::sphere_from_material(center_material).with_transform(
        Matrix::identity()
            .scale(0.25, 0.25, 0.05)
            .apply(&daisy_position),
    );

    let mut stem_material = Material::from_color(rgb(0, 128, 0));
    stem_material.diffuse = 0.8;
    stem_material.specular = 0.3;
    stem_material.shininess = 300.0;
    let stem = Shape::sphere_from_material(stem_material).with_transform(
        Matrix::identity()
            .scale(0.05, 0.7, 0.05)
            .translate(0.0, 0.7, 0.0)
            .translate(1.7, 0.0, 0.6),
    );

    let mut petal_material = Material::from_color(white());
    petal_material.diffuse = 0.8;
    stem_material.specular = 0.3;
    stem_material.shininess = 300.0;

    let mut petals = Vec::new();

    for i in 0..8 {
        let petal = Shape::sphere_from_material(petal_material).with_transform(
            Matrix::identity()
                .scale(0.2, 0.05, 0.3)
                .rotate_x(FRAC_PI_2)
                .translate(0.0, 0.5, 0.0)
                .rotate_z(i as f64 * FRAC_PI_8 * 2.0)
                .apply(&daisy_position),
        );

        petals.push(petal);
    }

    petals.push(stem);
    petals.push(center);
    petals
}

fn pond() -> Scene<'static> {
    let mut pond_material = Material::from_color(rgb(0, 191, 255));
    pond_material.diffuse = 0.9;
    pond_material.specular = 1.0;
    pond_material.shininess = 300.0;
    pond_material.transparency = 0.9;
    pond_material.reflective = 0.9;
    pond_material.refractive_index = 1.5;
    let pond = Shape::sphere_from_material(pond_material).with_transform(
        Matrix::identity()
            .scale(2.5, 0.03, 1.4)
            .translate(0.0, 0.0, -0.8),
    );

    vec![pond]
}

pub fn maggies_world() -> Scene<'static> {
    let mut grass_material = Material::from_color(rgb(124, 252, 0));
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

    let mut v = vec![sky, grass];
    v.extend(daisy());
    v.extend(pond());
    v
}
