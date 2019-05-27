use super::Random;
use super::Shuffler;
use paste;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng, SeedableRng, RngCore};
use std::cell::RefCell;
use wonderbox::autoresolvable;
use rand_hc::Hc128Rng;
use std::rc::Rc;
use std::fmt::Debug;

pub type Seed = <Hc128Rng as SeedableRng>::Seed;

/// Random number generator implementation that uses the `rand` crate
#[derive(Debug, Clone)]
pub struct RandomImpl {
    rng: Rc<RefCell<dyn DebugRngCore>>,
}

trait DebugRngCore = RngCore + Debug;

impl RandomImpl {
    /// Constructs a new [`RandomImpl`] by seeding a new threaded rng source
    pub fn new() -> Self {
        Self {
            rng: Rc::new(RefCell::new(thread_rng())),
        }
    }

    pub fn with_seed(seed: Seed) -> Self {
        unimplemented!()
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
        test_flips_true_on_100_percent_chance(random());
    }

    #[test]
    fn flips_true_on_100_percent_chance_with_seed() {
        test_flips_true_on_100_percent_chance(seeded_random());
    }

    fn test_flips_true_on_100_percent_chance(random: RandomImpl) {
        let coin_flip_result = random.flip_coin_with_probability(1.0);
        assert!(coin_flip_result);
    }

    #[test]
    fn flips_false_on_0_percent_chance() {
        test_flips_false_on_0_percent_chance(random());
    }

    #[test]
    fn flips_false_on_0_percent_chance_with_seed() {
        test_flips_false_on_0_percent_chance(seeded_random());
    }

    fn test_flips_false_on_0_percent_chance(random: RandomImpl) {
        let coin_flip_result = random.flip_coin_with_probability(0.0);
        assert!(!coin_flip_result);
    }

    #[test]
    fn does_not_panic_on_valid_range() {
        test_does_not_panic_on_valid_range(random());
    }

    #[test]
    fn does_not_panic_on_valid_range_with_seed() {
        test_does_not_panic_on_valid_range(seeded_random());
    }

    fn test_does_not_panic_on_valid_range(random: RandomImpl) {
        random.flip_coin_with_probability(0.5);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_probability() {
        test_panics_on_negative_probability_with_seed(random());
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_probability_with_seed() {
        test_panics_on_negative_probability_with_seed(seeded_random());
    }

    fn test_panics_on_negative_probability_with_seed(random: RandomImpl) {
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
    fn panics_too_high_probability_with_seed() {
        let random = RandomImpl::new();
        random.flip_coin_with_probability(1.1);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_higher_than_max() {
        test_panics_when_min_is_higher_than_max(random());
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_higher_than_max_with_seed() {
        test_panics_when_min_is_higher_than_max(seeded_random());
    }

    fn test_panics_when_min_is_higher_than_max(random: RandomImpl) {
        random.i32_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_max() {
        test_panics_when_min_is_max(random());
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_max_with_seed() {
        test_panics_when_min_is_max(random());
    }

    fn test_panics_when_min_is_max(random: RandomImpl) {
        const ONLY_BOUND: i32 = 1;
        random.i32_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_number_is_in_range() {
        test_returned_number_is_in_range(random());
    }

    #[test]
    fn returned_number_is_in_range_with_seed() {
        test_returned_number_is_in_range(seeded_random());
    }

    fn test_returned_number_is_in_range(random: RandomImpl) {
        const MIN: i32 = -1;
        const MAX: i32 = 3;

        let number = random.i32_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[test]
    fn returns_only_possibility() {
        test_returns_only_possibility(random());
    }

    #[test]
    fn returns_only_possibility_with_seed() {
        test_returns_only_possibility(seeded_random());
    }

    fn test_returns_only_possibility(random: RandomImpl) {
        const MIN: i32 = 1;
        const MAX: i32 = 2;

        let number = random.i32_in_range(MIN, MAX);
        assert_eq!(1, number);
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_higher_than_max() {
        test_panics_when_float_min_is_higher_than_max(random());
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_higher_than_max_with_seed() {
        test_panics_when_float_min_is_higher_than_max(seeded_random());
    }

    fn test_panics_when_float_min_is_higher_than_max(random: RandomImpl) {
        random.f64_in_range(1.0, 0.0);
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_max() {
        test_panics_when_float_min_is_max(random());
    }

    #[should_panic]
    #[test]
    fn panics_when_float_min_is_max_with_seed() {
        test_panics_when_float_min_is_max(seeded_random());
    }

    fn test_panics_when_float_min_is_max(random: RandomImpl) {
        const ONLY_BOUND: f64 = 1.0;
        random.f64_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_float_is_in_range() {
        test_returned_float_is_in_range(random());
    }

    #[test]
    fn returned_float_is_in_range_with_seed() {
        test_returned_float_is_in_range(seeded_random());
    }

    fn test_returned_float_is_in_range(random: RandomImpl) {
        const MIN: f64 = -1.0;
        const MAX: f64 = 3.0;

        let number = random.f64_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[should_panic]
    #[test]
    fn panics_when_usize_min_is_higher_than_max() {
        test_panics_when_usize_min_is_higher_than_max(random());
    }

    #[should_panic]
    #[test]
    fn panics_when_usize_min_is_higher_than_max_with_seed() {
        test_panics_when_usize_min_is_higher_than_max(seeded_random());
    }

    fn test_panics_when_usize_min_is_higher_than_max(random: RandomImpl) {
        random.usize_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_usize_min_is_max() {
        test_panics_when_usize_min_is_max(random());
    }

    #[should_panic]
    #[test]
    fn panics_when_usize_min_is_max_with_seed() {
        test_panics_when_usize_min_is_max(seeded_random());
    }

    fn test_panics_when_usize_min_is_max(random: RandomImpl) {
        const ONLY_BOUND: usize = 1;
        random.usize_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returned_usize_is_in_range() {
        test_returned_usize_is_in_range(random());
    }

    #[test]
    fn returned_usize_is_in_range_with_seed() {
        test_returned_usize_is_in_range(seeded_random());
    }

    fn test_returned_usize_is_in_range(random: RandomImpl) {
        const MIN: usize = 1;
        const MAX: usize = 3;

        let number = random.usize_in_range(MIN, MAX);
        assert!(number >= MIN && number < MAX);
    }

    #[test]
    fn shuffled_vector_has_the_same_length() {
        test_shuffled_vector_has_the_same_length(random());
    }

    #[test]
    fn shuffled_vector_has_the_same_length_with_seed() {
        test_shuffled_vector_has_the_same_length(seeded_random());
    }

    fn test_shuffled_vector_has_the_same_length(random: RandomImpl) {
        let values = vec![10, 20, 30];
        let shuffled_values = random.shuffle(values.clone());
        assert_eq!(values.len(), shuffled_values.len());
    }

    #[test]
    fn shuffled_vector_contains_the_same_elements() {
        test_shuffled_vector_contains_the_same_elements(random());
    }

    #[test]
    fn shuffled_vector_contains_the_same_elements_with_seed() {
        test_shuffled_vector_contains_the_same_elements(seeded_random());
    }

    fn test_shuffled_vector_contains_the_same_elements(random: RandomImpl) {
        let values = vec![10, 20, 30];
        let shuffled_values = random.shuffle(values.clone());

        for element in values {
            assert!(shuffled_values.contains(&element));
        }
    }

    #[test]
    fn empty_vector_can_be_shuffled() {
        test_empty_vector_can_be_shuffled(random());
    }

    #[test]
    fn empty_vector_can_be_shuffled_with_seed() {
        test_empty_vector_can_be_shuffled(seeded_random());
    }

    fn test_empty_vector_can_be_shuffled(random: RandomImpl) {
        let empty_vector: Vec<()> = Vec::default();
        let shuffled_vector = random.shuffle(empty_vector);
        assert!(shuffled_vector.is_empty());
    }

    fn random() -> RandomImpl {
        RandomImpl::default()
    }

    fn seeded_random() -> RandomImpl {
        const SEED: Seed = [
            0x84, 0x54, 0xe5, 0x1e, 0x6, 0x95, 0x65, 0xf, 0x81, 0xa9, 0x99, 0x29, 0xf6, 0xa2, 0xc9,
            0x26, 0xce, 0x48, 0x5a, 0x95, 0xb0, 0xc0, 0x4a, 0x1c, 0xa8, 0xf2, 0x12, 0x56, 0xae,
            0x34, 0x10, 0xf3,
        ];

        RandomImpl::with_seed(SEED)
    }
}
