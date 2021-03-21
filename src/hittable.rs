pub mod aabb;
pub mod box_obj;
pub mod bvh;
pub mod constant_medium;
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
pub use constant_medium::ConstantMedium;
pub use hittable_list::HittableList;
pub use rect::{XYRect, XZRect, YZRect};
pub use sphere::{MovingSphere, Sphere};
pub use transform::{FlipFace, RotateY, Translate};
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

#[derive(Clone)]
pub enum HittableEnum {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
    BoxObj(BoxObj),
    Triangle(Triangle),
    FlipFace(FlipFace),
    RotateY(RotateY),
    Translate(Translate),
    HittableList(HittableList),
    BvhNode(BvhNode),
}

impl Hittable for HittableEnum {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord> {
        match self {
            HittableEnum::Sphere(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::MovingSphere(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::XYRect(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::XZRect(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::YZRect(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::BoxObj(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::Triangle(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::FlipFace(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::RotateY(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::Translate(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::HittableList(o) => o.hit(r, t_min, t_max, rng),
            HittableEnum::BvhNode(o) => o.hit(r, t_min, t_max, rng),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self {
            HittableEnum::Sphere(o) => o.bounding_box(time0, time1),
            HittableEnum::MovingSphere(o) => o.bounding_box(time0, time1),
            HittableEnum::XYRect(o) => o.bounding_box(time0, time1),
            HittableEnum::XZRect(o) => o.bounding_box(time0, time1),
            HittableEnum::YZRect(o) => o.bounding_box(time0, time1),
            HittableEnum::BoxObj(o) => o.bounding_box(time0, time1),
            HittableEnum::Triangle(o) => o.bounding_box(time0, time1),
            HittableEnum::FlipFace(o) => o.bounding_box(time0, time1),
            HittableEnum::RotateY(o) => o.bounding_box(time0, time1),
            HittableEnum::Translate(o) => o.bounding_box(time0, time1),
            HittableEnum::HittableList(o) => o.bounding_box(time0, time1),
            HittableEnum::BvhNode(o) => o.bounding_box(time0, time1),
        }
    }
}

impl HittableEnum {
    pub fn sphere(center: Point3, radius: f64, mat_ptr: MaterialPtr) -> HittablePtr {
        Arc::new(HittableEnum::Sphere(Sphere::new(center, radius, mat_ptr)))
    }
    pub fn moving_sphere(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: MaterialPtr,
    ) -> HittablePtr {
        Arc::new(HittableEnum::MovingSphere(MovingSphere::new(
            center0, center1, time0, time1, radius, mat_ptr,
        )))
    }
    pub fn xy_rect(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: MaterialPtr,
    ) -> HittablePtr {
        Arc::new(HittableEnum::XYRect(XYRect::new(
            x0, x1, y0, y1, k, material,
        )))
    }
    pub fn xz_rect(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: MaterialPtr,
    ) -> HittablePtr {
        Arc::new(HittableEnum::XZRect(XZRect::new(
            x0, x1, z0, z1, k, material,
        )))
    }
    pub fn yz_rect(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: MaterialPtr,
    ) -> HittablePtr {
        Arc::new(HittableEnum::YZRect(YZRect::new(
            y0, y1, z0, z1, k, material,
        )))
    }
    pub fn box_obj(p0: Point3, p1: Point3, mat: MaterialPtr) -> HittablePtr {
        Arc::new(HittableEnum::BoxObj(BoxObj::new(p0, p1, mat)))
    }
    pub fn triangle(p0: Point3, p1: Vec3, p2: Vec3, material: MaterialPtr) -> HittablePtr {
        Arc::new(HittableEnum::Triangle(Triangle::new(p0, p1, p2, material)))
    }
    pub fn flip_face(obj: HittablePtr) -> HittablePtr {
        Arc::new(HittableEnum::FlipFace(FlipFace::new(obj)))
    }
    pub fn rotate_y(obj: HittablePtr, theta: f64) -> HittablePtr {
        Arc::new(HittableEnum::RotateY(RotateY::new(obj, theta)))
    }
    pub fn translate(obj: HittablePtr, offset: Vec3) -> HittablePtr {
        Arc::new(HittableEnum::Translate(Translate::new(obj, offset)))
    }
    pub fn hittable_list(o: HittableList) -> HittablePtr {
        Arc::new(HittableEnum::HittableList(o))
    }
    pub fn bvh_node(o: BvhNode) -> HittablePtr {
        Arc::new(HittableEnum::BvhNode(o))
    }
}

impl Default for HittableEnum {
    fn default() -> Self {
        HittableEnum::HittableList(Default::default())
    }
}

pub type HittablePtr = Arc<HittableEnum>;
