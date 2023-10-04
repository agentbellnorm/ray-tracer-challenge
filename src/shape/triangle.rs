#[cfg(test)]
mod triangle_test {
    use crate::{
        shape::{Shape, ShapeType},
        tuple::{point_i, vector_i, point}, world::World,
    };

    #[test]
    fn constructing_a_triangle() {

        let triangle = Shape::triangle(
            point_i(0, 1, 0),
            point_i(-1, 0, 0),
            point_i(1, 0, 0)
        );

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
            point_i(1, 0, 0)
        ));

        let triangle_shape = world.get_shape(triangle);

        let n1 = triangle_shape.normal_at(&world, point(0.0, 0.5, 0.0));
        let n2 = triangle_shape.normal_at(&world, point(-0.5, 0.75, 0.0));
        let n3 = triangle_shape.normal_at(&world, point(0.5, 0.25, 0.0));

        let triangle_normal = match triangle_shape.shape_type {
            ShapeType::Triangle(_, _, _, _, _, normal) => normal,
            _ => panic!("wtf")
        };

        assert_eq!(triangle_normal, n1);
        assert_eq!(triangle_normal, n2);
        assert_eq!(triangle_normal, n3);
    }
}

