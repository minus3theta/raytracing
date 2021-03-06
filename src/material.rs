use std::rc::Rc;

use crate::{Color, HitRecord, Random, Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)>;
}

pub type MaterialPtr = Rc<dyn Material>;

pub mod lambertian;
pub mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;
