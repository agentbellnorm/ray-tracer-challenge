use crate::{
    intersection::{Intersection, Intersections},
    rays::Ray,
    tuple::{Tuple, EPSILON},
};

#[cfg(test)]
mod triangle_test {
    use crate::{
        rays::Ray,
        shape::{triangle::triangle_intersect, Shape, ShapeType},
        tuple::{point, point_i, vector_i},
        world::World,
    };

    #[test]
    fn constructing_a_triangle() {
        let triangle = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let (e1, e2, normal) = match triangle.shape_type {
            ShapeType::Triangle(_, _, _, e1, e2, normal) => (e1, e2, normal),
            _ => panic!("wtf"),
        };

        assert_eq!(e1, vector_i(-1, -1, 0));
        assert_eq!(e2, vector_i(1, -1, 0));
        assert_eq!(normal, vector_i(0, 0, -1));
    }

    #[test]
    fn finding_normal_on_triangle() {
        let mut world = World::default();
        let triangle = world.add_shape(Shape::triangle(
            point_i(0, 1, 0),
            point_i(-1, 0, 0),
            point_i(1, 0, 0),
        ));

        let triangle_shape = world.get_shape(triangle);

        let n1 = triangle_shape.normal_at(&world, point(0.0, 0.5, 0.0));
        let n2 = triangle_shape.normal_at(&world, point(-0.5, 0.75, 0.0));
        let n3 = triangle_shape.normal_at(&world, point(0.5, 0.25, 0.0));

        let triangle_normal = match triangle_shape.shape_type {
            ShapeType::Triangle(_, _, _, _, _, normal) => normal,
            _ => panic!("wtf"),
        };

        assert_eq!(triangle_normal, n1);
        assert_eq!(triangle_normal, n2);
        assert_eq!(triangle_normal, n3);
    }

    #[test]
    fn intersecting_ray_parallel_to_triangle() {
        let triangle = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let ray = Ray::with(point_i(0, -1, -2), vector_i(0, 1, 0));

        let (p1, e1, e2) = match &triangle.shape_type {
            ShapeType::Triangle(p1, _, _, e1, e2, _) => (p1, e1, e2),
            _ => panic!("wtf"),
        };

        assert!(triangle_intersect(p1, e1, e2, &ray, 0).is_empty());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let triangle = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let ray = Ray::with(point_i(1, 1, -2), vector_i(0, 0, 1));

        let (p1, e1, e2) = match &triangle.shape_type {
            ShapeType::Triangle(p1, _, _, e1, e2, _) => (p1, e1, e2),
            _ => panic!("wtf"),
        };

        assert!(triangle_intersect(p1, e1, e2, &ray, 0).is_empty());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let triangle = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let ray = Ray::with(point_i(-1, 1, -2), vector_i(0, 0, 1));

        let (p1, e1, e2) = match &triangle.shape_type {
            ShapeType::Triangle(p1, _, _, e1, e2, _) => (p1, e1, e2),
            _ => panic!("wtf"),
        };

        assert!(triangle_intersect(p1, e1, e2, &ray, 0).is_empty());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let triangle = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let ray = Ray::with(point_i(0, -1, -2), vector_i(0, 0, 1));

        let (p1, e1, e2) = match &triangle.shape_type {
            ShapeType::Triangle(p1, _, _, e1, e2, _) => (p1, e1, e2),
            _ => panic!("wtf"),
        };

        assert!(triangle_intersect(p1, e1, e2, &ray, 0).is_empty());
    }

    #[test]
    fn ray_strikes_triangle() {
        let triangle = Shape::triangle(point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let ray = Ray::with(point(0.0, 0.5, -2.0), vector_i(0, 0, 1));

        let (p1, e1, e2) = match &triangle.shape_type {
            ShapeType::Triangle(p1, _, _, e1, e2, _) => (p1, e1, e2),
            _ => panic!("wtf"),
        };

        assert_eq!(
            triangle_intersect(p1, e1, e2, &ray, 0)
                .xs
                .first()
                .unwrap()
                .t,
            2.0
        );
    }
}

pub fn triangle_intersect(
    p1: &Tuple,
    e1: &Tuple,
    e2: &Tuple,
    ray: &Ray,
    shape_id: usize,
) -> Intersections {
    let dir_cross_e2 = ray.direction.cross(e2);
    let det = e1.dot(&dir_cross_e2);
    if det.abs() < EPSILON {
        return Intersections::empty();
    }

    let f = 1.0 / det;

    let p1_to_origin = &ray.origin - p1;
    let u = f * p1_to_origin.dot(&dir_cross_e2);
    if u < 0.0 || u > 1.0 {
        return Intersections::empty();
    }

    let origin_cross_el = p1_to_origin.cross(e1);
    let v = f * ray.direction.dot(&origin_cross_el);
    if v < 0.0 || (u + v) > 1.0 {
        return Intersections::empty();
    }

    Intersections::from(vec![Intersection::with_u_and_v(
        f * e2.dot(&origin_cross_el),
        shape_id,
        u,
        v,
    )])
}
