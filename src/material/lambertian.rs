use std::sync::Arc;

use crate::texture::SolidColor;
use crate::{Color, HitRecord, Material, Random, Ray, TexturePtr};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray, f64)> {
        let scatter_direction = rec.normal.random_in_hemisphere(rng).unit_vector();
        let pdf = 0.5 / std::f64::consts::PI;
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p.clone(), scatter_direction, r_in.time),
            pdf,
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
