use crate::{
    intersection::{Intersection, Intersections},
    rays::Ray,
    world::World,
};

use super::{CsgType, Shape, ShapeType};

#[cfg(test)]
mod csg_test {
    use crate::{
        intersection::Intersection,
        matrix::Matrix,
        rays::Ray,
        shape::csg::{filter_intersections, intersection_allowed},
        shape::{csg::csg_intersects, CsgType, Intersections, Shape, ShapeType::CSG},
        tuple::{point_i, vector_i},
        world::World,
    };
    use rstest::rstest;

    fn get_csg(shape: &Shape) -> (CsgType, usize, usize) {
        match shape.shape_type.clone() {
            CSG(csg_type, left, right) => (csg_type, left, right),
            _ => panic!("not a csg!!"),
        }
    }

    #[test]
    fn csg_is_created_with_operation_and_two_shapes() {
        let mut world = World::default();

        let s1_id = world.add_shape(Shape::sphere_default());
        let s2_id = world.add_shape(Shape::cube_default());
        let c_id = world.create_csg(CsgType::UNION, s1_id, s2_id);

        let c = world.get_shape(c_id);
        let s1 = world.get_shape(s1_id);
        let s2 = world.get_shape(s2_id);

        let (csg_type, left, right) = get_csg(c);

        assert_eq!(csg_type, CsgType::UNION);
        assert_eq!(left, s1_id);
        assert_eq!(right, s2_id);
        assert_eq!(s1.parent, Some(c_id));
        assert_eq!(s2.parent, Some(c_id));
    }

    #[rstest]
    #[case(&CsgType::UNION, true, true, true, false)]
    #[case(&CsgType::UNION, true, true, false, true)]
    #[case(&CsgType::UNION, true, false, true, false)]
    #[case(&CsgType::UNION, true, false, false, true)]
    #[case(&CsgType::UNION, false, true, true, false)]
    #[case(&CsgType::UNION, false, true, false, false)]
    #[case(&CsgType::UNION, false, false, true, true)]
    #[case(&CsgType::UNION, false, false, false, true)]
    #[case(&CsgType::INTERSECTION, true, true, true, true)]
    #[case(&CsgType::INTERSECTION, true, true, false, false)]
    #[case(&CsgType::INTERSECTION, true, false, true, true)]
    #[case(&CsgType::INTERSECTION, true, false, false, false)]
    #[case(&CsgType::INTERSECTION, false, true, true, true)]
    #[case(&CsgType::INTERSECTION, false, true, false, true)]
    #[case(&CsgType::INTERSECTION, false, false, true, false)]
    #[case(&CsgType::INTERSECTION, false, false, false, false)]
    #[case(&CsgType::DIFFERENCE, true, true, true, false)]
    #[case(&CsgType::DIFFERENCE, true, true, false, true)]
    #[case(&CsgType::DIFFERENCE, true, false, true, false)]
    #[case(&CsgType::DIFFERENCE, true, false, false, true)]
    #[case(&CsgType::DIFFERENCE, false, true, true, true)]
    #[case(&CsgType::DIFFERENCE, false, true, false, true)]
    #[case(&CsgType::DIFFERENCE, false, false, true, false)]
    #[case(&CsgType::DIFFERENCE, false, false, false, false)]
    fn evaluating_rule_for_csg_operation(
        #[case] op: &CsgType,
        #[case] lhit: bool,
        #[case] inl: bool,
        #[case] inr: bool,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, intersection_allowed(op, lhit, inl, inr))
    }

    #[rstest]
    #[case(CsgType::UNION, 0, 3)]
    #[case(CsgType::INTERSECTION, 1, 2)]
    #[case(CsgType::DIFFERENCE, 0, 1)]
    fn filtering_list_of_intersections(
        #[case] csg_type: CsgType,
        #[case] x0: usize,
        #[case] x1: usize,
    ) {
        let mut world = World::default();

        let s1_id = world.add_shape(Shape::sphere_default());
        let s2_id = world.add_shape(Shape::cube_default());
        let c_id = world.create_csg(csg_type.clone(), s1_id, s2_id);

        let to_filter = Intersections::from(vec![
            Intersection::new(1.0, s1_id),
            Intersection::new(2.0, s2_id),
            Intersection::new(3.0, s1_id),
            Intersection::new(4.0, s2_id),
        ]);

        let intersections = filter_intersections(&world, c_id, to_filter.clone());

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.get(0), &to_filter.xs[x0]);
        assert_eq!(intersections.get(1), &to_filter.xs[x1]);
    }

    #[test]
    fn ray_misses_csg_objet() {
        let mut world = World::default();

        let s1_id = world.add_shape(Shape::sphere_default());
        let s2_id = world.add_shape(Shape::cube_default());
        let c_id = world.create_csg(CsgType::UNION, s1_id, s2_id);

        let ray = Ray::with(point_i(0, 2, -5), vector_i(0, 0, 1));

        let xs = csg_intersects(&world, &ray, c_id, s1_id, s2_id);

        assert!(xs.xs.is_empty())
    }

    #[test]
    fn ray_hits_csg_object() {
        let mut world = World::default();

        let s1_id = world.add_shape(Shape::sphere_default());
        let s2_id = world.add_shape(
            Shape::sphere_default().with_transform(Matrix::identity().translate(0.0, 0.0, 0.5)),
        );

        let c_id = world.create_csg(CsgType::UNION, s1_id, s2_id);

        let ray = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));

        let xs = csg_intersects(&world, &ray, c_id, s1_id, s2_id);

        assert_eq!(xs.xs.len(), 2);

        assert_eq!(xs.get(0).t, 4.0);
        assert_eq!(xs.get(0).object_id, s1_id);

        assert_eq!(xs.get(1).t, 6.5);
        assert_eq!(xs.get(1).object_id, s2_id);
    }
}

fn get_csg(shape: &Shape) -> (CsgType, usize, usize) {
    match shape.shape_type.clone() {
        ShapeType::CSG(csg_type, left, right) => (csg_type, left, right),
        _ => panic!("not a csg!!"),
    }
}

pub fn intersection_allowed(
    operand: &CsgType,
    left_hit: bool,
    inside_left: bool,
    inside_right: bool,
) -> bool {
    match operand {
        CsgType::UNION => (left_hit && !inside_right) || (!left_hit && !inside_left),
        CsgType::INTERSECTION => (left_hit && inside_right) || (!left_hit && inside_left),
        CsgType::DIFFERENCE => (left_hit && !inside_right) || (!left_hit && inside_left),
    }
}

pub fn filter_intersections(
    world: &World,
    csg_id: usize,
    intersections: Intersections,
) -> Intersections {
    let mut inside_left = false;
    let mut inside_right = false;

    let mut result: Vec<Intersection> = vec![];

    let csg_shape = world.get_shape(csg_id);
    let (operand, left_id, _) = get_csg(csg_shape);

    for intersection in intersections.xs {
        let left_hit = world.includes(left_id, intersection.object_id);

        if intersection_allowed(&operand, left_hit, inside_left, inside_right) {
            result.push(intersection);
        }

        if left_hit {
            inside_left = !inside_left;
        } else {
            inside_right = !inside_right;
        }
    }

    Intersections::from(result)
}

pub fn csg_intersects(
    world: &World,
    ray: &Ray,
    csg_id: usize,
    left: usize,
    right: usize,
) -> Intersections {
    let left = world.get_shape(left);
    let right = world.get_shape(right);

    let mut left_xs = left.intersects(world, ray);
    let mut right_xs = right.intersects(world, ray);

    left_xs.xs.append(right_xs.xs.as_mut());

    left_xs.xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    filter_intersections(world, csg_id, left_xs)
}
