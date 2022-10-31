use crate::shape::Scene;
use crate::{color, Material, Matrix, Pattern, Shape};
use std::f64::consts::FRAC_PI_2;

// let world = World::with(
//     debug_glass(),
//     PointLight::with(point_i(2, 10, -5), white() * 0.9),
// );

// let camera = Camera::new(1280, 720, 11.0 * PI / 18.0).set_transform(view_transformation(
// let camera = Camera::new(600, 600, 0.45).set_transform(view_transformation(
//     point_i(0, 0, -5),
//     point_i(0, 0, 0),
//     vector_i(0, 1, 0),
// ));

pub fn debug_glass() -> Scene<'static> {
    vec![
        Shape::plane_from_material(Material::from_pattern(Pattern::checkers(
            color(0.15, 0.15, 0.15),
            color(0.85, 0.85, 0.85),
        )))
        .with_transform(
            Matrix::identity()
                .rotate_x(FRAC_PI_2)
                .translate(0.0, 0.0, 10.0),
        ),
        Shape::sphere_glass(),
        Shape::sphere_from_material(Material::air())
            .with_transform(Matrix::identity().scale(0.5, 0.5, 0.5)),
    ]
}
