#[cfg(test)]
mod tuple_test {
    use crate::canvas::Canvas;
    use crate::color::{color, Color};
    use crate::io::save_to_file;
    use crate::tuple::{is_point, is_vector, point, vector, Tuple};

    #[test]
    pub fn create() {
        let v = vector(4.3, -4.2, 3.1);
        let p = point(4.3, -4.2, 3.1);

        assert!(is_vector(&v));
        assert!(v.is_vector());
        assert!(is_point(&p));
        assert!(p.is_point());
        assert!(!is_vector(&p));
        assert!(!is_point(&v));
    }

    #[test]
    pub fn equality() {
        assert_eq!(
            point(1.11111, 2.222222, 3.333333),
            point(1.11111, 2.222222, 3.333333)
        );
        assert_eq!(
            vector(1.11111, 2.222222, 3.333333),
            vector(1.11111, 2.222222, 3.333333)
        );

        assert_ne!(
            vector(1.11112, 2.222223, 3.333334),
            vector(1.11111, 2.222222, 3.333333)
        );
        assert_ne!(
            point(1.11112, 2.222223, 3.333334),
            point(1.11111, 2.222222, 3.333333)
        );

        assert_ne!(
            point(1.11111, 2.222222, 3.333333),
            vector(1.11111, 2.222222, 3.333333)
        );
    }

    #[test]
    pub fn addition() {
        assert_eq!(
            point(3.0, -2.0, 5.0) + vector(-2.0, 3.0, 1.0),
            point(1.0, 1.0, 6.0)
        )
    }

    #[test]
    pub fn subtraction() {
        assert_eq!(
            point(3.0, 2.0, 1.0) - vector(5.0, 6.0, 7.0),
            point(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            vector(3.0, 2.0, 1.0) - vector(5.0, 6.0, 7.0),
            vector(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    pub fn negation() {
        assert_eq!(
            -Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            },
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    #[test]
    pub fn multiplication() {
        assert_eq!(
            Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            } * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        );
        assert_eq!(
            Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            } * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    pub fn division() {
        assert_eq!(
            Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            } / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    pub fn magnitude() {
        assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    pub fn normalize() {
        assert_eq!(vector(4.0, 0.0, 0.0).normalize(), vector(1.0, 0.0, 0.0));
        assert_eq!(
            vector(1.0, 2.0, 3.0).normalize(),
            vector(
                1.0 / 14.0_f32.sqrt(),
                2.0 / 14.0_f32.sqrt(),
                3.0 / 14.0_f32.sqrt()
            )
        )
    }

    #[test]
    pub fn dot() {
        assert_eq!(vector(1.0, 2.0, 3.0).dot(&vector(2.0, 3.0, 4.0)), 20.0)
    }

    #[test]
    pub fn cross() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
    }

    struct Projectile {
        position: Tuple,
        velocity: Tuple,
    }

    struct Environment {
        gravity: Tuple,
        wind: Tuple,
    }

    fn tick(mut projectile: Projectile, environment: &Environment) -> Projectile {
        projectile.position = projectile.position + projectile.velocity;
        projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
        projectile
    }

    #[test]
    pub fn cannon() {
        let mut projectile = Projectile {
            position: point(0.0, 1.0, 0.0),
            velocity: vector(1.0, 1.0, 0.0).normalize() * 3.0,
        };

        let environment = Environment {
            gravity: vector(0.0, -0.1, 0.0),
            wind: vector(-0.01, 0.0, 0.0),
        };

        while projectile.position.y >= 0.0 {
            // println!("position is {:?}", projectile.position);
            projectile = tick(projectile, &environment);
        }

        let expected_distance = 83.878105;

        assert_eq!(projectile.position.x, expected_distance)
    }

    #[test]
    fn fire_to_file() {
        let mut projectile = Projectile {
            position: point(0.0, 1.0, 0.0),
            velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25,
        };

        let environment = Environment {
            gravity: vector(0.0, -0.1, 0.0),
            wind: vector(-0.01, 0.0, 0.0),
        };

        let mut canvas = Canvas::new(900, 550, Color::black());

        let red = color(1.0, 0.0, 0.0);

        while projectile.position.y >= 0.0 {
            // println!("position is {:?}", projectile.position);
            canvas = canvas.write_pixel(
                projectile.position.x.round() as i32,
                550 - projectile.position.y.round() as i32,
                red,
            );
            projectile = tick(projectile, &environment);
        }

        let res = save_to_file("src/tuple/kanon.ppm", canvas.to_ppm());

        assert!(res.is_ok());
    }

    #[test]
    fn reflecting_vector_approaching_at_45_deg() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);

        assert_eq!(v.reflect(&n), vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0);

        assert_eq!(v.reflect(&n), vector(1.0, 0.0, 0.0));
    }
}
