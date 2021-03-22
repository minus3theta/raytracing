pub mod checker;
pub mod image_texture;
pub mod noise;
pub mod solid_color;

pub use checker::Checker;
pub use image_texture::ImageTexture;
pub use noise::{Marble, NoiseTexture, Turbulence};
pub use solid_color::SolidColor;

use std::{path::Path, sync::Arc};

use crate::{Color, Point3, Random};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Clone)]
pub enum TextureEnum {
    Checker(Checker),
    Image(ImageTexture),
    Marble(Marble),
    Noise(NoiseTexture),
    Turbulence(Turbulence),
    SolidColor(SolidColor),
}

impl Texture for TextureEnum {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            TextureEnum::Checker(t) => t.value(u, v, p),
            TextureEnum::Image(t) => t.value(u, v, p),
            TextureEnum::Marble(t) => t.value(u, v, p),
            TextureEnum::Noise(t) => t.value(u, v, p),
            TextureEnum::Turbulence(t) => t.value(u, v, p),
            TextureEnum::SolidColor(t) => t.value(u, v, p),
        }
    }
}

impl TextureEnum {
    pub fn checker(even: impl Into<TexturePtr>, odd: impl Into<TexturePtr>) -> TexturePtr {
        Arc::new(TextureEnum::Checker(Checker::new(even, odd)))
    }

    pub fn image(path: impl AsRef<Path>) -> anyhow::Result<TexturePtr> {
        Ok(Arc::new(TextureEnum::Image(ImageTexture::new(path)?)))
    }

    pub fn marble(scale: f64, rng: &mut Random) -> TexturePtr {
        Arc::new(TextureEnum::Marble(Marble::with_rng(scale, rng)))
    }

    pub fn noise(scale: f64, rng: &mut Random) -> TexturePtr {
        Arc::new(TextureEnum::Noise(NoiseTexture::with_rng(scale, rng)))
    }

    pub fn turblence(scale: f64, rng: &mut Random) -> TexturePtr {
        Arc::new(TextureEnum::Turbulence(Turbulence::with_rng(scale, rng)))
    }

    pub fn solid_color(color_value: Color) -> TexturePtr {
        Arc::new(TextureEnum::SolidColor(SolidColor::new(color_value)))
    }
}

pub type TexturePtr = Arc<TextureEnum>;
