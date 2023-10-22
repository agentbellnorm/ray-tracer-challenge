use super::CsgType;

#[cfg(test)]
mod csg_test {
    use crate::{
        shape::csg::intersection_allowed,
        shape::{CsgType, Shape, ShapeType::CSG},
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
    #[case(CsgType::UNION, true, true, true, false)]
    #[case(CsgType::UNION, true, true, false, true)]
    #[case(CsgType::UNION, true, false, true, false)]
    #[case(CsgType::UNION, true, false, false, true)]
    #[case(CsgType::UNION, false, true, true, false)]
    #[case(CsgType::UNION, false, true, false, false)]
    #[case(CsgType::UNION, false, false, true, true)]
    #[case(CsgType::UNION, false, false, false, true)]
    #[case(CsgType::INTERSECTION, true, true, true, true)]
    #[case(CsgType::INTERSECTION, true, true, false, false)]
    #[case(CsgType::INTERSECTION, true, false, true, true)]
    #[case(CsgType::INTERSECTION, true, false, false, false)]
    #[case(CsgType::INTERSECTION, false, true, true, true)]
    #[case(CsgType::INTERSECTION, false, true, false, true)]
    #[case(CsgType::INTERSECTION, false, false, true, false)]
    #[case(CsgType::INTERSECTION, false, false, false, false)]
    #[case(CsgType::DIFFERENCE, true, true, true, false)]
    #[case(CsgType::DIFFERENCE, true, true, false, true)]
    #[case(CsgType::DIFFERENCE, true, false, true, false)]
    #[case(CsgType::DIFFERENCE, true, false, false, true)]
    #[case(CsgType::DIFFERENCE, false, true, true, true)]
    #[case(CsgType::DIFFERENCE, false, true, false, true)]
    #[case(CsgType::DIFFERENCE, false, false, true, false)]
    #[case(CsgType::DIFFERENCE, false, false, false, false)]
    fn evaluating_rule_for_csg_operation(
        #[case] op: CsgType,
        #[case] lhit: bool,
        #[case] inl: bool,
        #[case] inr: bool,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, intersection_allowed(op, lhit, inl, inr))
    }
}

pub fn intersection_allowed(
    operand: CsgType,
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
