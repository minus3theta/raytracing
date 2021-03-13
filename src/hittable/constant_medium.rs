use std::sync::Arc;

use super::{Aabb, HitRecord, Hittable, HittablePtr};
use crate::material::Isotropic;
use crate::{MaterialPtr, Random, Ray, TexturePtr, Vec3};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: HittablePtr,
    phase_function: MaterialPtr,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: HittablePtr, density: f64, texture: impl Into<TexturePtr>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(texture.into())),
            neg_inv_density: (-1.0 / density),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord> {
        let inf = f64::INFINITY;
        let eps = 0.0001;

        let rec1 = self.boundary.hit(r, -inf, inf, rng)?;
        let rec2 = self.boundary.hit(r, rec1.t + eps, inf, rng)?;

        let t1 = rec1.t.max(t_min);
        let t2 = rec2.t.min(t_max);
        if t1 >= t2 {
            return None;
        }
        let t1 = t1.max(0.0);

        let ray_length = r.dir.length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * rng.unit_f64().ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t1 + hit_distance / ray_length;
        let p = r.at(t);
        Some(HitRecord::new(
            p,
            t,
            0.0,
            0.0,
            r,
            Vec3::new(1.0, 0.0, 0.0),
            self.phase_function.clone(),
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
