use std::sync::Arc;

use super::Material;

use crate::texture::SolidColor;
use crate::{Color, HitRecord, TexturePtr};

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
    fn emmitted(&self, rec: &HitRecord) -> Color {
        if rec.front_face {
            self.emit.value(rec.u, rec.v, &rec.p)
        } else {
            Color::default()
        }
    }
}
