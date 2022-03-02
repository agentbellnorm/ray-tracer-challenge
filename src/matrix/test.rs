#[cfg(test)]
mod matrix_test {
    use crate::matrix::{is_equal_float, Matrix};
    use crate::tuple::Tuple;

    #[test]
    fn create_matrix() {
        let matrix = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 13.5, 15.5, 16.5],
        ]);

        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 3), 4.0);
        assert_eq!(matrix.get(1, 0), 5.5);
        assert_eq!(matrix.get(1, 2), 7.5);
        assert_eq!(matrix.get(2, 2), 11.0);
        assert_eq!(matrix.get(3, 0), 13.5);
        assert_eq!(matrix.get(3, 2), 15.5);
    }

    #[test]
    fn create_2x2() {
        let matrix = Matrix::from_values(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn create_3x3() {
        let matrix = Matrix::from_values(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(1, 1), -2.0);
        assert_eq!(matrix.get(2, 2), 1.0);
    }

    #[test]
    fn equality() {
        let a = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let c = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0001, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b);

        assert_ne!(b, c);
    }

    #[test]
    fn multiplication_with_self() {
        let a = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from_values(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            &a * &b,
            Matrix::from_values(vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ])
        )
    }

    #[test]
    fn multiplication_with_tuple() {
        let matrix = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let tuple = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert_eq!(
            tuple * &matrix,
            Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0,
            }
        )
    }

    #[test]
    fn identity() {
        let matrix = Matrix::from_values(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 6.0, 16.0, 32.0],
        ]);

        assert_eq!(
            &matrix * &Matrix::identity(),
            Matrix::from_values(vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 4.0, 8.0],
                vec![2.0, 4.0, 8.0, 16.0],
                vec![4.0, 6.0, 16.0, 32.0],
            ])
        )
    }

    #[test]
    fn transpose() {
        let matrix = Matrix::from_values(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);

        // println!("{:?}", matrix.transpose());

        assert_eq!(
            matrix.transpose(),
            Matrix::from_values(vec![
                vec![0.0, 9.0, 1.0, 0.0],
                vec![9.0, 8.0, 8.0, 0.0],
                vec![3.0, 0.0, 5.0, 5.0],
                vec![0.0, 8.0, 3.0, 8.0],
            ])
        );
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix::identity(), Matrix::identity().transpose());
    }

    #[test]
    fn determinant_2x2() {
        assert_eq!(
            Matrix::from_values(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]).determinant(),
            17.0
        )
    }

    #[test]
    fn submatrix_2x2() {
        let super_matrix = Matrix::from_values(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, 3.0],
        ]);

        assert_eq!(
            super_matrix.submatrix(0, 2),
            Matrix::from_values(vec![vec![-3.0, 2.0], vec![0.0, 6.0]])
        )
    }

    #[test]
    fn submatrix_4x4() {
        let super_matrix = Matrix::from_values(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(
            super_matrix.submatrix(2, 1),
            Matrix::from_values(vec![
                vec![-6.0, 1.0, 6.0],
                vec![-8.0, 8.0, 6.0],
                vec![-7.0, -1.0, 1.0],
            ])
        )
    }

    #[test]
    fn minor_3x3() {
        let a = Matrix::from_values(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![-6.0, -1.0, 5.0],
        ]);

        let b = a.submatrix(1, 0);

        assert_eq!(b.determinant(), 25.0);

        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3() {
        let a = Matrix::from_values(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![-6.0, -1.0, 5.0],
        ]);

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);

        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3() {
        let a = Matrix::from_values(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn determinant_4x4() {
        let a = Matrix::from_values(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn invertibility_invertible() {
        let a = Matrix::from_values(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);

        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }
    #[test]
    fn invertibility_not_invertible() {
        let a = Matrix::from_values(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse() {
        let a = Matrix::from_values(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);

        let b = a.inverse();

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert!(is_equal_float(b.get(3, 2), -160.0 / 532.0));
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert!(is_equal_float(b.get(2, 3), 105.0 / 532.0));

        assert_eq!(
            b,
            Matrix::from_values(vec![
                vec![0.21805, 0.45113, 0.24060, -0.04511],
                vec![-0.80827, -1.45677, -0.44361, 0.52068],
                vec![-0.07895, -0.22368, -0.05263, 0.19737],
                vec![-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn inverse_again() {
        let a = Matrix::from_values(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);

        assert_eq!(
            a.inverse(),
            Matrix::from_values(vec![
                vec![-0.15385, -0.15385, -0.28205, -0.53846],
                vec![-0.07692, 0.12308, 0.02564, 0.03077],
                vec![0.35897, 0.35897, 0.43590, 0.92308],
                vec![-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn inverse_again_and_again() {
        let a = Matrix::from_values(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);

        assert_eq!(
            a.inverse(),
            Matrix::from_values(vec![
                vec![-0.04074, -0.07778, 0.14444, -0.22222],
                vec![-0.07778, 0.03333, 0.36667, -0.33333],
                vec![-0.02901, -0.14630, -0.10926, 0.12963],
                vec![0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }

    #[test]
    fn multiplying_with_inverse() {
        let a = Matrix::from_values(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix::from_values(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);

        let c = &a * &b;

        assert_eq!(&c * &b.inverse(), a);
    }
}

#[cfg(test)]
mod translation {
    use crate::canvas::Canvas;
    use crate::color::color;
    use crate::io::save_to_file;
    use crate::matrix::Matrix;
    use crate::tuple::{point, vector};
    use std::f32::consts::PI;

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = Matrix::identity().translate(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(p * &transform, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_with_inverse_of_translation_matrix() {
        let transform = Matrix::identity().translate(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(p * &inv, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::identity().translate(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(v * &transform, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let scaling = Matrix::identity().scale(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);

        assert_eq!(p * &scaling, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let scaling = Matrix::identity().scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(v * &scaling, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_with_invers_of_scaling_matrix() {
        let s = Matrix::identity().scale(2.0, 3.0, 4.0);
        let inv_s = s.inverse();
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(v * &inv_s, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_by_scaling_with_negative() {
        let s = Matrix::identity().scale(-1.0, 1.0, 1.0);
        let p = vector(2.0, 3.0, 4.0);

        assert_eq!(p * &s, vector(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::identity().rotate_x(PI / 4.0);
        let full_quarter = Matrix::identity().rotate_x(PI / 2.0);

        assert_eq!(
            p * &half_quarter,
            point(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(p * &full_quarter, point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::identity().rotate_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert_eq!(
            p * &inv,
            point(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0)
        )
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::identity().rotate_y(PI / 4.0);
        let full_quarter = Matrix::identity().rotate_y(PI / 2.0);

        assert_eq!(
            p * &half_quarter,
            point(f32::sqrt(2.0) / 2.0, 0.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(p * &full_quarter, point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::identity().rotate_z(PI / 4.0);
        let full_quarter = Matrix::identity().rotate_z(PI / 2.0);

        assert_eq!(
            p * &half_quarter,
            point(-f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(p * &full_quarter, point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_x_y() {
        let transform = Matrix::identity().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(p * &transform, point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_x_z() {
        let transform = Matrix::identity().shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(p * &transform, point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_y_x() {
        let transform = Matrix::identity().shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(p * &transform, point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_y_z() {
        let transform = Matrix::identity().shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(p * &transform, point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_z_x() {
        let transform = Matrix::identity().shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(p * &transform, point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_z_y() {
        let transform = Matrix::identity().shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(p * &transform, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = point(1.0, 0.0, 1.0);

        let transform = Matrix::identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(p * &transform, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn draw_clock() {
        let mut canvas = Canvas::new(200, 200, color(0.0, 0.0, 0.0));

        for i in 0..12 {
            let p = point(0.0, 0.0, 0.0);
            let t = Matrix::identity()
                .translate(0.0, 1.0, 0.0)
                .rotate_z(i as f32 * ((2.0 * PI) / 12.0))
                .scale(75.0, 75.0, 75.0)
                .translate(100.0, 100.0, 0.0);

            let time = p * &t;

            canvas = canvas.write_pixel(
                time.x.round() as i32,
                time.y.round() as i32,
                color(255.0, 255.0, 255.0),
            );
        }

        let res = save_to_file("src/matrix/klocka.ppm", canvas.to_ppm());

        assert!(res.is_ok());
    }
}
