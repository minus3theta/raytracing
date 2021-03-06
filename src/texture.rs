pub mod checker;
pub mod image_texture;
pub mod noise;
pub mod solid_color;

pub use checker::Checker;
pub use image_texture::ImageTexture;
pub use noise::{Marble, NoiseTexture, Turbulence};
pub use solid_color::SolidColor;

use std::sync::Arc;

use crate::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub type TexturePtr = Arc<dyn Texture + Send + Sync>;
