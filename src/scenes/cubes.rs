use crate::scenes::Scene;
use crate::{color, rgb, Material, Matrix, Pattern, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn cubes_scene() -> Scene {
    vec![
        Shape::plane_from_material(Material::from_pattern(Pattern::ring(
            rgb(107, 144, 149),
            rgb(185, 215, 233),
        )))
        .with_transform(
            Matrix::identity()
                .rotate_x(FRAC_PI_2)
                .translate(0.0, 0.0, 10.0),
        ),
        Shape::plane_from_material(Material::from_pattern(Pattern::checkers(
            color(0.0, 0.0, 0.0),
            color(1.0, 1.0, 1.0),
        )))
        .with_transform(Matrix::identity().translate(0.0, 0.0, 10.0)),
        Shape::cube_default()
            .with_material(Material::glass())
            .with_transform(Matrix::identity().translate(1.0, 1.0, 3.0).rotate_y(0.5)),
        Shape::cube_default()
            .with_material(Material::chrome())
            .with_transform(
                Matrix::identity()
                    .scale(0.8, 0.8, 0.8)
                    .translate(-3.0, 0.8, 3.0),
            ),
        Shape::cube_default()
            .with_material(Material::pastel(rgb(179, 217, 170)))
            .with_transform(
                Matrix::identity()
                    .rotate_y(-0.6)
                    .scale(0.3, 0.3, 0.3)
                    .translate(-2.5, 0.3, 1.0),
            ),
        Shape::cube_default()
            .with_material(Material::pastel(rgb(149, 52, 47)))
            .with_transform(
                Matrix::identity()
                    .scale(3.0, 0.5, 0.5)
                    .rotate_y(-0.6)
                    .translate(1.0, 0.5, 6.0),
            ),
    ]
}
