use structopt::clap::arg_enum;
use structopt::StructOpt;

use crate::{scene::Scene, Random};

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Width of output image (px)
    #[structopt(short = "w", long, default_value = "600")]
    pub image_width: usize,

    /// Number of samples per pixel for each thread
    #[structopt(short = "s", long = "samples", default_value = "64")]
    pub samples_per_pixel: usize,

    /// Scenes (random, twospheres, twoperlinspheres, earth, simplelight)
    #[structopt(default_value = "random")]
    pub scene: SceneSelector,
}

arg_enum! {
    #[derive(Debug, Copy, Clone)]
    pub enum SceneSelector {
        Random,
        TwoSpheres,
        TwoPerlinSpheres,
        Earth,
        SimpleLight,
    }
}

impl SceneSelector {
    pub fn generate_scene(&self, rng: &mut Random) -> Scene {
        match self {
            SceneSelector::Random => Scene::random_scene(rng),
            SceneSelector::TwoSpheres => Scene::two_spheres(rng),
            SceneSelector::TwoPerlinSpheres => Scene::two_perlin_spheres(rng),
            SceneSelector::Earth => Scene::earth(rng),
            SceneSelector::SimpleLight => Scene::simple_light(rng),
        }
    }
}
