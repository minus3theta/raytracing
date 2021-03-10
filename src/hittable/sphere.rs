use crate::{Point3, Vec3};

use super::{Aabb, HitRecord, Hittable, MaterialPtr};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: MaterialPtr,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: MaterialPtr) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        shpere_hit(&self.center, self.radius, &self.mat_ptr, r, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let v = Vec3::new(self.radius, self.radius, self.radius);
        Some(Aabb::new(&self.center - &v, &self.center + &v))
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    center0: Point3,
    move_vec: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: MaterialPtr,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: MaterialPtr,
    ) -> Self {
        let move_vec = &center1 - &center0;
        Self {
            center0,
            move_vec,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }
    fn center(&self, time: f64) -> Point3 {
        &self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * &self.move_vec
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        shpere_hit(
            &self.center(r.time),
            self.radius,
            &self.mat_ptr,
            r,
            t_min,
            t_max,
        )
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let c0 = self.center(time0);
        let c1 = self.center(time1);
        let v = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = Aabb::new(&c0 - &v, &c0 + &v);
        let box1 = Aabb::new(&c1 - &v, &c1 + &v);
        Some(box0.surrounding_box(&box1))
    }
}

fn shpere_hit(
    center: &Point3,
    radius: f64,
    mat_ptr: &MaterialPtr,
    r: &crate::Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord> {
    let oc = &r.orig - center;
    let a = r.dir.dot(&r.dir);
    let half_b = oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius.powi(2);
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
    let root = filter_root((-half_b - sqrtd) / a).or_else(|| filter_root((-half_b + sqrtd) / a))?;
    let t = root;
    let p = r.at(t);
    let outward_normal = (&p - center) / radius;
    let (u, v) = get_sphere_uv(&outward_normal);
    Some(HitRecord::new(
        p,
        t,
        u,
        v,
        r,
        outward_normal,
        mat_ptr.clone(),
    ))
}

fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    use std::f64::consts::{PI, TAU};
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;

    (phi / TAU, theta / PI)
}
