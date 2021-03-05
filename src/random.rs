use rand::distributions::{Standard, Uniform};
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Random<R> {
    rng: R,
}

impl<R: Rng> Random<R> {
    pub fn unit_f64(&mut self) -> f64 {
        self.rng.sample(Standard)
    }
    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        let distr = Uniform::new(min, max);
        self.rng.sample(distr)
    }
}

impl Default for Random<ThreadRng> {
    fn default() -> Self {
        Self { rng: thread_rng() }
    }
}
