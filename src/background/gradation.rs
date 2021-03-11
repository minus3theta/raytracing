use super::Background;
use crate::{Color, Ray};

#[derive(Debug, Clone)]
pub struct Gradation {
    bottom: Color,
    top: Color,
}

impl Gradation {
    pub fn new(bottom: Color, top: Color) -> Self {
        Self { bottom, top }
    }
}

impl Background for Gradation {
    fn value(&self, ray: &Ray) -> Color {
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * &self.bottom + t * &self.top
    }
}
