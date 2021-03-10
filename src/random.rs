use rand::distributions::{Standard, Uniform};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub fn unit_f64(&mut self) -> f64 {
        self.rng.sample(Standard)
    }
    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        if min == max {
            min
        } else {
            let distr = Uniform::new(min, max);
            self.rng.sample(distr)
        }
    }
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        let distr = Uniform::new(min, max);
        self.rng.sample(distr)
    }
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        slice.shuffle(&mut self.rng);
    }
}

impl Default for Random {
    fn default() -> Self {
        Self { rng: thread_rng() }
    }
}
