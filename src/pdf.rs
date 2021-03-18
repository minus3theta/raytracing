use std::f64::consts::PI;

use crate::{hittable::EmittablePtr, Onb, Point3, Random, Vec3};

pub trait Pdf {
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64;
    fn generate(&self, rng: &mut Random) -> Vec3;
}

#[derive(Clone)]
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3, _: &mut Random) -> f64 {
        let cosine = direction.unit_vector().dot(&self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        self.uvw.local_vec(&Vec3::random_cosine_direction(rng))
    }
}

#[derive(Clone)]
pub struct EmittablePdf {
    obj: EmittablePtr,
    o: Point3,
}

impl EmittablePdf {
    pub fn new(obj: EmittablePtr, o: Point3) -> Self {
        Self { obj, o }
    }
}

impl Pdf for EmittablePdf {
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64 {
        self.obj.pdf_value(&self.o, direction, rng)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        self.obj.random(&self.o, rng)
    }
}
