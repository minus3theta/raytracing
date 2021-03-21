use std::sync::Arc;

use crate::{
    background::BackgroundEnum,
    background::{dark, sky},
    emittable::EmittableEnum,
    hittable::ConstantMedium,
    hittable::HittableEnum,
    hittable::Triangle,
    hittable::{
        BoxObj, BvhNode, FlipFace, HittableList, MovingSphere, Sphere, XYRect, XZRect, YZRect,
    },
    texture::{Checker, ImageTexture, Marble},
    Color, EmittablePtr, HittablePtr, Material, Point3, Random, TexturePtr, Vec3,
};

pub struct Scene {
    pub world: HittableEnum,
    pub lights: EmittableEnum,
    pub background: BackgroundEnum,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vfov: f64,
    pub aperture: f64,
    pub aspect_ratio: f64,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            world: Default::default(),
            lights: Default::default(),
            background: sky(),
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::default(),
            vfov: 20.0,
            aperture: 0.0,
            aspect_ratio: 3.0 / 2.0,
        }
    }
}

impl Scene {
    pub fn random_scene(rng: &mut Random) -> Self {
        let mut world = HittableList::default();

        let checker: TexturePtr = Arc::new(Checker::with_color(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));
        let ground_material = Material::lambertian(checker);
        world.add(HittableEnum::sphere(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        ));

        let mut objects: Vec<HittablePtr> = Vec::new();

        let glass_material = Material::dielectric(1.5);
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
                    let mat = Material::lambertian(albedo);
                    let center2 = &center + Vec3::new(0.0, rng.range_f64(0.0, 0.5), 0.0);
                    objects.push(HittableEnum::moving_sphere(
                        center, center2, 0.0, 1.0, 0.2, mat,
                    ));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng);
                    let fuzz = rng.range_f64(0.0, 0.5);
                    let mat = Material::metal(albedo, fuzz);
                    objects.push(HittableEnum::sphere(center, 0.2, mat));
                } else {
                    objects.push(HittableEnum::sphere(center, 0.2, glass_material.clone()));
                }
            }
        }
        let bvh = BvhNode::new(&mut objects, 0.0, 1.0, rng).unwrap();
        world.add(HittableEnum::bvh_node(bvh));

        world.add(HittableEnum::sphere(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Material::dielectric(1.5),
        ));
        world.add(HittableEnum::sphere(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::lambertian(Color::new(0.4, 0.2, 0.1)),
        ));
        world.add(HittableEnum::sphere(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Material::metal(Color::new(0.7, 0.6, 0.5), 0.0),
        ));

        Scene {
            world: world.into(),
            aperture: 0.1,
            ..Default::default()
        }
    }

    pub fn two_spheres(_: &mut Random) -> Self {
        let mut world = HittableList::default();

        let checker: TexturePtr = Arc::new(Checker::with_color(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));
        let mat = Material::lambertian(checker);
        world.add(HittableEnum::sphere(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            mat.clone(),
        ));
        world.add(HittableEnum::sphere(Point3::new(0.0, 10.0, 0.0), 10.0, mat));

        Scene {
            world: world.into(),
            ..Default::default()
        }
    }

    // pub fn two_perlin_spheres(rng: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let pertext = Arc::new(Marble::with_rng(4.0, rng));
    //     let mat = Arc::new(Lambertian::new(pertext));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(0.0, -1000.0, 0.0),
    //         1000.0,
    //         mat.clone(),
    //     )));
    //     world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)));

    //     Scene {
    //         world: Arc::new(world),
    //         ..Default::default()
    //     }
    // }

    // pub fn earth(_: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let earth_texture = Arc::new(ImageTexture::new("res/earthmap.jpg").unwrap());
    //     let earth_surface = Arc::new(Lambertian::new(earth_texture));
    //     let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    //     world.add(globe);

    //     Scene {
    //         world: Arc::new(world),
    //         ..Default::default()
    //     }
    // }

    // pub fn simple_light(rng: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let pertext = Arc::new(Marble::with_rng(4.0, rng));
    //     let mat = Arc::new(Lambertian::new(pertext));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(0.0, -1000.0, 0.0),
    //         1000.0,
    //         mat.clone(),
    //     )));
    //     world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)));

    //     let difflight = Arc::new(DiffuseLight::with_color(Color::new(4.0, 4.0, 4.0)));
    //     world.add(Arc::new(XYRect::new(
    //         3.0,
    //         5.0,
    //         1.0,
    //         3.0,
    //         -2.0,
    //         difflight.clone(),
    //     )));

    //     Scene {
    //         world: Arc::new(world),
    //         background: dark(),
    //         lookfrom: Point3::new(26.0, 3.0, 6.0),
    //         lookat: Point3::new(0.0, 2.0, 0.0),
    //         ..Default::default()
    //     }
    // }

    // pub fn cornell_box(_: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let red = Arc::new(Lambertian::with_color(Color::new(0.65, 0.05, 0.05)));
    //     let white = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
    //     let green = Arc::new(Lambertian::with_color(Color::new(0.12, 0.45, 0.15)));
    //     let light = Arc::new(DiffuseLight::with_color(Color::new(15.0, 15.0, 15.0)));

    //     world.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    //     world.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    //     let light_rect = Arc::new(XZRect::new(213., 343., 227., 332., 554., light));
    //     world.add(Arc::new(FlipFace::new(light_rect.clone())));
    //     world.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    //     world.add(Arc::new(XZRect::new(
    //         0.,
    //         555.,
    //         0.,
    //         555.,
    //         555.,
    //         white.clone(),
    //     )));
    //     world.add(Arc::new(XYRect::new(
    //         0.,
    //         555.,
    //         0.,
    //         555.,
    //         555.,
    //         white.clone(),
    //     )));

    //     let box1 = Arc::new(BoxObj::new(
    //         Point3::default(),
    //         Point3::new(165., 330., 165.),
    //         white.clone(),
    //     ));
    //     let box1 = rotate_y(box1, 15.);
    //     let box1 = translate(box1, Vec3::new(265., 0., 295.));
    //     world.add(box1);
    //     let box2 = Arc::new(BoxObj::new(
    //         Point3::default(),
    //         Point3::new(165., 165., 165.),
    //         white.clone(),
    //     ));
    //     let box2 = rotate_y(box2, -18.);
    //     let box2 = translate(box2, Vec3::new(130., 0., 65.));
    //     world.add(box2);

    //     Scene {
    //         world: Arc::new(world),
    //         lights: light_rect,
    //         background: dark(),
    //         lookfrom: Point3::new(278., 278., -800.),
    //         lookat: Point3::new(278., 278., 0.),
    //         vfov: 40.0,
    //         aspect_ratio: 1.0,
    //         ..Default::default()
    //     }
    // }

    // pub fn cornell_metal(_: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let red = Arc::new(Lambertian::with_color(Color::new(0.65, 0.05, 0.05)));
    //     let white = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
    //     let green = Arc::new(Lambertian::with_color(Color::new(0.12, 0.45, 0.15)));
    //     let light = Arc::new(DiffuseLight::with_color(Color::new(15.0, 15.0, 15.0)));

    //     world.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    //     world.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    //     let light_rect = Arc::new(XZRect::new(213., 343., 227., 332., 554., light));
    //     world.add(Arc::new(FlipFace::new(light_rect.clone())));
    //     world.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    //     world.add(Arc::new(XZRect::new(
    //         0.,
    //         555.,
    //         0.,
    //         555.,
    //         555.,
    //         white.clone(),
    //     )));
    //     world.add(Arc::new(XYRect::new(
    //         0.,
    //         555.,
    //         0.,
    //         555.,
    //         555.,
    //         white.clone(),
    //     )));

    //     let alminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    //     let box1 = Arc::new(BoxObj::new(
    //         Point3::default(),
    //         Point3::new(165., 330., 165.),
    //         alminum,
    //     ));
    //     let box1 = rotate_y(box1, 15.);
    //     let box1 = translate(box1, Vec3::new(265., 0., 295.));
    //     world.add(box1);
    //     let box2 = Arc::new(BoxObj::new(
    //         Point3::default(),
    //         Point3::new(165., 165., 165.),
    //         white.clone(),
    //     ));
    //     let box2 = rotate_y(box2, -18.);
    //     let box2 = translate(box2, Vec3::new(130., 0., 65.));
    //     world.add(box2);

    //     Scene {
    //         world: Arc::new(world),
    //         lights: light_rect,
    //         background: dark(),
    //         lookfrom: Point3::new(278., 278., -800.),
    //         lookat: Point3::new(278., 278., 0.),
    //         vfov: 40.0,
    //         aspect_ratio: 1.0,
    //         ..Default::default()
    //     }
    // }

    pub fn cornell_sphere(_: &mut Random) -> Self {
        let mut world = HittableList::default();

        let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
        let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
        let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
        let light = Material::diffuse_light(Color::new(15.0, 15.0, 15.0));

        world.add(HittableEnum::yz_rect(0., 555., 0., 555., 555., green));
        world.add(HittableEnum::yz_rect(0., 555., 0., 555., 0., red));
        let light_rect = XZRect::new(213., 343., 227., 332., 554., light);
        world.add(HittableEnum::flip_face(Arc::new(light_rect.clone().into())));
        world.add(HittableEnum::xz_rect(0., 555., 0., 555., 0., white.clone()));
        world.add(HittableEnum::xz_rect(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        ));
        world.add(HittableEnum::xy_rect(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        ));

        let box1 = HittableEnum::box_obj(Point3::default(), Point3::new(165., 330., 165.), white);
        let box1 = HittableEnum::rotate_y(box1, 15.);
        let box1 = HittableEnum::translate(box1, Vec3::new(265., 0., 295.));
        world.add(box1);

        let sphere = Sphere::new(Point3::new(190., 90., 190.), 90., Material::dielectric(1.5));
        world.add(Arc::new(sphere.clone().into()));

        let lights: Vec<EmittableEnum> = vec![light_rect.into(), sphere.into()];

        Scene {
            world: world.into(),
            lights: lights.into(),
            background: dark(),
            lookfrom: Point3::new(278., 278., -800.),
            lookat: Point3::new(278., 278., 0.),
            vfov: 40.0,
            aspect_ratio: 1.0,
            ..Default::default()
        }
    }

    // pub fn cornell_smoke(_: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let red = Arc::new(Lambertian::with_color(Color::new(0.65, 0.05, 0.05)));
    //     let white = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
    //     let green = Arc::new(Lambertian::with_color(Color::new(0.12, 0.45, 0.15)));
    //     let light = Arc::new(DiffuseLight::with_color(Color::new(7.0, 7.0, 7.0)));

    //     world.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    //     world.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    //     let light_rect = Arc::new(XZRect::new(113., 443., 127., 432., 554., light));
    //     world.add(Arc::new(FlipFace::new(light_rect.clone())));
    //     world.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    //     world.add(Arc::new(XZRect::new(
    //         0.,
    //         555.,
    //         0.,
    //         555.,
    //         555.,
    //         white.clone(),
    //     )));
    //     world.add(Arc::new(XYRect::new(
    //         0.,
    //         555.,
    //         0.,
    //         555.,
    //         555.,
    //         white.clone(),
    //     )));

    //     let box1 = Arc::new(BoxObj::new(
    //         Point3::default(),
    //         Point3::new(165., 330., 165.),
    //         white.clone(),
    //     ));
    //     let box1 = rotate_y(box1, 15.);
    //     let box1 = translate(box1, Vec3::new(265., 0., 295.));
    //     let box2 = Arc::new(BoxObj::new(
    //         Point3::default(),
    //         Point3::new(165., 165., 165.),
    //         white.clone(),
    //     ));
    //     let box2 = rotate_y(box2, -18.);
    //     let box2 = translate(box2, Vec3::new(130., 0., 65.));

    //     world.add(Arc::new(ConstantMedium::new(
    //         box1,
    //         0.01,
    //         Color::new(0.0, 0.0, 0.0),
    //     )));
    //     world.add(Arc::new(ConstantMedium::new(
    //         box2,
    //         0.01,
    //         Color::new(1.0, 1.0, 1.0),
    //     )));

    //     let lights: Arc<Vec<EmittablePtr>> = Arc::new(vec![light_rect]);

    //     Scene {
    //         world: Arc::new(world),
    //         lights,
    //         background: dark(),
    //         lookfrom: Point3::new(278., 278., -800.),
    //         lookat: Point3::new(278., 278., 0.),
    //         vfov: 40.0,
    //         aspect_ratio: 1.0,
    //         ..Default::default()
    //     }
    // }

    // pub fn final_scene(rng: &mut Random) -> Scene {
    //     let mut world = HittableList::default();

    //     let mut boxes1: Vec<HittablePtr> = Vec::new();
    //     let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53).into()));
    //     let boxes_per_side = 20;
    //     for i in 0..boxes_per_side {
    //         for j in 0..boxes_per_side {
    //             let i = i as f64;
    //             let j = j as f64;

    //             let w = 100.0;
    //             let x0 = -1000.0 + i * w;
    //             let z0 = -1000.0 + j * w;
    //             let y0 = 0.0;
    //             let x1 = x0 + w;
    //             let y1 = rng.range_f64(1.0, 101.0);
    //             let z1 = z0 + w;

    //             boxes1.push(Arc::new(BoxObj::new(
    //                 Point3::new(x0, y0, z0),
    //                 Point3::new(x1, y1, z1),
    //                 ground.clone(),
    //             )));
    //         }
    //     }
    //     world.add(Arc::new(BvhNode::new(&mut boxes1, 0.0, 1.0, rng).unwrap()));

    //     let light = Arc::new(DiffuseLight::with_color(Color::new(7., 7., 7.)));
    //     let light_rect = Arc::new(XZRect::new(123., 423., 147., 412., 554., light));
    //     world.add(Arc::new(FlipFace::new(light_rect.clone())));

    //     let center0 = Point3::new(400., 400., 200.);
    //     let center1 = &center0 + Vec3::new(30.0, 0.0, 0.0);
    //     let moving_sphere_material = Arc::new(Lambertian::with_color(Color::new(0.7, 0.3, 0.1)));
    //     world.add(Arc::new(MovingSphere::new(
    //         center0,
    //         center1,
    //         0.0,
    //         1.0,
    //         50.0,
    //         moving_sphere_material,
    //     )));

    //     let dielectric = Arc::new(Dielectric::new(1.5));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(260.0, 150.0, 45.0),
    //         50.0,
    //         dielectric.clone(),
    //     )));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(0.0, 150.0, 145.0),
    //         50.0,
    //         Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    //     )));

    //     let boundary = Arc::new(Sphere::new(
    //         Point3::new(360., 150., 145.),
    //         70.,
    //         dielectric.clone(),
    //     ));
    //     world.add(boundary.clone());
    //     world.add(Arc::new(ConstantMedium::new(
    //         boundary,
    //         0.2,
    //         Color::new(0.2, 0.4, 0.9),
    //     )));
    //     let boundary = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 5000., dielectric));
    //     world.add(Arc::new(ConstantMedium::new(
    //         boundary,
    //         0.0001,
    //         Color::new(1.0, 1.0, 1.0),
    //     )));

    //     let emat = Arc::new(Lambertian::new(Arc::new(
    //         ImageTexture::new("res/earthmap.jpg").unwrap(),
    //     )));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(400., 200., 400.),
    //         100.,
    //         emat,
    //     )));
    //     let pertext = Arc::new(Marble::with_rng(0.1, rng));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(220., 280., 300.),
    //         80.,
    //         Arc::new(Lambertian::new(pertext)),
    //     )));

    //     let mut boxes2: Vec<HittablePtr> = Vec::new();
    //     let white = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
    //     let ns = 1000;
    //     for _ in 0..ns {
    //         boxes2.push(Arc::new(Sphere::new(
    //             Point3::random(rng, 0.0, 165.),
    //             10.,
    //             white.clone(),
    //         )))
    //     }
    //     world.add(translate(
    //         rotate_y(
    //             Arc::new(BvhNode::new(&mut boxes2, 0.0, 1.0, rng).unwrap()),
    //             15.,
    //         ),
    //         Vec3::new(-100., 270., 395.),
    //     ));

    //     let lights: Arc<Vec<EmittablePtr>> = Arc::new(vec![light_rect]);

    //     Scene {
    //         world: Arc::new(world),
    //         lights,
    //         background: dark(),
    //         lookfrom: Point3::new(478., 278., -600.),
    //         lookat: Point3::new(278., 278., 0.),
    //         vfov: 40.0,
    //         aspect_ratio: 1.0,
    //         ..Default::default()
    //     }
    // }

    // pub fn triangle(_: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let ground_material = Arc::new(Lambertian::with_color(Color::new(0.8, 0.8, 0.0)));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(0.0, -1000.0, 0.0),
    //         1000.0,
    //         ground_material,
    //     )));

    //     let triangle_material = Arc::new(Lambertian::new(Arc::new(
    //         ImageTexture::new("res/earthmap.jpg").unwrap(),
    //     )));
    //     let p0 = Point3::new(1.0, 0.5, 3.0);
    //     let p1 = Point3::new(5.0, 1.5, 1.0);
    //     let p2 = Point3::new(1.0, 4.0, 1.0);
    //     let p3 = Point3::new(2.0, 0.2, -1.0);

    //     world.add(Arc::new(Triangle::new(
    //         p0.clone(),
    //         p1.clone(),
    //         p2.clone(),
    //         triangle_material.clone(),
    //     )));
    //     world.add(Arc::new(Triangle::new(
    //         p0.clone(),
    //         p1.clone(),
    //         p3.clone(),
    //         triangle_material.clone(),
    //     )));
    //     world.add(Arc::new(Triangle::new(
    //         p0.clone(),
    //         p2.clone(),
    //         p3.clone(),
    //         triangle_material.clone(),
    //     )));
    //     world.add(Arc::new(Triangle::new(
    //         p1.clone(),
    //         p2.clone(),
    //         p3.clone(),
    //         triangle_material.clone(),
    //     )));

    //     Scene {
    //         world: Arc::new(world),
    //         vfov: 40.0,
    //         ..Default::default()
    //     }
    // }

    // pub fn teapot(rng: &mut Random) -> Self {
    //     let mut world = HittableList::default();

    //     let checker = Arc::new(Checker::with_color(
    //         Color::new(0.2, 0.3, 0.1),
    //         Color::new(0.9, 0.9, 0.9),
    //     ));
    //     let ground_material = Arc::new(Lambertian::new(checker));
    //     world.add(Arc::new(Sphere::new(
    //         Point3::new(0.0, -1000.0, 0.0),
    //         1000.0,
    //         ground_material,
    //     )));

    //     let pot_mat = Arc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
    //     let pot = BvhNode::load("res/teapot.obj", 0.0, 1.0, pot_mat, rng).unwrap();

    //     world.add(Arc::new(pot));

    //     Scene {
    //         world: Arc::new(world),
    //         lookfrom: Point3::new(3.0, 2.0, 13.0),
    //         vfov: 40.0,
    //         ..Default::default()
    //     }
    // }
}
