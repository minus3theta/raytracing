use crate::{MaterialPtr, Point3, Random, Ray};

use super::{Aabb, HitRecord, Hittable, HittableEnum, HittableList};

#[derive(Clone)]
pub struct BoxObj {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl BoxObj {
    pub fn new(p0: Point3, p1: Point3, mat: MaterialPtr) -> Self {
        let mut sides = HittableList::default();

        sides.add(HittableEnum::xy_rect(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            mat.clone(),
        ));
        sides.add(HittableEnum::xy_rect(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            mat.clone(),
        ));

        sides.add(HittableEnum::xz_rect(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            mat.clone(),
        ));
        sides.add(HittableEnum::xz_rect(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            mat.clone(),
        ));

        sides.add(HittableEnum::yz_rect(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            mat.clone(),
        ));
        sides.add(HittableEnum::yz_rect(p0.y, p1.y, p0.z, p1.z, p0.x, mat));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for BoxObj {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max, rng)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min.clone(), self.box_max.clone()))
    }
}
