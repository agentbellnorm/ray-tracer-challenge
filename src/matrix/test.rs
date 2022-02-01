#[cfg(test)]
mod matrix_test {
    use crate::matrix::Matrix;
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
    fn create_two_by_two() {
        let matrix = Matrix::from_values(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn create_three_by_three() {
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
            a * b,
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
            tuple * matrix,
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
            matrix * Matrix::identity(),
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
    fn determinant() {
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
}
