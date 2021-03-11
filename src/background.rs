use crate::{Color, Ray};

pub trait Background {
    fn value(&self, ray: &Ray) -> Color;
}

pub fn sky() -> BackgroundPtr {
    Box::new(Gradation::new(
        Color::new(1.0, 1.0, 1.0),
        Color::new(0.5, 0.7, 1.0),
    ))
}

pub fn dark() -> BackgroundPtr {
    Box::new(SolidBackground::new(Color::default()))
}

pub type BackgroundPtr = Box<dyn Background + Send + Sync>;

pub mod gradation;
pub mod solid_background;

pub use gradation::Gradation;
pub use solid_background::SolidBackground;
