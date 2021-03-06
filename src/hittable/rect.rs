use crate::{MaterialPtr, Point3, Random, Ray, Vec3};

use super::{Aabb, HitRecord, Hittable};

const EPS: f64 = 0.0001;

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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _: &mut Random) -> Option<HitRecord> {
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
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - EPS),
            Point3::new(self.x1, self.y1, self.k + EPS),
        ))
    }
}

#[derive(Clone)]
pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: MaterialPtr,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: MaterialPtr) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _: &mut Random) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            r,
            Vec3::new(0.0, 1.0, 0.0),
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - EPS, self.z0),
            Point3::new(self.x1, self.k + EPS, self.z1),
        ))
    }
}

#[derive(Clone)]
pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: MaterialPtr,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: MaterialPtr) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _: &mut Random) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            r,
            Vec3::new(1.0, 0.0, 0.0),
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.k - EPS, self.y0, self.z0),
            Point3::new(self.k + EPS, self.y1, self.z1),
        ))
    }
}
