use std::sync::Arc;

use indicatif::{ProgressBar, ProgressIterator};

use raytracing::hittable::{BvhNode, Hittable, HittableList, MovingSphere, Sphere};
use raytracing::material::{Dielectric, Lambertian, Metal};
use raytracing::{Camera, Color, Point3, Random, Ray, Vec3};

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
        if let Some((attenuation, scattered)) = rec.mat_ptr.as_ref().scatter(r, &rec, rng) {
            return attenuation * ray_color(&scattered, world, depth - 1, rng);
        }
        return Color::default();
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene(rng: &mut Random) -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::with_color(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut objects: Vec<Arc<dyn Hittable + Send + Sync>> = Vec::new();

    let glass_material = Arc::new(Dielectric::new(1.5));
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.unit_f64(),
                0.2,
                b as f64 + 0.9 * rng.unit_f64(),
            );
            if (&center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }
            let choose_mat = rng.unit_f64();
            if choose_mat < 0.8 {
                let albedo = Color::random(rng) * Color::random(rng);
                let mat = Arc::new(Lambertian::with_color(albedo));
                let center2 = &center + Vec3::new(0.0, rng.range_f64(0.0, 0.5), 0.0);
                objects.push(Arc::new(MovingSphere::new(
                    center, center2, 0.0, 1.0, 0.2, mat,
                )));
            } else if choose_mat < 0.95 {
                let albedo = Color::random(rng);
                let fuzz = rng.range_f64(0.0, 0.5);
                let mat = Arc::new(Metal::new(albedo, fuzz));
                objects.push(Arc::new(Sphere::new(center, 0.2, mat)));
            } else {
                objects.push(Arc::new(Sphere::new(center, 0.2, glass_material.clone())));
            }
        }
    }
    let bvh = BvhNode::new(&mut objects, 0.0, 1.0, rng).unwrap();
    world.add(Arc::new(bvh));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::with_color(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
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

    let world = random_scene(&mut rng);

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0f64.to_radians(),
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let pic = render_recursive(
        &cam,
        &world,
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
