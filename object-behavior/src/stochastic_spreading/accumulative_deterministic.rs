use super::RandomChanceChecker;

/// Random number generator that just counts its calls
/// until a statistical probability has been met.
#[derive(Debug, Clone, Default)]
pub struct AccumulativeDeterministicChanceChecker {
    counter: f64,
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
        self.counter += probability;
        let chance_has_been_hit = self.counter >= 1.0;
        if chance_has_been_hit {
            self.counter = 0.0;
        }
        chance_has_been_hit
    }

    fn random_number_in_range(&mut self, min: i32, max: i32) -> i32 {
        unimplemented!()
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
}
