use std::{convert::TryFrom, ops};

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
    pub fn reflect(&self, n: &Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }
    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
        r_out_perp + r_out_parallel
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
    pub fn random_in_unit_disk(rng: &mut Random) -> Self {
        loop {
            let p = Self::new(rng.range_f64(-1.0, 1.0), rng.range_f64(-1.0, 1.0), 0.0);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
    pub fn to_matrix(a: &Self, b: &Self, c: &Self) -> Vec<Vec<f64>> {
        vec![
            vec![a.x, b.x, c.x],
            vec![a.y, b.y, c.y],
            vec![a.z, b.z, c.z],
        ]
    }
}

impl Into<Vec<f64>> for &Vec3 {
    fn into(self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }
}

impl Into<Vec<f64>> for Vec3 {
    fn into(self) -> Vec<f64> {
        (&self).into()
    }
}

impl TryFrom<&Vec<f64>> for Vec3 {
    type Error = ();

    fn try_from(value: &Vec<f64>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(());
        }
        Ok(Self::new(value[0], value[1], value[2]))
    }
}

impl TryFrom<Vec<f64>> for Vec3 {
    type Error = ();

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
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
