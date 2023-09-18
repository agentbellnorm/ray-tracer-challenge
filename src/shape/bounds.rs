use std::ops::Mul;

use crate::{
    matrix::Matrix,
    shape::ShapeType,
    tuple::{point, Tuple},
    world::World,
};

#[cfg(test)]
mod bounds_test {
    use crate::{shape::bounds::SPHERE_BOUND, world::World};

    use super::bound;

    // #[test]
    // fn bounds_circle() {
    //     let bounds = bound(Shape::sphere_default());
    //     assert_eq!(bounds.min, point(-1.0, -1.0, -1.0));
    //     assert_eq!(bounds.max, point(1.0, 1.0, 1.0));
    // }
    //
    // #[test]
    // fn bounds_plane() {
    //     let bounds = bound(Shape::plane_default());
    //     assert_eq!(bounds.min, point(-1.0, -1.0, -1.0));
    //     assert_eq!(bounds.max, point(1.0, 1.0, 1.0));
    // }

    #[test]
    fn group_bounds_run() {
        let world = World::test_world_with_group();
        assert_eq!(bound(&world, 1), SPHERE_BOUND);
    }
}

#[derive(PartialEq, Debug)]
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
    min: point(-f64::INFINITY, 0.0, -f64::INFINITY),
    max: point(f64::INFINITY, 0.0, f64::INFINITY),
};
pub const CUBE_BOUNDS: Bounds = Bounds {
    min: point(-1.0, -1.0, -1.0),
    max: point(1.0, 1.0, 1.0),
};

const NO_BOUNDS: Bounds = Bounds {
    min: point(0.0, 0.0, 0.0),
    max: point(0.0, 0.0, 0.0),
};

pub fn bound(world: &World, shape_id: usize) -> Bounds {
    let shape = world.get_shape(shape_id);
    match &shape.shape_type {
        ShapeType::Sphere => SPHERE_BOUND,
        ShapeType::Plane => PLANE_BOUNDS,
        ShapeType::Cube => CUBE_BOUNDS,
        ShapeType::Cylinder(y_min, y_max, _) => Bounds {
            min: point(-1.0, *y_min, -1.0),
            max: point(1.0, *y_max, 1.0),
        },
        ShapeType::Cone(y_min, y_max, _) => Bounds {
            min: point(-1.0, *y_min, -1.0),
            max: point(1.0, *y_max, 1.0),
        },
        ShapeType::Group(children) => children
            .into_iter()
            .map(|child| {
                let child_bounds = bound(world, *child);
                let child_corners = bounds_to_corners(&child_bounds);
                let transformed_corners =
                    transform_corners(child_corners, &shape.inverse_transformation);
                corners_to_bounds(transformed_corners)
            })
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

// // converts bounds of all children into group space and combines them to a single bounding box
// fn group_bounds(world: &World, group_id: usize) -> Bounds {
//     let group = world.get_shape(group_id);
//
//     if let ShapeType::Group(children) = &group.shape_type {
//         return children
//             .into_iter()
//             .map(|child| {
//                 let child_shape = world.get_shape(child.clone());
//                 let child_bounds = bound(world, child.clone());
//                 let child_transform = child_shape.inverse_transformation;
//                 let child_corners = bounds_to_corners(&child_bounds);
//                 let transformed_corners = transform_corners(child_corners, &child_transform);
//                 corners_to_bounds(transformed_corners)
//             })
//             .fold(NO_BOUNDS, combine_bounds);
//     } else {
//         todo!("why would you write a function where this is possible?");
//         panic!("why would you call this method with a non group");
//     }
// }
