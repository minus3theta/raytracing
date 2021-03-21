use crate::{Color, Ray};

#[derive(Clone)]
pub enum BackgroundEnum {
    Solid(SolidBackground),
    Gradation(Gradation),
}

impl Background for BackgroundEnum {
    fn value(&self, ray: &Ray) -> Color {
        match self {
            BackgroundEnum::Solid(b) => b.value(ray),
            BackgroundEnum::Gradation(b) => b.value(ray),
        }
    }
}

pub trait Background {
    fn value(&self, ray: &Ray) -> Color;
}

pub fn sky() -> BackgroundEnum {
    BackgroundEnum::Gradation(Gradation::new(
        Color::new(1.0, 1.0, 1.0),
        Color::new(0.5, 0.7, 1.0),
    ))
}

pub fn dark() -> BackgroundEnum {
    BackgroundEnum::Solid(SolidBackground::new(Color::default()))
}

pub mod gradation;
pub mod solid_background;

pub use gradation::Gradation;
pub use solid_background::SolidBackground;
