use std::ops;

use crate::{Random, Vec3};

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct Color(pub Vec3);

const RGB_SCALE: f64 = 256.0;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }
    pub fn r(&self) -> u8 {
        (RGB_SCALE * Self::clamp_color(self.0.x.sqrt())) as u8
    }
    pub fn g(&self) -> u8 {
        (RGB_SCALE * Self::clamp_color(self.0.y.sqrt())) as u8
    }
    pub fn b(&self) -> u8 {
        (RGB_SCALE * Self::clamp_color(self.0.z.sqrt())) as u8
    }
    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
    fn clamp_color(x: f64) -> f64 {
        Self::clamp(x, 0.0, 1.0)
    }
    pub fn random(rng: &mut Random) -> Self {
        Self(Vec3::random(rng, 0.0, 1.0))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r(), self.g(), self.b())
    }
}

impl_op_ex!(+|c: &Color, d: &Color| -> Color { Color(&c.0 + &d.0) });
impl_op_ex!(+=|c: &mut Color, d: &Color| { c.0 += &d.0 });

impl_op_ex!(*|c: &Color, d: &Color| -> Color { Color(&c.0 * &d.0) });
impl_op_ex_commutative!(*|c: &Color, t: f64| -> Color { Color(&c.0 * t) });

impl_op_ex!(/|c: &Color, t: f64| -> Color { c * (1.0 / t) });
impl_op_ex!(/=|c: &mut Color, t: f64| { c.0 /= t });
