use crate::intersection::{Intersection, Intersections};
use crate::rays::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;
use std::cell::Ref;
use std::rc::Rc;

#[cfg(test)]
mod group_test {
    use crate::matrix::Matrix;
    use crate::rays::Ray;
    use crate::shape::{Shape, ShapeType};
    use crate::tuple::{point_i, vector_i};
    use std::borrow::Borrow;
    use std::rc::Rc;

    #[test]
    pub fn creating_new_group() {
        let g = Shape::group();
        assert_eq!(g.inverse_transformation, Matrix::identity().inverse());
        assert!(!g.has_children())
    }

    #[test]
    pub fn shape_has_parent_attribute() {
        let g = Shape::group();
        assert!(g.get_parent().is_none())
    }

    #[test]
    fn add_child_to_group() {
        let s = Shape::sphere_default();
        let g = Shape::group().to_rc();
        Shape::add_shape_to_group(&g, s);

        assert!(g.has_children());
        matches!(
            g.get_children().unwrap().get(0).unwrap().shape_type,
            ShapeType::Sphere
        );
        matches!(
            g.get_children()
                .unwrap()
                .get(0)
                .unwrap()
                .get_parent()
                .unwrap()
                .borrow()
                .shape_type,
            ShapeType::Group(_)
        );
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let g = Shape::group();
        let r = Ray::with(point_i(0, 0, 0), vector_i(0, 0, 1));

        let xs = Shape::intersects(g.to_rc(), &r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let s1 = Shape::sphere_default();
        let s2 =
            Shape::sphere_default().with_transform(Matrix::identity().translate(0.0, 0.0, -3.0));
        let s3 =
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let children = vec![s1.clone(), s2.clone(), s3.clone()];

        let group = Shape::group_with_children(children);
        let ray = Ray::with(point_i(0, 0, -5), vector_i(0, 0, 1));

        let xs = Shape::intersects(group.to_rc(), &ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs.get(0).object.as_ref(), &s2);
        assert_eq!(xs.get(1).object.as_ref(), &s2);
        assert_eq!(xs.get(2).object.as_ref(), &s1);
        assert_eq!(xs.get(3).object.as_ref(), &s1);
    }
}

// pub fn group_intersects<'a, 'b>(
//     ray: &'b Ray,
//     children: Ref<'a, Vec<Rc<Shape>>>,
// ) -> Vec<Intersection<'a>> {
//     let mut all_intersections = children
//         .iter()
//         .flat_map(|child| Shape::intersects(child.to_rc(), ray).xs)
//         .map(|intersection| Intersection { ..intersection })
//         .collect::<Vec<Intersection>>();
//
//     all_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
//
//     all_intersections
// }
