use std::sync::Arc;

use crate::{texture::SolidColor, CosinePdf};
use crate::{Color, HitRecord, Material, Random, Ray, TexturePtr};

use super::ScatterRecord;

#[derive(Clone)]
pub struct Lambertian {
    albedo: TexturePtr,
}

impl Lambertian {
    pub fn new(albedo: TexturePtr) -> Self {
        Self { albedo }
    }
    pub fn with_color(albedo: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, _: &mut Random) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_scatter(
            self.albedo.value(rec.u, rec.v, &rec.p),
            Arc::new(CosinePdf::new(&rec.normal)),
        ))
    }

    fn scattering_pdf(&self, _: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal.dot(&scattered.dir.unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}
