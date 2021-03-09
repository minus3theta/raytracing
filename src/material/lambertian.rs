use crate::{Color, HitRecord, Material, Random, Ray, Vec3};

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
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
            self.albedo.clone(),
            Ray::new(rec.p.clone(), scatter_direction, r_in.time),
        ))
    }
}
