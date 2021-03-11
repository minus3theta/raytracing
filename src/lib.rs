#[macro_use]
extern crate impl_ops;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod opt;
pub mod random;
pub mod ray;
pub mod scene;
pub mod texture;
pub mod vec3;

pub use camera::Camera;
pub use color::Color;
pub use hittable::{HitRecord, Hittable, HittablePtr};
pub use material::{Material, MaterialPtr};
pub use opt::Opt;
pub use random::Random;
pub use ray::Ray;
pub use texture::{Texture, TexturePtr};
pub use vec3::{Point3, Vec3};
