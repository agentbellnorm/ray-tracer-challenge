#[cfg(test)]
mod group_test {
    use crate::matrix::Matrix;
    use crate::shape::{Shape, ShapeType};
    use crate::world::World;
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
        let g = Rc::new(Shape::group());
        Shape::add_shape_to_group(&g, s);

        // assert!(g.has_children());
        // matches!(
        //     g.get_children().unwrap().get(0).unwrap().shape_type,
        //     ShapeType::Sphere
        // );
        // println!("{:?}", g);
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
        // assert!(matches!(&gr.shape_type, ShapeType::Group(children) if
        //             matches!(
        //                 children.borrow().get(0).unwrap().get_parent().unwrap().shape_type,
        //                 ShapeType::Group(_))));
    }
}
