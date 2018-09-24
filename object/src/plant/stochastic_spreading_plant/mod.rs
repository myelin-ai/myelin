use myelin_environment::object::{ImmovableAction, ImmovableObject, Kind};
use std::fmt;

mod random_chance_checker_impl;
pub use self::random_chance_checker_impl::RandomChanceCheckerImpl;

/// Plant that spreads in stochastic intervals
#[derive(Debug)]
pub struct StochasticSpreadingPlant {
    random_chance_checker: Box<dyn RandomChanceChecker>,
}

impl StochasticSpreadingPlant {
    pub fn new(random_chance_checker: Box<dyn RandomChanceChecker>) -> Self {
        Self {
            random_chance_checker,
        }
    }
}

impl ImmovableObject for StochasticSpreadingPlant {
    fn step(&mut self) -> Vec<ImmovableAction> {
        unimplemented!()
    }
    fn kind(&self) -> Kind {
        Kind::Plant
    }
}

/// Dedicated random number generator
pub trait RandomChanceChecker: fmt::Debug {
    /// Returns a random boolean with a given probability of returning true.
    /// The probability is defined in the range `[0.0; 1.0]` where `0.0` means
    /// always return `false` and `1.0` means always return `true`.
    /// # Errors
    /// Is allowed to panic if `probability` is outside the range [0.0; 1.0]
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;

    #[test]
    fn is_correct_kind() {
        let random_chance_checker = Box::new(RandomChanceCheckerMock::with_coin_flip_result(true));
        let object = StochasticSpreadingPlant::new(random_chance_checker);
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }

    #[derive(Debug)]
    struct RandomChanceCheckerMock {
        coin_flip_result: bool,
    }
    impl RandomChanceCheckerMock {
        fn with_coin_flip_result(coin_flip_result: bool) -> Self {
            Self { coin_flip_result }
        }
    }

    impl RandomChanceChecker for RandomChanceCheckerMock {
        fn flip_coin_with_probability(&mut self, _probability: f64) -> bool {
            self.coin_flip_result
        }
    }
}
