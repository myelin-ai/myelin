use super::RandomChanceChecker;

/// Random number generator that just counts its calls
/// until a statistical probability has been met.
#[derive(Debug, Clone, Default)]
pub struct AccumulativeDeterministicChanceChecker {
    coin_flip_counter: f64,
    element_counter: i32,
}

impl RandomChanceChecker for AccumulativeDeterministicChanceChecker {
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool {
        if probability < 0.0 || probability > 1.0 {
            let error = format!(
                "Expected probability to be in range [0.0; 1.0] but got {}",
                probability
            );
            panic!(error);
        }
        self.coin_flip_counter += probability;
        let chance_has_been_hit = self.coin_flip_counter >= 1.0;
        if chance_has_been_hit {
            self.coin_flip_counter = 0.0;
        }
        chance_has_been_hit
    }

    fn random_number_in_range(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            panic!("min cannot be greater or equal to max")
        }

        if self.element_counter >= max {
            self.element_counter = min;
        } else if self.element_counter < min {
            self.element_counter = min;
        }

        let returned_number = self.element_counter;
        self.element_counter += 1;
        returned_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_true_on_100_percent_chance() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        let coin_flip_result = random_chance_checker.flip_coin_with_probability(1.0);
        assert!(coin_flip_result);
    }

    #[test]
    fn flips_false_on_0_percent_chance() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        let coin_flip_result = random_chance_checker.flip_coin_with_probability(0.0);
        assert!(!coin_flip_result);
    }

    #[test]
    fn does_not_hit_chance_with_too_little_accumulation() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        let coin_flip_result = random_chance_checker.flip_coin_with_probability(0.75);
        assert!(!coin_flip_result);
    }

    #[test]
    fn hits_chance_with_enough_accumulation() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        let coin_flip_result = random_chance_checker.flip_coin_with_probability(0.75);
        assert!(!coin_flip_result);

        let coin_flip_result = random_chance_checker.flip_coin_with_probability(0.25);
        assert!(coin_flip_result);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_probability() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        random_chance_checker.flip_coin_with_probability(-1.0);
    }

    #[should_panic]
    #[test]
    fn panics_too_high_probability() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        random_chance_checker.flip_coin_with_probability(1.1);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_higher_than_max() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        random_chance_checker.random_number_in_range(1, 0);
    }

    #[should_panic]
    #[test]
    fn panics_when_min_is_max() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        const ONLY_BOUND: i32 = 1;
        random_chance_checker.random_number_in_range(ONLY_BOUND, ONLY_BOUND);
    }

    #[test]
    fn returns_zero_per_default() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        let number = random_chance_checker.random_number_in_range(-4, 5);
        assert_eq!(0, number);
    }

    #[test]
    fn returned_number_is_incremented() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        random_chance_checker.random_number_in_range(-4, 5);
        let number = random_chance_checker.random_number_in_range(-4, 5);
        assert_eq!(1, number);
    }

    #[test]
    fn returned_number_overflows_into_min() {
        let mut random_chance_checker = AccumulativeDeterministicChanceChecker::default();
        random_chance_checker.random_number_in_range(-4, 5);
        let number = random_chance_checker.random_number_in_range(-1, 1);
        assert_eq!(-1, number);
    }
}
