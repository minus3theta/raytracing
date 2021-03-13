use crate::{Color, HitRecord, Material, Random, Ray, TexturePtr, Vec3};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p.clone(), Vec3::random_in_unit_sphere(rng), r_in.time),
        ))
    }
}
