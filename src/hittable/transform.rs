use std::sync::Arc;

use crate::{Point3, Random, Ray, Vec3};

use super::{Aabb, HitRecord, Hittable, HittablePtr};

#[derive(Clone)]
pub struct Translate {
    obj: HittablePtr,
    offset: Vec3,
}

impl Translate {
    pub fn new(obj: HittablePtr, offset: Vec3) -> Self {
        Self { obj, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord> {
        let moved_r = Ray::new(&r.orig - &self.offset, r.dir.clone(), r.time);
        let rec = self.obj.hit(&moved_r, t_min, t_max, rng)?;
        Some(HitRecord {
            p: &rec.p + &self.offset,
            ..rec
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let Aabb { minimum, maximum } = self.obj.bounding_box(time0, time1)?;
        Some(Aabb::new(minimum + &self.offset, maximum + &self.offset))
    }
}

#[derive(Clone)]
pub struct RotateY {
    obj: HittablePtr,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(obj: HittablePtr, theta: f64) -> Self {
        let theta = theta.to_radians();

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let bbox = obj.bounding_box(0.0, 1.0).map(|bb| {
            let inf = f64::INFINITY;
            let mut min = Point3::new(inf, bb.minimum.y, inf);
            let mut max = Point3::new(-inf, bb.maximum.y, -inf);

            for &x in &[bb.minimum.x, bb.maximum.x] {
                for &z in &[bb.minimum.z, bb.maximum.z] {
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    min.x = min.x.min(newx);
                    max.x = max.x.max(newx);
                    min.z = min.z.min(newz);
                    max.z = max.z.max(newz);
                }
            }

            Aabb::new(min, max)
        });

        Self {
            obj,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord> {
        let inv = |v: &Vec3| {
            Vec3::new(
                self.cos_theta * v.x - self.sin_theta * v.z,
                v.y,
                self.sin_theta * v.x + self.cos_theta * v.z,
            )
        };

        let rotated_r = Ray::new(inv(&r.orig), inv(&r.dir), r.time);
        let rec = self.obj.hit(&rotated_r, t_min, t_max, rng)?;

        let rot = |v: &Vec3| {
            Vec3::new(
                self.cos_theta * v.x + self.sin_theta * v.z,
                v.y,
                -self.sin_theta * v.x + self.cos_theta * v.z,
            )
        };
        Some(HitRecord {
            p: rot(&rec.p),
            normal: rot(&rec.normal),
            ..rec
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        self.bbox.clone()
    }
}

pub fn translate(obj: HittablePtr, offset: Vec3) -> HittablePtr {
    Arc::new(Translate::new(obj, offset))
}

pub fn rotate_y(obj: HittablePtr, theta: f64) -> HittablePtr {
    Arc::new(RotateY::new(obj, theta))
}
