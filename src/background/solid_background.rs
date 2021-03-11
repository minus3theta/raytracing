use super::Background;
use crate::{Color, Ray};

#[derive(Debug, Clone)]
pub struct SolidBackground {
    color: Color,
}

impl SolidBackground {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Background for SolidBackground {
    fn value(&self, _: &Ray) -> Color {
        self.color.clone()
    }
}
