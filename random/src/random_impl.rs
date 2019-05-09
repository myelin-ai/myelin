use super::Random;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::cell::RefCell;
use wonderbox::autoresolvable;

/// Random number generator implementation that uses the `rand` crate
#[derive(Debug, Clone)]
pub struct RandomImpl {
    rng: RefCell<ThreadRng>,
}

#[autoresolvable]
impl RandomImpl {
    /// Constructs a new [`RandomImpl`] by seeding a new threaded rng source
    pub fn new() -> Self {
        Self {
            rng: RefCell::new(thread_rng()),
        }
    }
}

impl Default for RandomImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Random for RandomImpl {
    fn flip_coin(&self) -> bool {
        self.rng.borrow_mut().gen()
    }

    fn flip_coin_with_probability(&self, probability: f64) -> bool {
        if probability < 0.0 || probability > 1.0 {
            panic!(
                "Expected probability to be in range [0.0; 1.0] but got {}",
                probability
            );
        }

        self.rng.borrow_mut().gen_bool(probability)
    }

    fn random_number_in_range(&self, min: i32, max: i32) -> i32 {
        self.rng.borrow_mut().gen_range(min, max)
    }

    fn random_float_in_range(&self, min: f64, max: f64) -> f64 {
        self.rng.borrow_mut().gen_range(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_true_on_100_percent_chance() {
        let random = RandomImpl::new();
        let coin_flip_result = random.flip_coin_with_probability(1.0);
        assert!(coin_flip_result);
    }

    #[test]
    fn flips_false_on_0_percent_chance() {
        let random = RandomImpl::new();
        let coin_flip_result = random.flip_coin_with_probability(0.0);
        assert!(!coin_flip_result);
    }

    #[test]
    fn does_not_panic_on_valid_range() {
        let random = RandomImpl::new();
        random.flip_coin_with_probability(0.5);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_probability() {
        let random = RandomImpl::new();
        random.flip_coin_with_probability(-1.0);
    }

    #[should_panic]
    #[test]
    fn panics_too_high_probability() {
        let random = RandomImpl::new();
        random.flip_coin_with_probability(1.1);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_higher_than_max() {
        let random = RandomImpl::default();
        random.random_number_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_max() {
        let random = RandomImpl::default();
        const ONLY_BOUND: i32 = 1;
        random.random_number_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_number_is_in_range() {
        let random = RandomImpl::default();
        const MIN: i32 = -1;
        const MAX: i32 = 3;

        let number = random.random_number_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[test]
    fn returns_only_possibility() {
        let random = RandomImpl::default();
        const MIN: i32 = 1;
        const MAX: i32 = 2;

        let number = random.random_number_in_range(MIN, MAX);
        assert_eq!(1, number);
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_higher_than_max() {
        let random = RandomImpl::default();
        random.random_float_in_range(1.0, 0.0);
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_max() {
        let random = RandomImpl::default();
        const ONLY_BOUND: f64 = 1.0;
        random.random_float_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_float_is_in_range() {
        let random = RandomImpl::default();
        const MIN: f64 = -1.0;
        const MAX: f64 = 3.0;

        let number = random.random_float_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }
}
