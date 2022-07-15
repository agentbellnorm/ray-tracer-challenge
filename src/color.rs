use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
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

#[test]
fn to_255_test() {
    assert_eq!(to_255(0.0), 0);
    assert_eq!(to_255(1.0), 255);
    assert_eq!(to_255(0.5), 128);
}

fn to_255(f: f64) -> i32 {
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

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

fn is_equal_float(a: f64, b: f64) -> bool {
    (a - b).abs() < crate::tuple::EPSILON
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

pub fn rgb(r: i32, g: i32, b: i32) -> Color {
    color(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
}

pub fn white() -> Color {
    color(1.0, 1.0, 1.0)
}

pub fn black() -> Color {
    color(0.0, 0.0, 0.0)
}

#[cfg(test)]
mod color_test {
    use crate::color::color;

    #[test]
    pub fn create() {
        let c = color(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    pub fn equality() {
        assert_eq!(color(-0.5, 0.4, 1.7), color(-0.5, 0.4, 1.7));

        assert_ne!(color(-0.6, 0.4, 1.7), color(-0.5, 0.4, 1.7));
    }

    #[test]
    pub fn addition() {
        assert_eq!(
            color(0.9, 0.6, 0.75) + color(0.7, 0.1, 0.25),
            color(1.6, 0.7, 1.0)
        )
    }

    #[test]
    pub fn subtraction() {
        assert_eq!(
            color(0.9, 0.6, 0.75) - color(0.7, 0.1, 0.25),
            color(0.2, 0.5, 0.5)
        )
    }

    #[test]
    pub fn negation() {
        assert_eq!(-color(0.2, 0.5, -0.5), color(-0.2, -0.5, 0.5));
    }

    #[test]
    pub fn multiplication_by_scalar() {
        assert_eq!(color(0.2, 0.3, 0.4) * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    pub fn multiplication() {
        assert_eq!(
            color(1.0, 0.2, 0.4) * color(0.9, 1.0, 0.1),
            color(0.9, 0.2, 0.04)
        );
    }
}
