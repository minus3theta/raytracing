use std::f64::consts::PI;

use crate::{emittable::EmittableEnum, Emittable, Onb, Point3, Random, Vec3};

#[derive(Clone)]
pub enum PdfEnum<'a> {
    Cosine(CosinePdf),
    UniformSphere(UniformSpherePdf),
    Emittable(EmittablePdf<'a, EmittableEnum>),
    Mixture(MixturePdf<'a, EmittablePdf<'a, EmittableEnum>, PdfEnum<'a>>),
}

impl<'a> Pdf for PdfEnum<'a> {
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64 {
        match self {
            PdfEnum::Cosine(p) => p.value(direction, rng),
            PdfEnum::UniformSphere(p) => p.value(direction, rng),
            PdfEnum::Emittable(p) => p.value(direction, rng),
            PdfEnum::Mixture(p) => p.value(direction, rng),
        }
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        match self {
            PdfEnum::Cosine(p) => p.generate(rng),
            PdfEnum::UniformSphere(p) => p.generate(rng),
            PdfEnum::Emittable(p) => p.generate(rng),
            PdfEnum::Mixture(p) => p.generate(rng),
        }
    }
}

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
pub struct EmittablePdf<'a, T> {
    obj: &'a T,
    o: Point3,
}

impl<'a, T> EmittablePdf<'a, T> {
    pub fn new(obj: &'a T, o: Point3) -> Self {
        Self { obj, o }
    }
}
impl<'a> EmittablePdf<'a, EmittableEnum> {
    pub fn mix(&'a self, other: &'a PdfEnum, ratio: f64) -> PdfEnum<'a> {
        let ratio = if self.obj.is_valid() { ratio } else { 0.0 };
        PdfEnum::Mixture(MixturePdf::new(self, other, ratio))
    }
}

impl<'a, T> Pdf for EmittablePdf<'a, T>
where
    T: Emittable,
{
    fn value(&self, direction: &Vec3, rng: &mut Random) -> f64 {
        self.obj.pdf_value(&self.o, direction, rng)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        self.obj.random(&self.o, rng)
    }
}

#[derive(Clone)]
pub struct MixturePdf<'a, T, U> {
    pub p0: &'a T,
    pub p1: &'a U,
    ratio: f64,
}

impl<'a, T, U> MixturePdf<'a, T, U> {
    pub fn new(p0: &'a T, p1: &'a U, ratio: f64) -> Self {
        Self { p0, p1, ratio }
    }
}

impl<'a, T, U> Pdf for MixturePdf<'a, T, U>
where
    T: Pdf,
    U: Pdf,
{
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
