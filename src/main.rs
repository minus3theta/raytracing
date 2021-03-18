use std::sync::Arc;

use indicatif::{ProgressBar, ProgressIterator};
use structopt::StructOpt;

use raytracing::{
    background::BackgroundPtr, hittable::EmittablePtr, pdf::EmittablePdf, pdf::MixturePdf, Camera,
    Color, CosinePdf, HittablePtr, Opt, Pdf, Random, Ray, Vec3,
};

const MAX_DEPTH: i32 = 50;
const RECURSION_DEPTH: i32 = 3;

fn ray_color(
    r: &Ray,
    world: &HittablePtr,
    lights: &EmittablePtr,
    background: &BackgroundPtr,
    depth: i32,
    rng: &mut Random,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY, rng) {
        let emmited = rec.mat_ptr.emmitted(&rec);

        if let Some((attenuation, _scattered, _pdf)) = rec.mat_ptr.scatter(r, &rec, rng) {
            let light_pdf = EmittablePdf::new(lights.clone(), rec.p.clone());
            let cosine_pdf = CosinePdf::new(&rec.normal);
            let mixed_pdf = MixturePdf::new(Arc::new(light_pdf), Arc::new(cosine_pdf));
            let scattered = Ray::new(rec.p.clone(), mixed_pdf.generate(rng), r.time);
            let pdf = mixed_pdf.value(&scattered.dir, rng);

            emmited
                + attenuation
                    * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                    * ray_color(&scattered, world, lights, background, depth - 1, rng)
                    / pdf
        } else {
            emmited
        }
    } else {
        background.value(r)
    }
}

type Picture = Vec<Vec<Color>>;

fn render(
    camera: &Camera,
    world: &HittablePtr,
    lights: &EmittablePtr,
    background: &BackgroundPtr,
    height: usize,
    width: usize,
    samples_per_pixel: usize,
    bar: &mut Option<ProgressBar>,
) -> Picture {
    let mut rng = Random::default();

    (0..height)
        .rev()
        .map(|j| {
            if let Some(bar) = bar {
                bar.inc(1);
            }
            (0..width)
                .map(|i| {
                    let mut color_pixel = Color::default();
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rng.unit_f64()) / (width - 1) as f64;
                        let v = (j as f64 + rng.unit_f64()) / (height - 1) as f64;
                        let r = camera.get_ray(u, v, &mut rng);
                        color_pixel +=
                            ray_color(&r, world, lights, background, MAX_DEPTH, &mut rng);
                    }

                    color_pixel / samples_per_pixel as f64
                })
                .collect()
        })
        .collect()
}

fn render_recursive(
    camera: &Camera,
    world: &HittablePtr,
    lights: &EmittablePtr,
    background: &BackgroundPtr,
    height: usize,
    width: usize,
    samples_per_pixel: usize,
    depth: i32,
    bar: &mut Option<ProgressBar>,
) -> Picture {
    if depth == 0 {
        return render(
            camera,
            world,
            lights,
            background,
            height,
            width,
            samples_per_pixel,
            bar,
        );
    }
    let (mut p1, p2) = rayon::join(
        || {
            render_recursive(
                camera,
                world,
                lights,
                background,
                height,
                width,
                samples_per_pixel,
                depth - 1,
                bar,
            )
        },
        || {
            render_recursive(
                camera,
                world,
                lights,
                background,
                height,
                width,
                samples_per_pixel,
                depth - 1,
                &mut None,
            )
        },
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
    let opt = Opt::from_args();

    let mut rng = Random::default();

    let sc = opt.scene.generate_scene(&mut rng);

    let image_width = opt.image_width;
    let image_height = (image_width as f64 / sc.aspect_ratio) as usize;

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::with_scene(&sc, vup, sc.aspect_ratio, dist_to_focus, 0.0, 1.0);

    let pic = render_recursive(
        &cam,
        &sc.world,
        &sc.lights,
        &sc.background,
        image_height,
        image_width,
        opt.samples_per_pixel,
        RECURSION_DEPTH,
        &mut Some(ProgressBar::new(image_height as u64)),
    );

    println!("P3\n{} {}\n255", image_width, image_height);

    for row in pic.iter().progress() {
        for color_pixel in row {
            println!("{}", color_pixel);
        }
    }
}
