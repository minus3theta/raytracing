use std::sync::Arc;

use super::Material;

use crate::texture::SolidColor;
use crate::{Color, HitRecord, Point3, Random, Ray, TexturePtr};

#[derive(Clone)]
pub struct DiffuseLight {
    emit: TexturePtr,
}

impl DiffuseLight {
    pub fn new(emit: TexturePtr) -> Self {
        Self { emit }
    }

    pub fn with_color(color: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(color)))
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
