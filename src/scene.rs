use std::sync::Arc;

use crate::background::{dark, sky, BackgroundPtr};
use crate::hittable::{
    rotate_y, translate, BoxObj, BvhNode, ConstantMedium, HittableList, MovingSphere, Sphere,
    XYRect, XZRect, YZRect,
};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::texture::{Checker, ImageTexture, Marble};
use crate::{Color, HittablePtr, Point3, Random, Vec3};

pub struct Scene {
    pub world: HittableList,
    pub background: BackgroundPtr,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vfov: f64,
    pub aperture: f64,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            world: Default::default(),
            background: sky(),
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::default(),
            vfov: 20.0,
            aperture: 0.0,
        }
    }
}

impl Scene {
    pub fn random_scene(rng: &mut Random) -> Self {
        let mut world = HittableList::default();

        let checker = Arc::new(Checker::with_color(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));
        let ground_material = Arc::new(Lambertian::new(checker));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        let mut objects: Vec<HittablePtr> = Vec::new();

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

        Scene {
            world,
            aperture: 0.1,
            ..Default::default()
        }
    }

    pub fn two_spheres(_: &mut Random) -> Self {
        let mut world = HittableList::default();

        let checker = Arc::new(Checker::with_color(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));
        let mat = Arc::new(Lambertian::new(checker));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            mat.clone(),
        )));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            mat,
        )));

        Scene {
            world,
            ..Default::default()
        }
    }

    pub fn two_perlin_spheres(rng: &mut Random) -> Self {
        let mut world = HittableList::default();

        let pertext = Arc::new(Marble::with_rng(4.0, rng));
        let mat = Arc::new(Lambertian::new(pertext));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        )));
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)));

        Scene {
            world,
            ..Default::default()
        }
    }

    pub fn earth(_: &mut Random) -> Self {
        let mut world = HittableList::default();

        let earth_texture = Arc::new(ImageTexture::new("res/earthmap.jpg").unwrap());
        let earth_surface = Arc::new(Lambertian::new(earth_texture));
        let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

        world.add(globe);

        Scene {
            world,
            ..Default::default()
        }
    }

    pub fn simple_light(rng: &mut Random) -> Self {
        let mut world = HittableList::default();

        let pertext = Arc::new(Marble::with_rng(4.0, rng));
        let mat = Arc::new(Lambertian::new(pertext));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        )));
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)));

        let difflight = Arc::new(DiffuseLight::with_color(Color::new(4.0, 4.0, 4.0)));
        world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

        Scene {
            world,
            background: dark(),
            lookfrom: Point3::new(26.0, 3.0, 6.0),
            lookat: Point3::new(0.0, 2.0, 0.0),
            ..Default::default()
        }
    }

    pub fn cornell_box(_: &mut Random) -> Self {
        let mut world = HittableList::default();

        let red = Arc::new(Lambertian::with_color(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::with_color(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::with_color(Color::new(15.0, 15.0, 15.0)));

        world.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
        world.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
        world.add(Arc::new(XZRect::new(213., 343., 227., 332., 554., light)));
        world.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
        world.add(Arc::new(XZRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));
        world.add(Arc::new(XYRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));

        let box1 = Arc::new(BoxObj::new(
            Point3::default(),
            Point3::new(165., 330., 165.),
            white.clone(),
        ));
        let box1 = rotate_y(box1, 15.);
        let box1 = translate(box1, Vec3::new(265., 0., 295.));
        world.add(box1);
        let box2 = Arc::new(BoxObj::new(
            Point3::default(),
            Point3::new(165., 165., 165.),
            white.clone(),
        ));
        let box2 = rotate_y(box2, -18.);
        let box2 = translate(box2, Vec3::new(130., 0., 65.));
        world.add(box2);

        Scene {
            world,
            background: dark(),
            lookfrom: Point3::new(278., 278., -800.),
            lookat: Point3::new(278., 278., 0.),
            vfov: 40.0,
            ..Default::default()
        }
    }

    pub fn cornell_smoke(_: &mut Random) -> Self {
        let mut world = HittableList::default();

        let red = Arc::new(Lambertian::with_color(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::with_color(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::with_color(Color::new(7.0, 7.0, 7.0)));

        world.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
        world.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
        world.add(Arc::new(XZRect::new(113., 443., 127., 432., 554., light)));
        world.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
        world.add(Arc::new(XZRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));
        world.add(Arc::new(XYRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));

        let box1 = Arc::new(BoxObj::new(
            Point3::default(),
            Point3::new(165., 330., 165.),
            white.clone(),
        ));
        let box1 = rotate_y(box1, 15.);
        let box1 = translate(box1, Vec3::new(265., 0., 295.));
        let box2 = Arc::new(BoxObj::new(
            Point3::default(),
            Point3::new(165., 165., 165.),
            white.clone(),
        ));
        let box2 = rotate_y(box2, -18.);
        let box2 = translate(box2, Vec3::new(130., 0., 65.));

        world.add(Arc::new(ConstantMedium::new(
            box1,
            0.01,
            Color::new(0.0, 0.0, 0.0),
        )));
        world.add(Arc::new(ConstantMedium::new(
            box2,
            0.01,
            Color::new(1.0, 1.0, 1.0),
        )));

        Scene {
            world,
            background: dark(),
            lookfrom: Point3::new(278., 278., -800.),
            lookat: Point3::new(278., 278., 0.),
            vfov: 40.0,
            ..Default::default()
        }
    }
}
