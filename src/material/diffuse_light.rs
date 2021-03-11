use super::Material;
use crate::{Color, HitRecord, Point3, Random, Ray, TexturePtr};

#[derive(Clone)]
pub struct DiffuseLight {
    emit: TexturePtr,
}

impl DiffuseLight {
    pub fn new(emit: TexturePtr) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Random) -> Option<(Color, Ray)> {
        None
    }

    fn emmitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
