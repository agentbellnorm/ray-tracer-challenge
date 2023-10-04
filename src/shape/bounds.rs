use std::{
    f64::{INFINITY, NEG_INFINITY},
    ops::Mul,
};

use crate::{
    matrix::Matrix,
    rays::Ray,
    shape::ShapeType,
    tuple::{point, Tuple},
    world::World,
};

use super::cube::cube_intersects;

#[cfg(test)]
mod bounds_test {
    use std::f64::{consts::FRAC_PI_4, INFINITY, NEG_INFINITY};

    use parameterized::parameterized;

    use crate::{
        matrix::Matrix,
        rays::Ray,
        shape::{
            bounds::{
                add_point_to_bounds, bounds_contain_other_bounds, bounds_contains_point,
                bounds_of_transformed_corners, combine_bounds, ray_misses_bounds, Bounds,
            },
            Shape,
        },
        tuple::{point, point_i, vector_i, Tuple},
        world::World,
    };

    use super::{bounds, parent_space_bounds_of, NO_BOUNDS};

    #[test]
    fn add_point_to_empty_bounds() {
        let p1 = point_i(-5, 2, 0);
        let p2 = point_i(7, 0, -3);

        let bounds = add_point_to_bounds(p1, &NO_BOUNDS);
        let bounds = add_point_to_bounds(p2, &bounds);

        assert_eq!(bounds.min, point_i(-5, 0, -3));
        assert_eq!(bounds.max, point_i(7, 2, 0));
    }

    #[test]
    fn sphere_bounds() {
        let mut world = World::default();
        let sphere = world.add_shape(Shape::sphere_default());
        let bounds = bounds(&world, sphere);

        assert_eq!(bounds.min, point_i(-1, -1, -1));
        assert_eq!(bounds.max, point_i(1, 1, 1));
    }

    #[test]
    fn plane_bounds() {
        let mut world = World::default();
        let plane = world.add_shape(Shape::plane_default());
        let b = bounds(&world, plane);

        assert_eq!(b.min, point(NEG_INFINITY, 0.0, NEG_INFINITY));
        assert_eq!(b.max, point(INFINITY, 0.0, INFINITY));
    }

    #[test]
    fn cube_bounds() {
        let mut world = World::default();
        let cube = world.add_shape(Shape::cube_default());
        let bounds = bounds(&world, cube);

        assert_eq!(bounds.min, point_i(-1, -1, -1));
        assert_eq!(bounds.max, point_i(1, 1, 1));
    }

    #[test]
    fn unbounded_cylinder_bounds() {
        let mut world = World::default();
        let cylinder = world.add_shape(Shape::cylinder_default());
        let bounds = bounds(&world, cylinder);

        assert_eq!(bounds.min, point(-1.0, NEG_INFINITY, -1.0));
        assert_eq!(bounds.max, point(1.0, INFINITY, 1.0));
    }

    #[test]
    fn bounded_cylinder_bounds() {
        let mut world = World::default();
        let cylinder = world.add_shape(Shape::cylinder(-5.0, 3.0, false));
        let bounds = bounds(&world, cylinder);

        assert_eq!(bounds.min, point(-1.0, -5.0, -1.0));
        assert_eq!(bounds.max, point(1.0, 3.0, 1.0));
    }

    #[test]
    fn cone_bounds() {
        let mut world = World::default();
        let cone = world.add_shape(Shape::cone_default());
        let bounds = bounds(&world, cone);

        assert_eq!(bounds.min, point(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY));
        assert_eq!(bounds.max, point(INFINITY, INFINITY, INFINITY));
    }

    #[test]
    fn bounded_cone_bounds() {
        let mut world = World::default();
        let cone = world.add_shape(Shape::cone(-5.0, 3.0, false));
        let bounds = bounds(&world, cone);

        assert_eq!(bounds.min, point_i(-5, -5, -5));
        assert_eq!(bounds.max, point_i(5, 3, 5));
    }

    #[test]
    fn triangle() {
        // todo implement for triangle
    }

    // #[test]
    // fn test_shape_arbitrary_bounds() {}

    #[test]
    fn add_two_bounds_together() {
        let b1 = Bounds {
            min: point_i(-5, -2, 0),
            max: point_i(7, 4, 4),
        };
        let b2 = Bounds {
            min: point_i(8, -7, -2),
            max: point_i(14, 2, 8),
        };

        assert_eq!(
            combine_bounds(b1, b2),
            Bounds {
                min: point_i(-5, -7, -2),
                max: point_i(14, 4, 8),
            }
        )
    }

    #[parameterized(
        point =  { point_i(5, -2, 0), point_i(11, 4, 7), point_i(8, 1, 3), point_i(3, 0, 3), point_i(8, -4, 3), point_i(8, 1, -1), point_i(13, 1, 3), point_i(8, 5, 3), point_i(8, 1, 8) },
        result = { true,              true,              true,             false,            false,             false,             false,             false,            false      },
    )]
    pub fn check_if_box_contains_point(point: Tuple, result: bool) {
        let bounds = Bounds {
            min: point_i(5, -2, 0),
            max: point_i(11, 4, 7),
        };

        assert_eq!(bounds_contains_point(&bounds, &point), result)
    }

    #[parameterized(
        min = {point_i(5, -2, 0), point_i(6, -1, 1), point_i(4, -3, -1), point_i(6, -1, 1)},
        max = {point_i(11, 4, 7), point_i(10, 3, 6), point_i(10, 3, 6),  point_i(12, 5, 8)},
        res = {true,              true,              false,              false}
    )]
    fn bounds_contains_bounds(min: Tuple, max: Tuple, res: bool) {
        let b1 = Bounds {
            min: point_i(5, -2, 0),
            max: point_i(11, 4, 7),
        };
        let b2 = Bounds { min, max };

        assert_eq!(bounds_contain_other_bounds(&b1, &b2), res)
    }

    #[test]
    fn transforming_bounding_box() {
        let b = Bounds {
            min: point_i(-1, -1, -1),
            max: point_i(1, 1, 1),
        };

        let transformation = Matrix::identity().rotate_y(FRAC_PI_4).rotate_x(FRAC_PI_4);

        assert_eq!(
            bounds_of_transformed_corners(&b, &transformation),
            Bounds {
                min: point(-1.414213, -1.707106, -1.707106),
                max: point(1.414213, 1.707106, 1.707106)
            }
        )
    }

    #[test]
    fn querying_shapes_bounding_box_in_parent_space() {
        let mut world = World::default();
        let sphere = world.add_shape(
            Shape::sphere_default().with_transform(
                Matrix::identity()
                    .scale(0.5, 2.0, 4.0)
                    .translate(1.0, -3.0, 5.0),
            ),
        );

        let bounds = parent_space_bounds_of(&world, sphere);

        assert_eq!(
            bounds,
            Bounds {
                min: point(0.5, -5.0, 1.0),
                max: point(1.5, -1.0, 9.0)
            }
        )
    }

    #[test]
    fn group_has_bounding_box_containing_children() {
        let mut world = World::default();

        let sphere = world.add_shape(
            Shape::sphere_default().with_transform(
                Matrix::identity()
                    .scale(2.0, 2.0, 2.0)
                    .translate(2.0, 5.0, -3.0),
            ),
        );

        let cylinder = world.add_shape(
            Shape::cylinder(-2.0, 2.0, false).with_transform(
                Matrix::identity()
                    .scale(0.5, 1.0, 0.5)
                    .translate(-4.0, -1.0, 4.0),
            ),
        );

        let group = world.add_shape(Shape::group());

        world.add_shape_to_group(group, sphere);
        world.add_shape_to_group(group, cylinder);

        let bounds = bounds(&world, group);
        let computed_bounds = match &world.get_shape(group).shape_type {
            crate::shape::ShapeType::Group(_, group_bounds) => group_bounds,
            _ => panic!("wat"),
        };

        assert_eq!(
            bounds,
            Bounds {
                min: point(-4.5, -3.0, -5.0),
                max: point(4.0, 7.0, 4.5)
            }
        );

        assert_eq!(
            computed_bounds.clone(),
            Bounds {
                min: point(-4.5, -3.0, -5.0),
                max: point(4.0, 7.0, 4.5)
            }
        )
    }

    #[parameterized(
        origin =    { point(5.0, 0.5, 0.0), point(-5.0, 0.5, 0.0), point(0.5, 5.0, 0.0), point(0.5, 5.0, 0.0), point(0.5, 0.0, 5.0), point(0.5, 0.0, -5.0), point(0.0, 0.5, 0.0), point_i(-2, 0, 0), point_i(0, 2, 0),  point_i(0, 0, -2), point_i(2, 0, 2),   point_i(0, 2, 2),   point_i(2, 2, 0)},
        direction = { vector_i(-1, 0, 0),   vector_i(1, 0, 0),     vector_i(0, -1, 0),   vector_i(0, 1, 0),    vector_i(0, 0, -1),   vector_i(0, 0, 1),     vector_i(0, 0, 1),    vector_i(2, 4, 6), vector_i(6, 2, 4), vector_i(4, 6, 2), vector_i(0, 0, -1), vector_i(0, -1, 0), vector_i(-1, 0, 0)},
        result =    { false,                false,                 false,                false,                false,                false,                 false,                true,              true,              true,              true,               true,               true}
    )]
    fn intersecting_ray_with_bounding_box_at_origin(origin: Tuple, direction: Tuple, result: bool) {
        let bounds = Bounds {
            min: point_i(-1, -1, -1),
            max: point_i(1, 1, 1),
        };
        let normalized_direction = direction.normalize();
        let ray = Ray::with(origin, normalized_direction);

        assert_eq!(ray_misses_bounds(&bounds, &ray), result)
    }

    #[parameterized(
        origin =    { point_i(15, 1, 2),  point_i(-5, -1, 4), point_i(7, 6, 5),   point_i(9, -5, 6), point_i(8, 2, 12),  point_i(6, 0, -5), point(8.0, 1.0, 3.5), point_i(9, -1, -8), point_i(8, 3, -4), point_i(9, -1, -2), point_i(4, 0, 9),   point_i(8, 6, -1),  point_i(12, 5, 4)},
        direction = { vector_i(-1, 0, 0), vector_i(1, 0,0),   vector_i(0, -1, 0), vector_i(0, 1, 0), vector_i(0, 0, -1), vector_i(0, 0, 1), vector_i(0, 0, 1),    vector_i(2, 4, 6),  vector_i(6, 2, 4), vector_i(4, 6, 2),  vector_i(0, 0, -1), vector_i(0, -1, 0), vector_i(-1, 0, 0)},
        result =    { false,              false,              false,              false,             false,              false,             false,                true,               true,              true,               true,               true,               true},

    )]
    fn intersecting_ray_with_non_cubic_bounds(origin: Tuple, direction: Tuple, result: bool) {
        let bounds = Bounds {
            min: point_i(5, -2, 0),
            max: point_i(11, 4, 7),
        };
        let normalized_direction = direction.normalize();
        let ray = Ray::with(origin, normalized_direction);

        assert_eq!(ray_misses_bounds(&bounds, &ray), result)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Bounds {
    pub min: Tuple,
    pub max: Tuple,
}

impl Mul<&Matrix> for Bounds {
    type Output = Self;
    fn mul(self, matrix: &Matrix) -> Self {
        Self {
            min: &self.min * matrix,
            max: &self.max * matrix,
        }
    }
}

type Corners = [Tuple; 8];

const SPHERE_BOUND: Bounds = Bounds {
    min: point(-1.0, -1.0, -1.0),
    max: point(1.0, 1.0, 1.0),
};
const PLANE_BOUNDS: Bounds = Bounds {
    min: point(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
    max: point(f64::INFINITY, 0.0, f64::INFINITY),
};
pub const CUBE_BOUNDS: Bounds = Bounds {
    min: point(-1.0, -1.0, -1.0),
    max: point(1.0, 1.0, 1.0),
};

pub const NO_BOUNDS: Bounds = Bounds {
    min: point(INFINITY, INFINITY, INFINITY),
    max: point(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
};

fn bounds_of_transformed_corners(bounds: &Bounds, transformation: &Matrix) -> Bounds {
    corners_to_bounds(transform_corners(bounds_to_corners(bounds), transformation))
}

pub fn parent_space_bounds_of(world: &World, shape: usize) -> Bounds {
    let bounds = bounds(world, shape);
    bounds_of_transformed_corners(
        &bounds,
        &world.get_shape(shape).inverse_transformation.inverse(),
    )
}

pub fn ray_misses_bounds(bounds: &Bounds, ray: &Ray) -> bool {
    cube_intersects(ray, bounds).is_empty()
}

pub fn bounds(world: &World, shape_id: usize) -> Bounds {
    let shape = world.get_shape(shape_id);
    match &shape.shape_type {
        ShapeType::Sphere => SPHERE_BOUND,
        ShapeType::Plane => PLANE_BOUNDS,
        ShapeType::Cube => CUBE_BOUNDS,
        ShapeType::Cylinder(y_min, y_max, _) => Bounds {
            min: point(-1.0, *y_min, -1.0),
            max: point(1.0, *y_max, 1.0),
        },
        ShapeType::Cone(y_min, y_max, _) => {
            let a = y_min.abs();
            let b = y_max.abs();
            let limit = a.max(b);
            Bounds {
                min: point(-limit, *y_min, -limit),
                max: point(limit, *y_max, limit),
            }
        }
        ShapeType::Triangle(_, _, _, _, _, _) => todo!(),
        ShapeType::Group(children, _) => children
            .into_iter()
            .map(|child: &usize| parent_space_bounds_of(&world, *child))
            .fold(NO_BOUNDS, combine_bounds),
    }
}

fn bounds_to_corners(bounds: &Bounds) -> Corners {
    let min = bounds.min;
    let max = bounds.max;

    [
        point(min.x, min.y, min.z),
        point(min.x, min.y, max.z),
        point(min.x, max.y, max.z),
        point(max.x, max.y, max.z),
        point(max.x, max.y, min.z),
        point(max.x, min.y, min.z),
        point(max.x, min.y, max.z),
        point(min.x, max.y, min.z),
    ]
}

/*
 * The bounding box of a set of (potentially rotated) corners of a box
 * */
fn corners_to_bounds(corners: Corners) -> Bounds {
    let mut bounds = NO_BOUNDS;

    for corner in corners {
        bounds = add_point_to_bounds(corner, &bounds);
    }

    bounds
}

fn add_point_to_bounds(p: Tuple, bounds: &Bounds) -> Bounds {
    Bounds {
        min: point(
            p.x.min(bounds.min.x),
            p.y.min(bounds.min.y),
            p.z.min(bounds.min.z),
        ),
        max: point(
            p.x.max(bounds.max.x),
            p.y.max(bounds.max.y),
            p.z.max(bounds.max.z),
        ),
    }
}

fn bounds_contains_point(bounds: &Bounds, p: &Tuple) -> bool {
    bounds.min.x <= p.x
        && p.x <= bounds.max.x
        && bounds.min.y <= p.y
        && p.y <= bounds.max.y
        && bounds.min.z <= p.z
        && p.z <= bounds.max.z
}

fn bounds_contain_other_bounds(bounds: &Bounds, other: &Bounds) -> bool {
    bounds_contains_point(bounds, &other.min) && bounds_contains_point(bounds, &other.max)
}

fn combine_bounds(a: Bounds, b: Bounds) -> Bounds {
    Bounds {
        min: point(
            f64::min(a.min.x, b.min.x),
            f64::min(a.min.y, b.min.y),
            f64::min(a.min.z, b.min.z),
        ),
        max: point(
            f64::max(a.max.x, b.max.x),
            f64::max(a.max.y, b.max.y),
            f64::max(a.max.z, b.max.z),
        ),
    }
}

fn transform_corners(corners: Corners, transform: &Matrix) -> Corners {
    [
        &corners[0] * transform,
        &corners[1] * transform,
        &corners[2] * transform,
        &corners[3] * transform,
        &corners[4] * transform,
        &corners[5] * transform,
        &corners[6] * transform,
        &corners[7] * transform,
    ]
}
