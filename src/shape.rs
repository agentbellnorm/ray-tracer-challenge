pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::shape::cone::{cone_intersects, cone_normal_at};
use crate::shape::cube::{cube_intersects, cube_normal_at};
use crate::shape::cylinder::{cylinder_intersects, cylinder_normal_at};
use crate::shape::plane::{plane_intersects, plane_normal_at};
use crate::shape::sphere::{sphere_intersects, sphere_normal_at};
use crate::tuple::Tuple;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub enum ShapeType {
    Sphere,
    Plane,
    Cube,
    Cylinder(f64, f64, bool),       // Cylinder(min_y, max_y, closed)
    Cone(f64, f64, bool),           // Cone(min_y, max_y, closed)
    Group(Vec<Rc<RefCell<Shape>>>), // Group(children, parent)
}

pub type Scene = Vec<Shape>;

#[derive(Clone, Debug)]
pub struct Shape {
    pub inverse_transformation: Matrix,
    pub material: Material,
    pub shape_type: ShapeType,
    pub parent: Option<Rc<RefCell<Shape>>>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.inverse_transformation == other.inverse_transformation
            && self.material == other.material
            && self.shape_type == other.shape_type
            && *self.parent.borrow() == *other.parent.borrow()
    }
}

impl Shape {
    fn default(shape_type: ShapeType) -> Self {
        Shape {
            shape_type,
            material: Material::default(),
            inverse_transformation: Matrix::identity().inverse(),
            parent: None,
        }
    }

    pub fn plane_default() -> Self {
        Shape::default(ShapeType::Plane)
    }

    pub fn sphere_default() -> Self {
        Shape::default(ShapeType::Sphere)
    }

    pub fn cube_default() -> Self {
        Shape::default(ShapeType::Cube)
    }

    pub fn cylinder_default() -> Self {
        Shape::default(ShapeType::Cylinder(-f64::INFINITY, f64::INFINITY, false))
    }

    pub fn cylinder(min: f64, max: f64, closed: bool) -> Self {
        Shape::default(ShapeType::Cylinder(min, max, closed))
    }

    pub fn cone_default() -> Self {
        Shape::default(ShapeType::Cone(-f64::INFINITY, f64::INFINITY, false))
    }

    pub fn cone(y_min: f64, y_max: f64, closed: bool) -> Self {
        Shape::default(ShapeType::Cone(y_min, y_max, closed))
    }

    pub fn group() -> Shape {
        Shape::default(ShapeType::Group(vec![]))
    }

    pub fn group_with_children(children: Vec<Shape>) -> Rc<RefCell<Shape>> {
        let group = Shape::group().pack();
        for child in children {
            Self::add_child_rc_to_group(group.clone(), child.pack())
        }

        group
    }

    pub fn pack(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn sphere_from_material(material: Material) -> Self {
        Shape::sphere_default().with_material(material)
    }

    pub fn sphere_from_transform(transform: Matrix) -> Self {
        Shape::sphere_default().with_transform(transform)
    }

    pub fn plane_from_material(material: Material) -> Self {
        Shape::plane_default().with_material(material)
    }

    pub fn sphere_glass() -> Self {
        Shape::sphere_from_material(Material::glass())
    }

    pub fn sphere_chrome() -> Self {
        Shape::sphere_from_material(Material::chrome())
    }

    // pub fn add_shape_to_group(group: &mut Rc<RefCell<Shape>>, child: &mut Rc<RefCell<Shape>>) {
    //     // add parent to child
    //     child.parent = Some(RefCell::new(group.clone()));
    //
    //     // add child to parent
    //     if let ShapeType::Group(children) = &group.shape_type {
    //         children.borrow_mut().push(Rc::clone(&Rc::new(child)));
    //     } else {
    //         panic!("can only add children to group")
    //     }
    // }

    pub fn add_child_rc_to_group(group: Rc<RefCell<Shape>>, child: Rc<RefCell<Shape>>) {
        // add parent to child
        // child.parent = Some(RefCell::new(group.clone()));

        child.borrow_mut().parent = Some(group.clone());
        // let x = group.borrow_mut();
        if let ShapeType::Group(children) = &mut group.as_ref().borrow_mut().shape_type {
            children.push(child.clone())
        };
    }

    // pub fn add_children_to_group(group: Rc<Shape>, children: Vec<Shape>) {
    //     for child in children {
    //         Self::add_child_to_group(group.clone(), child)
    //     }
    // }

    pub fn is_group(&self) -> bool {
        matches!(self.shape_type, ShapeType::Group(_))
    }

    pub fn has_children(&self) -> bool {
        if let ShapeType::Group(children) = &self.shape_type {
            return !children.is_empty();
        } else {
            return false;
        }
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Shape>>> {
        match self.parent.borrow() {
            Some(parent) => Some(parent.clone()),
            None => None,
        }
    }

    pub fn get_children(&self) -> Option<&Vec<Rc<RefCell<Shape>>>> {
        match &self.shape_type {
            ShapeType::Group(children) => Some(children),
            _ => None,
        }
    }

    pub fn world_to_object(&self, point: Tuple) -> Tuple {
        let mut res_point = point.clone();
        println!("{:?}", self.parent);
        if let Some(parent) = &self.parent {
            res_point = parent.as_ref().borrow().world_to_object(point);
        }

        res_point * &self.inverse_transformation
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let object_point = world_point * &self.inverse_transformation;

        let object_normal = match &self.shape_type {
            ShapeType::Sphere => sphere_normal_at(object_point),
            ShapeType::Plane => plane_normal_at(object_point),
            ShapeType::Cube => cube_normal_at(object_point),
            ShapeType::Cylinder(y_min, y_max, _) => {
                cylinder_normal_at(object_point, *y_min, *y_max)
            }
            ShapeType::Cone(y_min, y_max, _) => cone_normal_at(object_point, *y_min, *y_max),
            ShapeType::Group(_) => panic!("not yet"),
        };

        let mut world_normal = object_normal * &self.inverse_transformation.transpose();
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn intersects(shape: Rc<RefCell<Shape>>, ray: &Ray) -> Intersections {
        let shape_ref = shape.as_ref().borrow();
        let transformed_ray = ray.transform(&shape.as_ref().borrow().inverse_transformation);

        if let ShapeType::Group(children) = &shape_ref.shape_type {
            let mut all_intersections = children
                .iter()
                .flat_map(|child| Self::intersects(child.clone(), &transformed_ray).xs)
                .collect::<Vec<Intersection>>();

            // println!("den är {:?}", all_intersections.len());

            all_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

            return Intersections {
                xs: all_intersections,
            };
        }

        let v = match &shape_ref.shape_type {
            ShapeType::Sphere => sphere_intersects(&transformed_ray),
            ShapeType::Plane => plane_intersects(&transformed_ray),
            ShapeType::Cube => cube_intersects(&transformed_ray),
            ShapeType::Cylinder(y_min, y_max, closed) => {
                cylinder_intersects(&transformed_ray, *y_min, *y_max, *closed)
            }
            ShapeType::Cone(y_min, y_max, closed) => {
                cone_intersects(&transformed_ray, *y_min, *y_max, *closed)
            }
            unknown => panic!("what shape type?? {:?}", unknown),
        };

        Intersections {
            xs: v
                .into_iter()
                .map(|t| Intersection::new(t, shape.clone()))
                .collect(),
        }
    }

    pub fn with_transform(mut self, transformation: Matrix) -> Self {
        self.inverse_transformation = transformation.inverse();
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}
