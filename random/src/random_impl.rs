use super::Random;
use super::Shuffler;
use paste;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
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

macro_rules! generate_random_in_range_implementations {
    ($($type:ty),+) => {
        $(
            paste::item! {
                fn [<$type _in_range>](&self, min: $type, max: $type) -> $type {
                    self.rng.borrow_mut().gen_range(min, max)
                }
            }
        )+
    };
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

    generate_random_in_range_implementations!(i32, f64, usize);
}

impl<T> Shuffler<T> for RandomImpl {
    fn shuffle(&self, mut values: Vec<T>) -> Vec<T> {
        values.shuffle(&mut *self.rng.borrow_mut());
        values
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
        random.i32_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_max() {
        let random = RandomImpl::default();
        const ONLY_BOUND: i32 = 1;
        random.i32_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_number_is_in_range() {
        let random = RandomImpl::default();
        const MIN: i32 = -1;
        const MAX: i32 = 3;

        let number = random.i32_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[test]
    fn returns_only_possibility() {
        let random = RandomImpl::default();
        const MIN: i32 = 1;
        const MAX: i32 = 2;

        let number = random.i32_in_range(MIN, MAX);
        assert_eq!(1, number);
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_higher_than_max() {
        let random = RandomImpl::default();
        random.f64_in_range(1.0, 0.0);
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_max() {
        let random = RandomImpl::default();
        const ONLY_BOUND: f64 = 1.0;
        random.f64_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_float_is_in_range() {
        let random = RandomImpl::default();
        const MIN: f64 = -1.0;
        const MAX: f64 = 3.0;

        let number = random.f64_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[should_panic]
    #[test]
    fn panics_when_usize_min_is_higher_than_max() {
        let random = RandomImpl::default();
        random.random_usize_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_usize_min_is_max() {
        let random = RandomImpl::default();
        const ONLY_BOUND: usize = 1;
        random.random_usize_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_usize_is_in_range() {
        let random = RandomImpl::default();
        const MIN: usize = 1;
        const MAX: usize = 3;

        let number = random.random_usize_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[test]
    fn shuffled_vector_has_the_same_length() {
        let random = RandomImpl::default();
        let values = vec![10, 20, 30];
        let shuffled_values = random.shuffle(values.clone());
        assert_eq!(values.len(), shuffled_values.len());
    }

    #[test]
    fn shuffled_vector_contains_the_same_elements() {
        let random = RandomImpl::default();
        let values = vec![10, 20, 30];
        let shuffled_values = random.shuffle(values.clone());

        for element in values {
            assert!(shuffled_values.contains(&element));
        }
    }

    #[test]
    fn empty_vector_can_be_shuffled() {
        let random = RandomImpl::default();
        let empty_vector: Vec<()> = Vec::default();
        let shuffled_vector = random.shuffle(empty_vector);
        assert!(shuffled_vector.is_empty());
    }
}
