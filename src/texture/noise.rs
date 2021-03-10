use super::Texture;
use crate::{Color, Point3, Random};

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(rng: &mut Random) -> Self {
        Self {
            noise: Perlin::new(rng),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}

type Perm = [usize; Perlin::POINT_COUNT];

#[derive(Debug, Clone)]
struct Perlin {
    ranfloat: [f64; Self::POINT_COUNT],
    perm_x: Perm,
    perm_y: Perm,
    perm_z: Perm,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(rng: &mut Random) -> Self {
        fn generate_perm(rng: &mut Random) -> Perm {
            let mut perm = [0; Perlin::POINT_COUNT];
            for (i, e) in perm.iter_mut().enumerate() {
                *e = i;
            }
            rng.shuffle(&mut perm);
            perm
        }
        let mut ranfloat = [0.0; Perlin::POINT_COUNT];
        for v in &mut ranfloat {
            *v = rng.unit_f64();
        }
        Self {
            ranfloat,
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        fn idx(x: f64) -> usize {
            ((4.0 * x) as i64).rem_euclid(Perlin::POINT_COUNT as i64) as usize
        }
        self.ranfloat[idx(p.x) ^ idx(p.y) ^ idx(p.z)]
    }
}
