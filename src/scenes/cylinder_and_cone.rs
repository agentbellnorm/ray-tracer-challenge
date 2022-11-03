use crate::shape::Scene;
use crate::{rgb, white, Material, Matrix, Pattern, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn cylinder_and_cone_scene() -> Scene {
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
            white(),
            rgb(33, 149, 77),
        )))
        .with_transform(Matrix::identity().translate(0.0, 0.0, 10.0)),
        Shape::cylinder(0.0, 2.0, false)
            .with_material(Material::glass())
            .with_transform(Matrix::identity().translate(1.0, 0.0, 3.0)),
        Shape::cone(-1.0, 0.0, false)
            .with_material(Material::glass())
            .with_transform(Matrix::identity().translate(1.0, 3.0, 3.0)),
        Shape::cylinder(0.0, 1.0, true)
            .with_material(Material::chrome())
            .with_transform(
                Matrix::identity()
                    .scale(0.8, 1.0, 0.8)
                    .translate(-3.0, 0.0, 3.0),
            ),
        Shape::cylinder(0.0, 3.0, false)
            .with_material(Material::pastel(rgb(179, 217, 170)))
            .with_transform(
                Matrix::identity()
                    .scale(0.4, 0.4, 0.4)
                    .rotate_x(FRAC_PI_2)
                    .translate(-1.5, 0.4, 0.0),
            ),
        Shape::cylinder(0.0, 1.0, true)
            .with_material(Material::pastel(rgb(149, 52, 47)))
            .with_transform(
                Matrix::identity()
                    .rotate_z(FRAC_PI_2)
                    .scale(3.0, 0.5, 0.5)
                    .rotate_y(-0.6)
                    .translate(1.0, 0.5, 6.0),
            ),
    ]
}
