use std::sync::Arc;

use super::Texture;
use crate::{Color, Point3, Random, Vec3};

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Arc<Perlin>,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64, noise: Arc<Perlin>) -> Self {
        Self { scale, noise }
    }

    pub fn with_rng(scale: f64, rng: &mut Random) -> Self {
        Self {
            scale,
            noise: Arc::new(Perlin::new(rng)),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * p)))
    }
}

#[derive(Debug, Clone)]
pub struct Turbulence {
    noise: Arc<Perlin>,
    scale: f64,
}

impl Turbulence {
    pub fn new(noise: Arc<Perlin>, scale: f64) -> Self {
        Self { noise, scale }
    }

    pub fn with_rng(scale: f64, rng: &mut Random) -> Self {
        Self {
            scale,
            noise: Arc::new(Perlin::new(rng)),
        }
    }
}

impl Texture for Turbulence {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.turb(self.scale * p, 7)
    }
}

#[derive(Debug, Clone)]
pub struct Marble {
    noise: Arc<Perlin>,
    scale: f64,
}

impl Marble {
    pub fn new(noise: Arc<Perlin>, scale: f64) -> Self {
        Self { noise, scale }
    }

    pub fn with_rng(scale: f64, rng: &mut Random) -> Self {
        Self {
            scale,
            noise: Arc::new(Perlin::new(rng)),
        }
    }
}

impl Texture for Marble {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p.clone(), 7)).sin())
    }
}

type Perm = [usize; Perlin::POINT_COUNT];

#[derive(Debug, Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
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
        Self {
            ranvec: (0..Self::POINT_COUNT)
                .map(|_| Vec3::random(rng, -1.0, 1.0))
                .collect(),
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c: [[[Vec3; 2]; 2]; 2] = Default::default();

        fn idx(i: i64, di: usize) -> usize {
            (i + di as i64).rem_euclid(Perlin::POINT_COUNT as i64) as usize
        }

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[idx(i, di)]
                        ^ self.perm_y[idx(j, dj)]
                        ^ self.perm_z[idx(k, dk)]]
                    .clone()
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        fn herm(x: f64) -> f64 {
            x * x * (3.0 - 2.0 * x)
        }
        let uu = herm(u);
        let vv = herm(v);
        let ww = herm(w);

        fn fac(i: usize, u: f64) -> f64 {
            i as f64 * u + (1 - i) as f64 * (1. - u)
        }
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += fac(i, uu) * fac(j, vv) * fac(k, ww) * c[i][j][k].dot(&weight);
                }
            }
        }
        accum
    }

    pub fn turb(&self, mut p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }

        accum.abs()
    }
}
