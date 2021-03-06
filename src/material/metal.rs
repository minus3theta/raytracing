use crate::{Color, HitRecord, Material, Random, Ray};

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, _rng: &mut Random) -> Option<(Color, Ray)> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected);
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}
