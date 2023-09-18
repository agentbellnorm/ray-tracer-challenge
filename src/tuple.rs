use std::ops::{Add, Div, Mul, Neg, Sub};

pub const EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        assert!(is_vector(self) && is_vector(other));

        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Self) -> Self {
        assert!(is_vector(self) && is_vector(other));

        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn reflect(&self, n: &Tuple) -> Tuple {
        self - &(n * self.dot(n) * 2.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self, other)
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<'a> Sub for &'a Tuple {
    type Output = Tuple;
    fn sub(self, other: Self) -> Self::Output {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<f64> for &Tuple {
    type Output = Tuple;
    fn mul(self, n: f64) -> Self::Output {
        Tuple {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
            w: self.w * n,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

fn is_equal_float(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub const fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub const fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn point_i(x: i32, y: i32, z: i32) -> Tuple {
    point(x as f64, y as f64, z as f64)
}

pub fn vector_i(x: i32, y: i32, z: i32) -> Tuple {
    vector(x as f64, y as f64, z as f64)
}

pub fn is_point(t: &Tuple) -> bool {
    t.w == 1.0
}

pub fn is_vector(t: &Tuple) -> bool {
    t.w == 0.0
}

pub fn is_equal(t1: &Tuple, t2: &Tuple) -> bool {
    is_equal_float(t1.x, t2.x)
        && is_equal_float(t1.y, t2.y)
        && is_equal_float(t1.z, t2.z)
        && is_equal_float(t1.w, t2.w)
}
