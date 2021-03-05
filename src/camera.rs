use crate::{Point3, Ray, Vec3};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = Self::ASPECT_RATIO * Self::VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(Self::VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, Self::VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, Self::FOCAL_LENGTH);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
