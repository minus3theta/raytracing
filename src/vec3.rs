use std::ops;

use crate::Random;

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
    pub fn random(rng: &mut Random, min: f64, max: f64) -> Self {
        Self::new(
            rng.range_f64(min, max),
            rng.range_f64(min, max),
            rng.range_f64(min, max),
        )
    }
    pub fn random_in_unit_sphere(rng: &mut Random) -> Self {
        loop {
            let p = Self::random(rng, -1.0, 1.0);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
    pub fn random_unit_vector(rng: &mut Random) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
    }
}

impl_op_ex!(-|v: &Vec3| -> Vec3 { Vec3::new(-v.x, -v.y, -v.z) });

impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(+=|a: &mut Vec3, b: &Vec3| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(-=|a: &mut Vec3, b: &Vec3| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });
impl_op_ex!(*=|a: &mut Vec3, b: &Vec3| {
    a.x *= b.x;
    a.y *= b.y;
    a.z *= b.z;
});

impl_op_ex_commutative!(*|v: &Vec3, t: f64| -> Vec3 { Vec3::new(v.x * t, v.y * t, v.z * t) });
impl_op_ex!(*=|v: &mut Vec3, t: f64| {
    v.x *= t;
    v.y *= t;
    v.z *= t;
});

impl_op_ex!(/|v: &Vec3, t: f64| -> Vec3 { v * (1.0/t) });
impl_op_ex!(/=|v: &mut Vec3, t: f64| {
    v.x /= t;
    v.y /= t;
    v.z /= t;
});

pub type Point3 = Vec3;
