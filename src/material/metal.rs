use crate::{Color, HitRecord, Material, Random, Ray, Vec3};

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
            r_in.time,
        );
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}
