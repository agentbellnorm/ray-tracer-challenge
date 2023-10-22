use crate::{intersection::Intersection, tuple::Tuple};

#[cfg(test)]
mod smooth_triangle_test {
    use crate::{
        intersection::{Intersection, Intersections},
        matrix::is_equal_float,
        rays::Ray,
        shape::{Shape, ShapeType},
        tuple::{is_point, is_vector, point, point_i, vector, vector_i},
        world::World,
    };

    fn tri() -> Shape {
        let p1 = point_i(0, 1, 0);
        let p2 = point_i(-1, 0, 0);
        let p3 = point_i(1, 0, 0);
        let n1 = vector_i(0, 1, 0);
        let n2 = vector_i(-1, 0, 0);
        let n3 = vector_i(1, 0, 0);

        Shape::smooth_triangle(p1, p2, p3, n1, n2, n3)
    }

    #[test]
    fn constructing_smooth_triangle() {
        let tri = tri();
        if let ShapeType::SmoothTriangle(p1, p2, p3, e1, e2, n1, n2, n3) = &tri.shape_type {
            assert!(is_point(p1));
            assert!(is_point(p2));
            assert!(is_point(p3));
            assert!(is_vector(e1));
            assert!(is_vector(e2));
            assert!(is_vector(n1));
            assert!(is_vector(n2));
            assert!(is_vector(n3));
        } else {
            panic!("wat")
        }
    }

    #[test]
    fn an_intersection_with_u_and_v() {
        let _ = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, -0), point_i(1, 0, 0));

        let i = Intersection::with_u_and_v(3.5, 0, 0.2, 0.4);

        assert_eq!(i.u, Some(0.2));
        assert_eq!(i.v, Some(0.4));
    }

    #[test]
    fn intersection_with_smooth_triangle_stores_u_and_v() {
        let mut world = World::default();
        let tri_id = world.add_shape(tri());
        let triangle = world.get_shape(tri_id);

        let ray = Ray::with(point(-0.2, 0.3, -2.0), vector_i(0, 0, 1));

        let xs = triangle.intersects(&world, &ray);

        assert!(is_equal_float(xs.xs[0].u.unwrap(), 0.45));
        assert!(is_equal_float(xs.xs[0].v.unwrap(), 0.25));
    }

    #[test]
    fn smooth_triangle_uses_u_v_to_interpolate_normal() {
        let mut world = World::default();
        let tri_id = world.add_shape(tri());
        let triangle = world.get_shape(tri_id);

        let i = Intersection::with_u_and_v(1.0, tri_id, 0.45, 0.25);

        let n = triangle.normal_at(&world, point_i(0, 0, 0), &i);

        assert_eq!(n, vector(-0.5547, 0.83205, 0.0))
    }

    #[test]
    fn preparing_normal_on_smooth_triangle() {
        let mut world = World::default();
        let tri_id = world.add_shape(tri());

        let i = Intersection::with_u_and_v(1.0, tri_id, 0.45, 0.25);
        let r = Ray::with(point(-0.2, 0.3, -2.0), vector_i(0, 0, 1));

        let xs = Intersections::from(vec![i]);

        let comps = i.prepare_computations(&world, &r, &xs);

        assert_eq!(comps.normal_vector, vector(-0.5547, 0.83205, 0.0))
    }
}

pub fn smooth_triangle_normal_at(n1: Tuple, n2: Tuple, n3: Tuple, hit: &Intersection) -> Tuple {
    if let (Some(u), Some(v)) = (hit.u, hit.v) {
        return n2 * u + n3 * v + n1 * (1.0 - u - v);
    }

    panic!("u or v was None when calculating smooth triangle normal!");
}
