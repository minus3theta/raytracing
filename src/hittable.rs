pub mod aabb;
pub mod box_obj;
pub mod bvh;
// pub mod constant_medium;
pub mod hittable_list;
pub mod rect;
pub mod sphere;
pub mod transform;
pub mod triangle;

use std::sync::Arc;

use crate::Random;

use super::{MaterialPtr, Point3, Ray, Vec3};

pub use aabb::Aabb;
pub use box_obj::BoxObj;
pub use bvh::BvhNode;
// pub use constant_medium::ConstantMedium;
pub use hittable_list::HittableList;
pub use rect::{XYRect, XZRect, YZRect};
pub use sphere::{MovingSphere, Sphere};
pub use transform::{rotate_y, translate, RotateY, Translate};
pub use triangle::Triangle;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: MaterialPtr,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
        outward_normal: Vec3,
        mat_ptr: MaterialPtr,
    ) -> Self {
        let front_face = r.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            t,
            u,
            v,
            front_face,
            normal,
            mat_ptr,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub type HittablePtr = Arc<dyn Hittable + Send + Sync>;
