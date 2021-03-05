use crate::Point3;

use super::{HitRecord, Hittable};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.orig - &self.center;
        let a = r.dir.dot(&r.dir);
        let half_b = oc.dot(&r.dir);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let filter_root = |root: f64| {
            if t_min <= root && root <= t_max {
                Some(root)
            } else {
                None
            }
        };
        let root =
            filter_root((-half_b - sqrtd) / a).or_else(|| filter_root((-half_b + sqrtd) / a))?;
        let t = root;
        let p = r.at(t);
        let outward_normal = (&p - &self.center) / self.radius;
        Some(HitRecord::new(p, t, r, outward_normal))
    }
}
