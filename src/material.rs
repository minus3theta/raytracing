use std::rc::Rc;

use crate::{Color, HitRecord, Random, Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)>;
}

pub type MaterialPtr = Rc<dyn Material>;

pub mod lambertian;

pub use lambertian::Lambertian;
