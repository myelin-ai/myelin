use super::RandomChanceChecker;
use rand::{thread_rng, Rng, ThreadRng};

/// Random number generator implementation that uses the `rand` crate
#[derive(Debug)]
pub struct RandomChanceCheckerImpl {
    rng: ThreadRng,
}

impl RandomChanceCheckerImpl {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl RandomChanceChecker for RandomChanceCheckerImpl {
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool {
        let num: f64 = self.rng.gen();
        num <= probability
    }
}
