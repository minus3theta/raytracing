use std::rc::Rc;

use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::{Color, Point3, Ray, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return Color(0.5 * (rec.normal + Vec3::new(1., 1., 1.)));
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let origin = Point3::default();
    let horiozntal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &horiozntal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin.clone(),
                &lower_left_corner + u * &horiozntal + v * &vertical - &origin,
            );
            let color_pixel = ray_color(&r, &world);

            println!("{}", color_pixel);
        }
    }
}
