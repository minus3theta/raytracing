use std::path::Path;

use anyhow::Context;
use image::io::Reader as ImageReader;
use image::{DynamicImage, Rgb, RgbImage};

use super::{Texture, TextureEnum};
use crate::{Color, Point3, TexturePtr};

#[derive(Debug, Clone)]
pub struct ImageTexture {
    img: RgbImage,
}

impl ImageTexture {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let img = ImageReader::open(path)?.decode()?;
        if let DynamicImage::ImageRgb8(img) = img {
            Ok(Self { img })
        } else {
            None.context("Not an RgbImage")
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = ((u * self.img.width() as f64) as u32).min(self.img.width());
        let j = ((v * self.img.height() as f64) as u32).min(self.img.height());

        let color_scale = 1.0 / 255.0;
        let &Rgb([r, g, b]) = self.img.get_pixel(i, j);
        Color::new(
            r as f64 * color_scale,
            g as f64 * color_scale,
            b as f64 * color_scale,
        )
    }
}

impl Into<TextureEnum> for ImageTexture {
    fn into(self) -> TextureEnum {
        TextureEnum::Image(self)
    }
}

impl Into<TexturePtr> for ImageTexture {
    fn into(self) -> TexturePtr {
        Into::<TextureEnum>::into(self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_jpeg() -> anyhow::Result<()> {
        let img = ImageReader::open("res/earthmap.jpg")?.decode()?;
        assert!(matches!(img, DynamicImage::ImageRgb8(_)));
        Ok(())
    }
}
