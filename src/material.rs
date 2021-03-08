use std::sync::Arc;

use crate::{Color, HitRecord, Random, Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)>;
}

pub type MaterialPtr = Arc<dyn Material + Send + Sync>;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
