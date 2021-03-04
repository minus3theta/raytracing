#[macro_use]
extern crate impl_ops;

pub mod color;
pub mod hittable;
pub mod ray;
pub mod vec3;

pub use color::Color;
pub use hittable::{HitRecord, Hittable};
pub use ray::Ray;
pub use vec3::{Point3, Vec3};
