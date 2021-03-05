use std::rc::Rc;

use indicatif::ProgressIterator;

use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::{Camera, Color, Point3, Random, Ray, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(
    r: &Ray,
    world: &impl Hittable,
    depth: i32,
    rng: &mut Random<impl rand::Rng>,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        let target = &rec.p + &rec.normal + Vec3::random_in_unit_sphere(rng);
        return 0.5
            * ray_color(
                &Ray::new(rec.p.clone(), &target - &rec.p),
                world,
                depth - 1,
                rng,
            );
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut rng = Random::default();

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::default();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev().progress() {
        for i in 0..IMAGE_WIDTH {
            let mut color_pixel = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.unit_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.unit_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                color_pixel += ray_color(&r, &world, MAX_DEPTH, &mut rng);
            }

            println!("{}", color_pixel / SAMPLES_PER_PIXEL as f64);
        }
    }
}
