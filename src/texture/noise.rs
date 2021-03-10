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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c = [[[0.0; 2]; 2]; 2];

        fn idx(i: i64, di: usize) -> usize {
            (i + di as i64).rem_euclid(Perlin::POINT_COUNT as i64) as usize
        }

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[idx(i, di)]
                        ^ self.perm_y[idx(j, dj)]
                        ^ self.perm_z[idx(k, dk)]]
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        fn fac(i: usize, u: f64) -> f64 {
            i as f64 * u + (1 - i) as f64 * (1. - u)
        }
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += fac(i, u) * fac(j, v) * fac(k, w) * c[i][j][k];
                }
            }
        }
        accum
    }
}
