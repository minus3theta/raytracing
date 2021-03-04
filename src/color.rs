use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};

use super::vec3::Vec3;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone, Add, AddAssign, Neg, Sub, SubAssign)]
pub struct Color(Vec3);

const RGB_SCALE: f64 = 255.999;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }
    pub fn r(&self) -> u8 {
        (RGB_SCALE * self.0.x) as u8
    }
    pub fn g(&self) -> u8 {
        (RGB_SCALE * self.0.y) as u8
    }
    pub fn b(&self) -> u8 {
        (RGB_SCALE * self.0.z) as u8
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r(), self.g(), self.b())
    }
}

impl std::ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color(self * rhs.0)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}
