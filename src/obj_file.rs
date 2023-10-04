use crate::{
    shape::Shape,
    tuple::{point, point_i, Tuple},
};

#[cfg(test)]
mod obj_file_test {
    use crate::{
        obj_file::parse_obj,
        shape::{Shape, ShapeType},
        tuple::{point, point_i, Tuple},
    };

    #[test]
    fn ignoring_unregognized_lines() {
        let gibberish = "
            Therre was a young lady named Bright
            who traveled much faster than light.
            She set out one day
            in a relative way,
            and came back the previous night.

            ";

        let result = parse_obj(gibberish);
        assert!(result.triangles.is_empty());
        assert_eq!(result.vertices.len(), 1);
    }

    #[test]
    fn vertex_records() {
        let file = "
            v -1 1 0
            v -1.0000 0.5000 0.0000
            v 1 0 0
            v 1 1 0
            ";

        let result = parse_obj(file);
        assert_eq!(result.vertices[1], point_i(-1, 1, 0));
        assert_eq!(result.vertices[2], point(-1.0, 0.5, 0.0));
        assert_eq!(result.vertices[3], point_i(1, 0, 0));
        assert_eq!(result.vertices[4], point_i(1, 1, 0));
    }

    fn get_points(triangle: &Shape) -> (Tuple, Tuple, Tuple) {
        if let ShapeType::Triangle(p1, p2, p3, _, _, _) = triangle.shape_type {
            return (p1, p2, p3);
        }
        panic!("{:?} was not a triangle", triangle);
    }

    #[test]
    fn parsing_triangle_faces() {
        let file = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            v 1 1 0

            f 1 2 3
            f 1 3 4
            ";

        let result = parse_obj(file);
        let t1 = &result.triangles[0];
        let t2 = &result.triangles[1];
        let (t1p1, t1p2, t1p3) = get_points(t1);
        let (t2p1, t2p2, t2p3) = get_points(t2);

        assert_eq!(t1p1, result.vertices[1]);
        assert_eq!(t1p2, result.vertices[2]);
        assert_eq!(t1p3, result.vertices[3]);

        assert_eq!(t2p1, result.vertices[1]);
        assert_eq!(t2p2, result.vertices[3]);
        assert_eq!(t2p3, result.vertices[4]);
    }
}

pub struct ParsedObj {
    pub vertices: Vec<Tuple>,
    pub triangles: Vec<Shape>,
}

pub fn parse_obj(content: &str) -> ParsedObj {
    let mut vertices = vec![point_i(6, 6, 6)]; // bogus point to make it 1 indexed
    let mut triangles = vec![];

    for line in content.lines().map(&str::trim) {
        if line.starts_with("v ") {
            vertices.push(parse_vertex(line));
        }

        if line.starts_with("f ") {
            let ids = parse_vertex_ids(line);
            triangles.push(Shape::triangle(
                vertices[ids[0]],
                vertices[ids[1]],
                vertices[ids[2]],
            ));
        }
    }

    ParsedObj {
        vertices,
        triangles,
    }
}

pub fn parse_vertex(line: &str) -> Tuple {
    let mut iter = line.split(" ").skip(1).map(parse_float);
    point(
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

pub fn parse_vertex_ids(line: &str) -> Vec<usize> {
    line.split(" ")
        .skip(1)
        .map(parse_integer)
        .collect::<Vec<usize>>()
}

pub fn parse_integer(s: &str) -> usize {
    if let Ok(i) = s.parse::<usize>() {
        return i;
    }

    panic!("Could not parse {} to i32", s);
}

pub fn parse_float(s: &str) -> f64 {
    if let Ok(f) = s.parse::<f64>() {
        return f;
    }

    panic!("Could not parse {} to f64", s)
}

pub fn int_big(string: &str) -> i64 {
    match string.parse::<i64>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i64", string),
    }
}
