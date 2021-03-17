use std::sync::Arc;

use crate::{Color, HitRecord, Point3, Random, Ray};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _rng: &mut Random,
    ) -> Option<(Color, Ray, f64)> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn emmitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

pub type MaterialPtr = Arc<dyn Material + Send + Sync>;

// pub mod dielectric;
pub mod diffuse_light;
// pub mod isotropic;
pub mod lambertian;
// pub mod metal;

// pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
// pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
// pub use metal::Metal;
