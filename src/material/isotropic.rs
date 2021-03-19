use std::{f64::consts::PI, sync::Arc};

use crate::{pdf::UniformSpherePdf, HitRecord, Material, Random, Ray, TexturePtr};

use super::ScatterRecord;

#[derive(Clone)]
pub struct Isotropic {
    albedo: TexturePtr,
}

impl Isotropic {
    pub fn new(albedo: impl Into<TexturePtr>) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _: &Ray, rec: &HitRecord, _: &mut Random) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_scatter(
            self.albedo.value(rec.u, rec.v, &rec.p),
            Arc::new(UniformSpherePdf::new()),
        ))
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
