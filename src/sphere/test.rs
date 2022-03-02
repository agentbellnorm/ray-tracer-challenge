#[cfg(test)]
mod sphere_test {
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};

    #[test]
    fn default_transformation() {
        let s = Sphere::unit();
        assert_eq!(s.transformation, Matrix::identity());
    }

    #[test]
    fn change_transformation() {
        let mut s = Sphere::unit();
        let t = Matrix::identity().translate(2.0, 3.0, 4.0);

        s = s.set_transform(t.clone());
        assert_eq!(s.transformation, t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::unit();

        s = s.set_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs.get(0).t, 3.0);
        assert_eq!(xs.get(1).t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::with(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::unit();

        s = s.set_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 0);
    }
}
