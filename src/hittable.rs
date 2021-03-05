pub mod hittable_list;
pub mod sphere;

use super::{Point3, Ray, Vec3};

pub use hittable_list::HittableList;
pub use sphere::Sphere;

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64) -> Self {
        Self {
            p,
            t,
            ..Default::default()
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}