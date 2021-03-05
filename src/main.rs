use std::rc::Rc;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::{Camera, Color, Point3, Ray, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return Color(0.5 * (rec.normal + Vec3::new(1., 1., 1.)));
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut rng = thread_rng();
    let distr = Uniform::new(0.0, 1.0);

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::default();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color_pixel = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.sample(&distr)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.sample(&distr)) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                color_pixel += ray_color(&r, &world);
            }

            println!("{}", color_pixel / SAMPLES_PER_PIXEL as f64);
        }
    }
}
