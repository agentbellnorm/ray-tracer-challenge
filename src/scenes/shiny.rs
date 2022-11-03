use crate::{rgb, white, Material, Matrix, Pattern, Shape};
use std::f64::consts::FRAC_PI_2;

use crate::shape::Scene;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const SEED: u64 = 89443326544;

const PASTELS: [[i32; 3]; 6] = [
    [49, 191, 243],
    [164, 132, 233],
    [244, 136, 154],
    [255, 175, 104],
    [246, 230, 131],
    [121, 212, 94],
];

const BILLIARDS: [[i32; 3]; 8] = [
    [0, 0, 0],
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    [255, 0, 255],
    [255, 255, 0],
    [0, 255, 255],
    [165, 42, 42],
];

fn glass_balls() -> Scene {
    let size = 1.5;
    let ball = Shape::sphere_glass().with_transform(
        Matrix::identity()
            .scale(size, size, size)
            .translate(1.3, size, 6.0),
    );

    vec![ball]
}

fn chrome_balls() -> Scene {
    let size = 1.0;
    let ball = Shape::sphere_chrome().with_transform(
        Matrix::identity()
            .scale(size, size, size)
            .translate(-2.8, size, 4.0),
    );

    vec![ball]
}

fn billiard_balls() -> Scene {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut v = vec![];
    for c in BILLIARDS {
        let mut material = Material::from_color(rgb(c[0], c[1], c[2]));
        material.shininess = 300.0;
        material.specular = 1.0;
        material.diffuse = 0.3;
        material.reflective = 0.1;
        material.ambient = 0.3;

        let ball_size = 0.6;
        let ball = Shape::sphere_from_material(material).with_transform(
            Matrix::identity()
                .scale(ball_size, ball_size, ball_size)
                .translate(
                    rng.gen_range(-50..50) as f64 / 10.0,
                    ball_size,
                    rng.gen_range(-40..=100) as f64 / 10.0,
                ),
        );
        v.push(ball)
    }

    v
}

fn pastel_balls() -> Scene {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut v = vec![];
    for _ in 0..100 {
        let rnd_c = PASTELS[rng.gen_range(0..6)];
        let mut material = Material::from_color(rgb(rnd_c[0], rnd_c[1], rnd_c[2]));
        material.shininess = 1.0;
        material.specular = 0.0;
        material.ambient = 0.3;

        let ball_size = 0.4;
        let ball = Shape::sphere_from_material(material).with_transform(
            Matrix::identity()
                .scale(ball_size, ball_size, ball_size)
                .translate(
                    rng.gen_range(-150..=150) as f64 / 10.0,
                    ball_size,
                    rng.gen_range(-40..=400) as f64 / 10.0,
                ),
        );
        v.push(ball)
    }

    v
}

pub fn shiny_scene() -> Scene {
    let mut ground_material = Material::from_color(white());
    ground_material.diffuse = 0.9;
    ground_material.specular = 0.1;
    ground_material.ambient = 0.5;
    let ground = Shape::plane_from_material(ground_material);

    let mut sky_material = Material::from_pattern(
        Pattern::gradient(rgb(135, 206, 250), white()).with_transformation(Matrix::identity()),
    );
    sky_material.diffuse = 0.9;
    sky_material.specular = 0.01;
    let sky = Shape::plane_from_material(sky_material).with_transform(
        Matrix::identity()
            .scale(35.0, 1.0, 1.0)
            .rotate_x(FRAC_PI_2)
            .rotate_z(FRAC_PI_2)
            .translate(0.0, 0.0, 60.0),
    );

    let mut objects = vec![ground, sky];
    objects.extend(glass_balls());
    objects.extend(pastel_balls());
    objects.extend(chrome_balls());
    objects.extend(billiard_balls());
    objects
}
