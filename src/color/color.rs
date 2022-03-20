use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[test]
fn clamp_test() {
    assert_eq!(clamp(-1), 0);
    assert_eq!(clamp(0), 0);
    assert_eq!(clamp(1), 1);
    assert_eq!(clamp(254), 254);
    assert_eq!(clamp(255), 255);
    assert_eq!(clamp(256), 255);
    assert_eq!(clamp(1000), 255);
}

fn clamp(n: i32) -> i32 {
    n.max(0).min(255)
}

fn to_255(f: f32) -> i32 {
    clamp((f * 255.0).round() as i32)
}

impl Color {
    pub fn de_normalized(self) -> Vec<i32> {
        vec![to_255(self.r), to_255(self.g), to_255(self.b)]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        is_equal_float(self.r, other.r)
            && is_equal_float(self.g, other.g)
            && is_equal_float(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

fn is_equal_float(a: f32, b: f32) -> bool {
    (a - b).abs() < crate::tuple::EPSILON
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b }
}
