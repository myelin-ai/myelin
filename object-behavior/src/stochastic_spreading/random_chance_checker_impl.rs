use super::RandomChanceChecker;
use rand::{thread_rng, Rng, ThreadRng};

/// Random number generator implementation that uses the `rand` crate
#[derive(Debug, Clone)]
pub struct RandomChanceCheckerImpl {
    rng: ThreadRng,
}

impl RandomChanceCheckerImpl {
    /// Constructs a new [`RandomChanceCheckerImpl`] by seeding a new threaded rng source
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Default for RandomChanceCheckerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomChanceChecker for RandomChanceCheckerImpl {
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool {
        if probability < 0.0 || probability > 1.0 {
            let error = format!(
                "Expected probability to be in range [0.0; 1.0] but got {}",
                probability
            );
            panic!(error);
        }
        let num: f64 = self.rng.gen();
        num <= probability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_true_on_100_percent_chance() {
        let mut random_chance_checker = RandomChanceCheckerImpl::new();
        let coin_flip_result = random_chance_checker.flip_coin_with_probability(1.0);
        assert!(coin_flip_result);
    }

    #[test]
    fn flips_false_on_0_percent_chance() {
        let mut random_chance_checker = RandomChanceCheckerImpl::new();
        let coin_flip_result = random_chance_checker.flip_coin_with_probability(0.0);
        assert!(!coin_flip_result);
    }

    #[test]
    fn does_not_panic_on_valid_range() {
        let mut random_chance_checker = RandomChanceCheckerImpl::new();
        random_chance_checker.flip_coin_with_probability(0.5);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_probability() {
        let mut random_chance_checker = RandomChanceCheckerImpl::new();
        random_chance_checker.flip_coin_with_probability(-1.0);
    }

    #[should_panic]
    #[test]
    fn panics_too_high_probability() {
        let mut random_chance_checker = RandomChanceCheckerImpl::new();
        random_chance_checker.flip_coin_with_probability(1.1);
    }
}
