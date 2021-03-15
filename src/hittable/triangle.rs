use mathru::{
    algebra::linear::{matrix::Solve, Matrix, Vector},
    matrix, vector,
};

use crate::{MaterialPtr, Point3, Random, Ray, Vec3};

use super::{Aabb, HitRecord, Hittable};

const EPS: f64 = 0.0001;

#[derive(Clone)]
pub struct Triangle {
    p0: Point3,
    a: Vec3,
    b: Vec3,
    normal: Vec3,
    material: MaterialPtr,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Vec3, p2: Vec3, material: MaterialPtr) -> Self {
        let a = p1 - &p0;
        let b = p2 - &p0;
        let normal = a.cross(&b).unit_vector();
        Self {
            p0,
            a,
            b,
            normal,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _: &mut Random) -> Option<HitRecord> {
        let coef = matrix![
            r.dir.x, -self.a.x, -self.b.x;
            r.dir.y, -self.a.y, -self.b.y;
            r.dir.z, -self.a.z, -self.b.z
        ];
        let rhs = &self.p0 - &r.orig;
        let rhs = vector![rhs.x; rhs.y; rhs.z];
        let tuv = coef.solve(&rhs).ok()?;
        let t = *tuv.get(0);
        if t < t_min || t > t_max {
            return None;
        }
        let u = *tuv.get(1);
        let v = *tuv.get(2);
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
        let b0 = Aabb::new(self.p0.clone(), self.p0.clone());
        let p1 = &self.p0 + &self.a;
        let b1 = Aabb::new(p1.clone(), p1);
        let p2 = &self.p0 + &self.b;
        let b2 = Aabb::new(p2.clone(), p2);

        let bb = b0.surrounding_box(&b1).surrounding_box(&b2);
        let eps = Vec3::new(EPS, EPS, EPS);
        Some(Aabb::new(&bb.minimum - &eps, &bb.maximum + &eps))
    }
}
