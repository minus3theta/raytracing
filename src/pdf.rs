use std::{f64::consts::PI, sync::Arc};

use crate::{hittable::EmittablePtr, Onb, Point3, Random, Vec3};

pub trait Pdf {
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64;
    fn generate(&self, rng: &mut Random) -> Vec3;
}

pub type PdfPtr = Arc<dyn Pdf + Send + Sync>;

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

#[derive(Clone)]
pub struct MixturePdf {
    pub p0: PdfPtr,
    pub p1: PdfPtr,
}

impl MixturePdf {
    pub fn new(p0: PdfPtr, p1: PdfPtr) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64 {
        0.5 * self.p0.value(direction, rng) + 0.5 * self.p1.value(direction, rng)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        if rng.unit_f64() < 0.5 {
            self.p0.generate(rng)
        } else {
            self.p1.generate(rng)
        }
    }
}
