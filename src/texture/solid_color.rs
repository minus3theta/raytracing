use super::Texture;
use crate::{Color, Point3};

#[derive(Debug, Clone, Default)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
        self.color_value.clone()
    }
}

impl From<Color> for SolidColor {
    fn from(color_value: Color) -> Self {
        Self::new(color_value)
    }
}
