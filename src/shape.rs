pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::{is_zero_float, Matrix};
use crate::rays::Ray;
use crate::shape::cone::{cone_intersects, cone_normal_at};
use crate::shape::cube::{cube_intersects, cube_normal_at};
use crate::shape::cylinder::{cylinder_intersects, cylinder_normal_at};
use crate::shape::plane::{plane_intersects, plane_normal_at};
use crate::shape::sphere::{sphere_intersects, sphere_normal_at};
use crate::tuple::Tuple;
use crate::World;

#[derive(PartialEq, Clone, Debug)]
pub enum ShapeType {
    Sphere,
    Plane,
    Cube,
    Cylinder(f64, f64, bool), // Cylinder(min_y, max_y, closed)
    Cone(f64, f64, bool),     // Cone(min_y, max_y, closed)
    Group(Vec<ShapeId>),      // Group(children)
}

pub type ShapeId = usize;

#[derive(PartialEq, Clone, Debug)]
pub struct Shape {
    pub inverse_transformation: Matrix,
    pub material: Material,
    pub shape_type: ShapeType,
    pub parent: Option<ShapeId>,
    pub id: Option<ShapeId>,
}

impl Shape {
    fn default(shape_type: ShapeType) -> Self {
        Shape {
            shape_type,
            material: Material::default(),
            inverse_transformation: Matrix::identity().inverse(),
            parent: None,
            id: None,
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

    pub fn group() -> Self {
        Shape::default(ShapeType::Group(vec![]))
    }

    pub fn is_group(&self) -> bool {
        matches!(self.shape_type, ShapeType::Group(_))
    }

    pub fn set_parent(mut self, parent_id: ShapeId) -> Self {
        self.parent = Some(parent_id);
        self
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

    pub fn normal_at(&self, world: &World, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let object_point = self.world_to_object(world, world_point);

        let object_normal = match &self.shape_type {
            ShapeType::Sphere => sphere_normal_at(object_point),
            ShapeType::Plane => plane_normal_at(object_point),
            ShapeType::Cube => cube_normal_at(object_point),
            ShapeType::Cylinder(y_min, y_max, _) => {
                cylinder_normal_at(object_point, *y_min, *y_max)
            }
            ShapeType::Cone(y_min, y_max, _) => cone_normal_at(object_point, *y_min, *y_max),
            ShapeType::Group(_) => {
                panic!("should never calculate normal for a group, it doesn't exist.")
            }
        };

        self.normal_to_world(world, object_normal)
    }

    pub fn intersects(&self, world: &World, ray: &Ray) -> Intersections {
        let transform = match self.parent {
            Some(parent_id) => {
                &self.inverse_transformation * &world.get_shape(parent_id).inverse_transformation
            }
            None => self.inverse_transformation,
        };

        let transformed_ray = ray.transform(&transform);

        let v = match &self.shape_type {
            ShapeType::Sphere => sphere_intersects(&transformed_ray),
            ShapeType::Plane => plane_intersects(&transformed_ray),
            ShapeType::Cube => cube_intersects(&transformed_ray),
            ShapeType::Cylinder(y_min, y_max, closed) => {
                cylinder_intersects(&transformed_ray, *y_min, *y_max, *closed)
            }
            ShapeType::Cone(y_min, y_max, closed) => {
                cone_intersects(&transformed_ray, *y_min, *y_max, *closed)
            }
            ShapeType::Group(child_ids) => {
                let mut xs: Vec<Intersection> =
                    child_ids
                        .iter()
                        .fold(Vec::new(), |mut intersections, child_id| {
                            intersections.append(
                                world
                                    .get_shape(*child_id)
                                    .intersects(world, ray)
                                    .xs
                                    .as_mut(),
                            );
                            intersections
                        });

                xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

                return Intersections { xs };
            }
        };

        Intersections {
            xs: v
                .into_iter()
                .map(|t| {
                    Intersection::new(
                        t,
                        self.id
                            .unwrap_or_else(|| panic!("Shape did not have id in .intersects()")),
                    )
                })
                .collect(),
        }
    }

    pub fn world_to_object(&self, world: &World, mut point_to_transform: Tuple) -> Tuple {
        point_to_transform = match self.parent {
            Some(parent_id) => world
                .get_shape(parent_id)
                .world_to_object(world, point_to_transform),
            None => point_to_transform,
        };
        point_to_transform * &self.inverse_transformation
    }

    pub fn normal_to_world(&self, world: &World, mut normal_to_transform: Tuple) -> Tuple {
        normal_to_transform = normal_to_transform * &self.inverse_transformation.transpose();
        normal_to_transform.w = 0.0;
        normal_to_transform = normal_to_transform.normalize();

        normal_to_transform = match self.parent {
            Some(parent_id) => world
                .get_shape(parent_id)
                .normal_to_world(world, normal_to_transform),
            None => normal_to_transform,
        };

        normal_to_transform
    }

    pub fn with_transform(mut self, transformation: Matrix) -> Self {
        self.inverse_transformation = transformation.inverse();
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn is_in_group(&self) -> bool {
        self.parent.is_some()
    }

    pub fn is_opaque(&self) -> bool {
        is_zero_float(self.material.transparency)
    }
}

#[cfg(test)]
mod shape_test {
    use crate::shape::ShapeType;
    use crate::tuple::point_i;
    use crate::{point, vector, Matrix, Shape, World};
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn converting_point_from_world_to_object_space() {
        let g1 = Shape::group().with_transform(Matrix::identity().rotate_y(FRAC_PI_2));
        let g2 = Shape::group().with_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let sphere =
            Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        let world = World::default()
            .with_group_and_children(g1, vec![g2])
            .add_shape_to_existing_group(1, sphere);

        let sphere = world.get_shape(2);
        assert_eq!(sphere.shape_type, ShapeType::Sphere);
        let transformed_point = sphere.world_to_object(&world, point_i(-2, 0, -10));

        assert_eq!(transformed_point, point_i(0, 0, -1));
    }

    #[test]
    fn converting_normal_from_object_to_world_space() {
        let g0 = Shape::group().with_transform(Matrix::identity().rotate_y(FRAC_PI_2));
        let g1 = Shape::group().with_transform(Matrix::identity().scale(1.0, 2.0, 3.0));
        let world = World::default()
            .with_group_and_children(g0, vec![g1])
            .add_shape_to_existing_group(1, Shape::sphere_default());

        let sphere = world.get_shape(2);
        let transformed_vector = sphere.normal_to_world(
            &world,
            vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );

        assert_eq!(transformed_vector, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    fn finding_normal_on_child_object() {
        let g0 = Shape::group().with_transform(Matrix::identity().rotate_y(FRAC_PI_2));
        let g1 = Shape::group().with_transform(Matrix::identity().scale(1.0, 2.0, 3.0));
        let world = World::default()
            .with_group_and_children(g0, vec![g1])
            .add_shape_to_existing_group(
                1,
                Shape::sphere_default().with_transform(Matrix::identity().translate(5.0, 0.0, 0.0)),
            );

        let normal = world
            .get_shape(2)
            .normal_at(&world, point(1.7321, 1.1547, -5.5774));

        assert_eq!(normal, vector(0.285703, 0.42854, -0.85716));
    }
}
