use std::sync::Arc;

use crate::texture::SolidColor;
use crate::{Color, HitRecord, Material, Random, Ray, TexturePtr, Vec3};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)> {
        let scatter_direction = &rec.normal + Vec3::random_unit_vector(rng);
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal.clone()
        } else {
            scatter_direction
        };
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p.clone(), scatter_direction, r_in.time),
        ))
    }
}
