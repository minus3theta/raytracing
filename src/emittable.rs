use std::sync::Arc;

use crate::{
    hittable::{Sphere, XZRect},
    Point3, Random, Vec3,
};

#[derive(Clone)]
pub enum EmittableEnum {
    XZRect(XZRect),
    Sphere(Sphere),
    List(Vec<EmittableEnum>),
}

impl Emittable for EmittableEnum {
    fn pdf_value(&self, o: &Point3, v: &Vec3, rng: &mut Random) -> f64 {
        match self {
            EmittableEnum::XZRect(e) => e.pdf_value(o, v, rng),
            EmittableEnum::Sphere(e) => e.pdf_value(o, v, rng),
            EmittableEnum::List(e) => e.pdf_value(o, v, rng),
        }
    }

    fn random(&self, o: &Point3, rng: &mut Random) -> Vec3 {
        match self {
            EmittableEnum::XZRect(e) => e.random(o, rng),
            EmittableEnum::Sphere(e) => e.random(o, rng),
            EmittableEnum::List(e) => e.random(o, rng),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            EmittableEnum::XZRect(e) => e.is_valid(),
            EmittableEnum::Sphere(e) => e.is_valid(),
            EmittableEnum::List(e) => e.is_valid(),
        }
    }
}

impl Default for EmittableEnum {
    fn default() -> Self {
        EmittableEnum::List(Default::default())
    }
}

impl From<Vec<EmittableEnum>> for EmittableEnum {
    fn from(v: Vec<EmittableEnum>) -> Self {
        EmittableEnum::List(v)
    }
}

pub trait Emittable {
    fn pdf_value(&self, o: &Point3, v: &Vec3, rng: &mut Random) -> f64;
    fn random(&self, o: &Point3, rng: &mut Random) -> Vec3;
    fn is_valid(&self) -> bool {
        true
    }
}

impl<T: Emittable> Emittable for Vec<T> {
    fn pdf_value(&self, o: &Point3, v: &Vec3, rng: &mut Random) -> f64 {
        self.iter()
            .map(|object| object.pdf_value(o, v, rng))
            .sum::<f64>()
            / self.len() as f64
    }

    fn random(&self, o: &Point3, rng: &mut Random) -> Vec3 {
        let choice = rng.range_usize(0, self.len());
        self[choice].random(o, rng)
    }

    fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

pub type EmittablePtr = Arc<EmittableEnum>;
