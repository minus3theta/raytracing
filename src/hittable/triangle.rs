use crate::{algebra::solve_equation, MaterialPtr, Point3, Random, Ray, Vec3};

use super::{Aabb, HitRecord, Hittable};

const EPS: f64 = 0.0001;

#[derive(Clone)]
pub struct Triangle {
    p0: Point3,
    a: Vec3,
    b: Vec3,
    normal: Vec3,
    material: MaterialPtr,
    bb: Aabb,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Vec3, p2: Vec3, material: MaterialPtr) -> Self {
        let a = &p1 - &p0;
        let b = &p2 - &p0;
        let normal = a.cross(&b).unit_vector();

        let b0 = Aabb::new(p0.clone(), p0.clone());
        let b1 = Aabb::new(p1.clone(), p1);
        let b2 = Aabb::new(p2.clone(), p2);

        let mut bb = b0.surrounding_box(&b1).surrounding_box(&b2);
        bb.wrap(EPS);

        Self {
            p0,
            a,
            b,
            normal,
            material,
            bb,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _: &mut Random) -> Option<HitRecord> {
        let coef = Vec3::to_matrix(&r.dir, &-&self.a, &-&self.b);
        let rhs = &self.p0 - &r.orig;
        let rhs = rhs.into();
        let tuv = solve_equation(coef, rhs);
        let t = tuv[0];
        if t < t_min || t > t_max {
            return None;
        }
        let u = tuv[1];
        let v = tuv[2];
        if u < 0.0 || v < 0.0 || u + v > 1.0 {
            return None;
        }
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            r,
            self.normal.clone(),
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.bb.clone())
    }
}
