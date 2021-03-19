use std::sync::Arc;

use crate::{pdf::PdfPtr, Color, HitRecord, Random, Ray};

#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scatter: ScatterType,
}

#[derive(Clone)]
pub enum ScatterType {
    Specular(Ray),
    Pdf(PdfPtr),
}

impl ScatterRecord {
    pub fn new_specular(attenuation: Color, specular_ray: Ray) -> Self {
        Self {
            attenuation,
            scatter: ScatterType::Specular(specular_ray),
        }
    }

    pub fn new_scatter(attenuation: Color, pdf: PdfPtr) -> Self {
        Self {
            attenuation,
            scatter: ScatterType::Pdf(pdf),
        }
    }
}

pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _rng: &mut Random) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn emmitted(&self, _r_in: &Ray, _rec: &HitRecord) -> Color {
        Color::default()
    }
}

pub type MaterialPtr = Arc<dyn Material + Send + Sync>;

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;
