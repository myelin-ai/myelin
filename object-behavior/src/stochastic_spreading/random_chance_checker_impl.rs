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
            panic!(
                "Expected probability to be in range [0.0; 1.0] but got {}",
                probability
            );
        }
        let num: f64 = self.rng.gen();
        num <= probability
    }

    fn random_number_in_range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min, max)
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

    #[should_panic]
    #[test]
    fn panics_when_min_is_higher_than_max() {
        let mut random_chance_checker = RandomChanceCheckerImpl::default();
        random_chance_checker.random_number_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_max() {
        let mut random_chance_checker = RandomChanceCheckerImpl::default();
        const ONLY_BOUND: i32 = 1;
        random_chance_checker.random_number_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_number_is_in_range() {
        let mut random_chance_checker = RandomChanceCheckerImpl::default();
        const MIN: i32 = -1;
        const MAX: i32 = 3;

        let number = random_chance_checker.random_number_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[test]
    fn returns_only_possibility() {
        let mut random_chance_checker = RandomChanceCheckerImpl::default();
        const MIN: i32 = 1;
        const MAX: i32 = 2;

        let number = random_chance_checker.random_number_in_range(MIN, MAX);
        assert_eq!(1, number);
    }
}
