use super::vec3::{Point3, Vec3};

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Self {
        Self { orig, dir, time }
    }
    pub fn at(&self, t: f64) -> Point3 {
        &self.orig + &(t * &self.dir)
    }
}
