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

    pub fn mix(self, other: PdfPtr, ratio: f64) -> PdfPtr {
        if self.obj.is_valid() {
            Arc::new(MixturePdf::new(Arc::new(self), other, ratio))
        } else {
            other
        }
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
    ratio: f64,
}

impl MixturePdf {
    pub fn new(p0: PdfPtr, p1: PdfPtr, ratio: f64) -> Self {
        Self { p0, p1, ratio }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64 {
        self.ratio * self.p0.value(direction, rng)
            + (1.0 - self.ratio) * self.p1.value(direction, rng)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        if rng.unit_f64() < self.ratio {
            self.p0.generate(rng)
        } else {
            self.p1.generate(rng)
        }
    }
}

#[derive(Clone)]
pub struct UniformSpherePdf {}

impl UniformSpherePdf {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pdf for UniformSpherePdf {
    fn value(&self, _: &Vec3, _: &mut Random) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        Vec3::random_in_unit_sphere(rng)
    }
}
