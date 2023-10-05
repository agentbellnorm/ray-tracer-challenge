use crate::{
    shape::Shape,
    tuple::{point, point_i, Tuple},
    world::World,
};

#[cfg(test)]
mod obj_file_test {
    use crate::{
        obj_file::parse_obj,
        shape::{Shape, ShapeType},
        tuple::{point, point_i, Tuple},
        world::World,
    };

    use super::add_obj_file;

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
        assert!(result.groups.is_empty());
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
        let t1 = &result.groups.first().unwrap().items[0];
        let t2 = &result.groups.first().unwrap().items[1];
        let (t1p1, t1p2, t1p3) = get_points(t1);
        let (t2p1, t2p2, t2p3) = get_points(t2);

        assert_eq!(t1p1, result.vertices[1]);
        assert_eq!(t1p2, result.vertices[2]);
        assert_eq!(t1p3, result.vertices[3]);

        assert_eq!(t2p1, result.vertices[1]);
        assert_eq!(t2p2, result.vertices[3]);
        assert_eq!(t2p3, result.vertices[4]);
    }

    #[test]
    fn triangulating_polygons() {
        let file = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            v 1 1 0
            v 0 2 0

            f 1 2 3 4 5
            ";

        let result = parse_obj(file);
        assert_eq!(result.vertices.len(), 5 + 1);
        assert_eq!(result.groups.first().unwrap().items.len(), 3);
        let t1 = &result.groups.first().unwrap().items[0];
        let t2 = &result.groups.first().unwrap().items[1];
        let t3 = &result.groups.first().unwrap().items[2];
        let (t1p1, t1p2, t1p3) = get_points(t1);
        let (t2p1, t2p2, t2p3) = get_points(t2);
        let (t3p1, t3p2, t3p3) = get_points(t3);

        assert_eq!(t1p1, result.vertices[1]);
        assert_eq!(t1p2, result.vertices[2]);
        assert_eq!(t1p3, result.vertices[3]);

        assert_eq!(t2p1, result.vertices[1]);
        assert_eq!(t2p2, result.vertices[3]);
        assert_eq!(t2p3, result.vertices[4]);

        assert_eq!(t3p1, result.vertices[1]);
        assert_eq!(t3p2, result.vertices[4]);
        assert_eq!(t3p3, result.vertices[5]);
    }

    const TRIANGLES_OBJ: &str = "
        v -1 1 0
        v -1 0 0
        v 1 0 0 
        v 1 1 0

        g FirstGroup
        f 1 2 3
        g SecondGroup
        f 1 3 4
        ";

    #[test]
    fn triangles_in_groups() {
        let result = parse_obj(TRIANGLES_OBJ);
        assert_eq!(result.vertices.len(), 4 + 1);
        assert_eq!(result.groups.len(), 2);
        let g1 = &result.groups.get(0).unwrap();
        let g2 = &result.groups.get(1).unwrap();
        let t1 = g1.items.get(0).unwrap();
        let t2 = g2.items.get(0).unwrap();
        let (t1p1, t1p2, t1p3) = get_points(t1);
        let (t2p1, t2p2, t2p3) = get_points(t2);

        assert_eq!(g1.name, "FirstGroup");
        assert_eq!(g2.name, "SecondGroup");

        assert_eq!(t1p1, result.vertices[1]);
        assert_eq!(t1p2, result.vertices[2]);
        assert_eq!(t1p3, result.vertices[3]);

        assert_eq!(t2p1, result.vertices[1]);
        assert_eq!(t2p2, result.vertices[3]);
        assert_eq!(t2p3, result.vertices[4]);
    }


    #[test]
    fn converting_obj_file_to_group() {
        let mut world = World::default();

        add_obj_file(&mut world, TRIANGLES_OBJ);

        let mut sub_group_ids = world.get_children(0).into_iter();

        let g1 = world.get_shape(sub_group_ids.next().unwrap());
        let g2 = world.get_shape(sub_group_ids.next().unwrap());

        let g1_children = world.get_children(g1.id.unwrap());
        let g2_children = world.get_children(g2.id.unwrap());

        assert_eq!(g1_children.len(), 1);
        assert_eq!(g2_children.len(), 1);
    }
}

struct TriangleGroup {
    pub name: String,
    pub items: Vec<Shape>,
}

impl TriangleGroup {
    pub fn default() -> TriangleGroup {
        TriangleGroup {
            name: "Default".to_owned(),
            items: vec![],
        }
    }

    pub fn with_name(name: &str) -> TriangleGroup {
        TriangleGroup {
            name: name.to_owned(),
            items: vec![],
        }
    }
}

type Groups = Vec<TriangleGroup>;

struct ParsedObj {
    pub vertices: Vec<Tuple>,
    pub groups: Groups,
}

pub fn add_obj_file(world: &mut World, content: &str) -> () {
    let parse_result = parse_obj(content);

    let root_group = world.add_shape(Shape::group());

    for group in parse_result.groups {
        let group_id = world.add_shape(Shape::group());
        for triangle in group.items {
            let triangle_id = world.add_shape(triangle);
            world.add_shape_to_group(group_id, triangle_id);
        }

        world.add_shape_to_group(root_group, group_id);
    }

    println!("parsed {} vertices", parse_result.vertices.len());
}

fn parse_obj(content: &str) -> ParsedObj {
    let mut vertices = vec![point_i(6, 6, 6)]; // bogus point to make it 1 indexed
    let mut groups = vec![];

    for line in content.lines().map(&str::trim) {
        if line.starts_with("v ") {
            vertices.push(parse_vertex(line));
        }

        if line.starts_with("f ") {
            if groups.is_empty() {
                groups.push(TriangleGroup::default())
            }

            groups
                .last_mut()
                .unwrap()
                .items
                .append(&mut fan_triangulation(&vertices, parse_vertex_ids(line)))
        }

        if line.starts_with("g ") {
            let name = line.split(" ").skip(1).next().unwrap();
            groups.push(TriangleGroup::with_name(name))
        }
    }

    ParsedObj { vertices, groups }
}

fn fan_triangulation(vertices: &Vec<Tuple>, vertex_ids: Vec<usize>) -> Vec<Shape> {
    let mut triangles = vec![];

    for index in 1..(vertex_ids.len() - 1) {
        triangles.push(Shape::triangle(
            vertices[vertex_ids[0]],
            vertices[vertex_ids[index]],
            vertices[vertex_ids[index + 1]],
        ))
    }

    triangles
}

fn parse_vertex(line: &str) -> Tuple {
    let mut iter = line.split(" ").skip(1).map(parse_float);
    point(
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

fn parse_vertex_ids(line: &str) -> Vec<usize> {
    line.split(" ")
        .skip(1)
        .map(parse_integer)
        .collect::<Vec<usize>>()
}

fn parse_integer(s: &str) -> usize {
    if let Ok(i) = s.parse::<usize>() {
        return i;
    }

    panic!("Could not parse {} to i32", s);
}

fn parse_float(s: &str) -> f64 {
    if let Ok(f) = s.parse::<f64>() {
        return f;
    }

    panic!("Could not parse {} to f64", s)
}

fn int_big(string: &str) -> i64 {
    match string.parse::<i64>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i64", string),
    }
}
