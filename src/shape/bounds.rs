use std::{
    f64::{INFINITY, NEG_INFINITY},
    ops::Mul,
};

use crate::{
    matrix::Matrix,
    shape::ShapeType,
    tuple::{point, Tuple},
    world::World,
};

use super::Shape;

#[cfg(test)]
mod bounds_test {
    use std::f64::{INFINITY, NEG_INFINITY};

    const TEST_BOUNDS: Bounds = Bounds {
        min: point_i(-1, -1, -1),
        max: point_i(1, 1, 1),
    };

    use parameterized::parameterized;

    use crate::{
        shape::{
            bounds::{
                add_point_to_bounds, bounds_contain_other_bounds, bounds_contains_point, combine_bounds,
                Bounds,
            },
            Shape,
        },
        tuple::{point, point_i, Tuple},
        world::World,
    };

    use super::{bounds, NO_BOUNDS};

    #[test]
    fn add_point_to_empty_bounds() {
        let p1 = point_i(-5, 2, 0);
        let p2 = point_i(7, 0, -3);

        let mut bounds = add_point_to_bounds(p1, &NO_BOUNDS);
        let mut bounds = add_point_to_bounds(p2, &bounds);

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

    // #[parameterized(
    // scenario= {     "+x",                   "-x",                   "+y",                   "-y",                   "+z",                   "-z",                   "inside"                },
    // origin = {      point(5.0, 0.5, 0.0),   point(-5.0, 0.5, 0.0),  point(0.5, 5.0, 0.0),   point(0.5, -5.0, 0.0),  point(0.5, 0.0, 5.0),   point(0.5, 0.0, -5.0),  point(0.0, 0.5, 0.0)    },
    // direction = {   vector_i(-1, 0, 0),     vector_i(1, 0, 0),      vector_i(0, -1, 0),     vector_i(0, 1, 0),      vector_i(0, 0, -1),     vector_i(0, 0, 1),      vector_i(0, 0, 1)       },
    // t1 = {          4.0,                    4.0,                    4.0,                    4.0,                    4.0,                    4.0,                    -1.0                    },
    // t2 = {          6.0,                    6.0,                    6.0,                    6.0,                    6.0,                    6.0,                    1.0                     }
    // )]
    // pub fn ray_intersects_cube(scenario: &str, origin: Tuple, direction: Tuple, t1: f64, t2: f64) {

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

    // #[test]
    // fn test_transformed_sphere_bounds() {
    //     let mut world = World::default();
    //     let sphere = world.add_shape(
    //         Shape::sphere_default().with_transform(Matrix::identity().scale(2.0, 2.0, 2.0)),
    //     );
    //     let group = world.add_shape(Shape::group());
    //     world.add_shape_to_group(group, sphere);
    //     assert_eq!(
    //         group_bounds(&world, group),
    //         Bounds {
    //             min: point(-2.0, -2.0, -2.0),
    //             max: point(2.0, 2.0, 2.0),
    //         }
    //     );
    // }
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
            min: self.min * matrix,
            max: self.max * matrix,
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

pub fn group_bounds(world: &World, group_id: usize) -> Bounds {
    let group = world.get_shape(group_id);
    if let ShapeType::Group(children, _) = &group.shape_type {
        return children
            .into_iter()
            .map(|child| {
                bounds_of_transformed_corners(
                    &bounds(world, *child),
                    &group.inverse_transformation.inverse(),
                )
            })
            .fold(NO_BOUNDS, combine_bounds);
    } else {
        panic!("group id {:?} is not a group! [{:?}]", group_id, group)
    }
}

fn bounds(world: &World, shape_id: usize) -> Bounds {
    let shape = world.get_shape(shape_id);
    let transformation = shape.inverse_transformation.inverse();
    match &shape.shape_type {
        ShapeType::Sphere => SPHERE_BOUND,
        ShapeType::Plane => PLANE_BOUNDS,
        ShapeType::Cube => CUBE_BOUNDS * &transformation,
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
        ShapeType::Group(children, _) => group_bounds(world, shape_id),
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

pub fn bounds_to_cube(bounds: &Bounds) -> Shape {
    let width = bounds.max.x - bounds.min.x;
    let height = bounds.max.y - bounds.min.y;
    let depth = bounds.max.z - bounds.min.z;

    println!("{:?}, {:?}, {:?}", width, height, depth);

    Shape::cube_default().with_transform(Matrix::identity().scale(width, height, width))
}

/*
 * The bounding box of a set of (potentially rotated) corners of a box
 * */
fn corners_to_bounds(corners: Corners) -> Bounds {
    let mut x = (f64::INFINITY, -f64::INFINITY);
    let mut y = (f64::INFINITY, -f64::INFINITY);
    let mut z = (f64::INFINITY, -f64::INFINITY);

    for point in corners {
        x.0 = f64::min(point.x, x.0);
        x.1 = f64::max(point.x, x.1);

        y.0 = f64::min(point.y, y.0);
        y.1 = f64::max(point.y, y.1);

        z.0 = f64::min(point.z, z.0);
        z.1 = f64::max(point.z, z.1);
    }

    Bounds {
        min: point(x.0, y.0, z.0),
        max: point(x.1, y.1, z.1),
    }
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
        corners[0] * transform,
        corners[1] * transform,
        corners[2] * transform,
        corners[3] * transform,
        corners[4] * transform,
        corners[5] * transform,
        corners[6] * transform,
        corners[7] * transform,
    ]
}
