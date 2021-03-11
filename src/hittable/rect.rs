use crate::{MaterialPtr, Point3, Ray, Vec3};

use super::{Aabb, HitRecord, Hittable};

#[derive(Clone)]
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: MaterialPtr,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: MaterialPtr) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            r,
            Vec3::new(0.0, 0.0, 1.0),
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let eps = 0.0001;
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - eps),
            Point3::new(self.x1, self.y1, self.k + eps),
        ))
    }
}
