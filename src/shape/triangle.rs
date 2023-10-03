#[cfg(test)]
mod triangle_test {
    use crate::{
        shape::{Shape, ShapeType},
        tuple::{point_i, vector_i},
    };

    #[test]
    fn constructing_a_triangle() {
        let (p1, p2, p3) = (point_i(0, 1, 0), point_i(-1, 0, 0), point_i(1, 0, 0));

        let triangle = Shape::triangle(p1, p2, p3);

        let (e1, e2, normal) = match triangle.shape_type {
            ShapeType::Triangle(_, _, _, e1, e2, normal) => (e1, e2, normal),
            _ => panic!("wtf"),
        };

        assert_eq!(e1, vector_i(-1, -1, 0));
        assert_eq!(e2, vector_i(1, -1, 0));
        assert_eq!(normal, vector_i(0, 0, -1));
    }
}

