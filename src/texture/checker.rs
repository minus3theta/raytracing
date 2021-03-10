use std::sync::Arc;

use crate::{Color, Point3};

use super::{SolidColor, Texture, TexturePtr};

#[derive(Clone)]
pub struct Checker {
    even: TexturePtr,
    odd: TexturePtr,
}

impl Checker {
    pub fn new(even: TexturePtr, odd: TexturePtr) -> Self {
        Self { even, odd }
    }
    pub fn with_color(even: Color, odd: Color) -> Self {
        Self::new(
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
