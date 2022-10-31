#[cfg(test)]
mod group_test {
    use crate::matrix::Matrix;
    use crate::shape::{Shape, ShapeType};

    #[test]
    pub fn creating_new_group() {
        let g = Shape::group();
        assert_eq!(g.inverse_transformation, Matrix::identity().inverse());
        assert!(matches!(g.shape_type, ShapeType::Group(children) if children.is_empty()))
    }

    #[test]
    pub fn shape_has_parent_attribute() {
        let g = Shape::group();
        assert!(g.parent.is_none())
    }

    #[test]
    fn add_child_to_group() {
        let s = Shape::sphere_default();
        let mut g = Shape::group_with_children(vec![s]);

        assert!(matches!(&g.shape_type, ShapeType::Group(children) if !children.is_empty()));
        assert!(matches!(&g.shape_type, ShapeType::Group(children) if
                    matches!(children.get(0).unwrap().shape_type, ShapeType::Sphere)));
        assert!(matches!(&g.shape_type, ShapeType::Group(children) if
                    matches!(
                        children.get(0).unwrap().parent.unwrap().shape_type,
                        ShapeType::Group(_))));
    }
}
