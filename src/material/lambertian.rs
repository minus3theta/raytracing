use std::sync::Arc;

use crate::texture::SolidColor;
use crate::{Color, HitRecord, Material, Onb, Random, Ray, TexturePtr, Vec3};

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
        let uvw = Onb::new(&rec.normal);
        let direction = uvw.local_vec(&Vec3::random_cosine_direction(rng));
        let pdf = uvw.w.dot(&direction) / std::f64::consts::PI;

        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p.clone(), direction, r_in.time),
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
