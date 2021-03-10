use indicatif::{ProgressBar, ProgressIterator};

use raytracing::hittable::Hittable;
use raytracing::scene;
use raytracing::{Camera, Color, Random, Ray, Vec3};

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: usize = 600;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 64;
const MAX_DEPTH: i32 = 50;
const RECURSION_DEPTH: i32 = 3;

fn ray_color(r: &Ray, world: &impl Hittable, depth: i32, rng: &mut Random) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec, rng) {
            return attenuation * ray_color(&scattered, world, depth - 1, rng);
        }
        return Color::default();
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

type Picture = Vec<Vec<Color>>;

fn render(camera: &Camera, world: &impl Hittable, bar: &mut Option<ProgressBar>) -> Picture {
    let mut rng = Random::default();

    (0..IMAGE_HEIGHT)
        .rev()
        .map(|j| {
            if let Some(bar) = bar {
                bar.inc(1);
            }
            (0..IMAGE_WIDTH)
                .map(|i| {
                    let mut color_pixel = Color::default();
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + rng.unit_f64()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (j as f64 + rng.unit_f64()) / (IMAGE_HEIGHT - 1) as f64;
                        let r = camera.get_ray(u, v, &mut rng);
                        color_pixel += ray_color(&r, world, MAX_DEPTH, &mut rng);
                    }

                    color_pixel / SAMPLES_PER_PIXEL as f64
                })
                .collect()
        })
        .collect()
}

fn render_recursive(
    camera: &Camera,
    world: &(impl Hittable + Send + Sync),
    depth: i32,
    bar: &mut Option<ProgressBar>,
) -> Picture {
    if depth == 0 {
        return render(camera, world, bar);
    }
    let (mut p1, p2) = rayon::join(
        || render_recursive(camera, world, depth - 1, bar),
        || render_recursive(camera, world, depth - 1, &mut None),
    );
    for (r1, r2) in p1.iter_mut().zip(&p2) {
        for (c1, c2) in r1.iter_mut().zip(r2) {
            *c1 += c2;
            *c1 /= 2.0;
        }
    }
    p1
}

fn main() {
    let mut rng = Random::default();

    let sc = scene::random_scene(&mut rng);

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::with_scene(&sc, vup, ASPECT_RATIO, dist_to_focus, 0.0, 1.0);

    let pic = render_recursive(
        &cam,
        &sc.world,
        RECURSION_DEPTH,
        &mut Some(ProgressBar::new(IMAGE_HEIGHT as u64)),
    );

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for row in pic.iter().progress() {
        for color_pixel in row {
            println!("{}", color_pixel);
        }
    }
}
