use crate::{Point3, Ray};

#[derive(Debug, Clone)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let mut update = |orig: f64, dir: f64, min: f64, max: f64| {
            let inv_d = 1.0 / dir;
            let mut t0 = (min - orig) * inv_d;
            let mut t1 = (max - orig) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            !(t_max <= t_min)
        };
        update(r.orig.x, r.dir.x, self.minimum.x, self.maximum.x)
            && update(r.orig.y, r.dir.y, self.minimum.y, self.maximum.y)
            && update(r.orig.z, r.dir.z, self.minimum.z, self.maximum.z)
    }
    pub fn surrounding_box(&self, other: &Self) -> Self {
        let minimum = Point3::new(
            self.minimum.x.min(other.minimum.x),
            self.minimum.y.min(other.minimum.y),
            self.minimum.z.min(other.minimum.z),
        );
        let maximum = Point3::new(
            self.maximum.x.max(other.maximum.x),
            self.maximum.y.max(other.maximum.y),
            self.maximum.z.max(other.maximum.z),
        );
        Self::new(minimum, maximum)
    }
    pub fn merge(self, other: &Option<Self>) -> Option<Self> {
        match other {
            Some(other) => Some(self.surrounding_box(other)),
            None => Some(self),
        }
    }
}
