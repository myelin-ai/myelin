use myelin_environment::object::{ImmovableAction, ImmovableObject, Kind};
use std::fmt;

mod random_chance_checker_impl;
pub use self::random_chance_checker_impl::RandomChanceCheckerImpl;

/// Plant that spreads in stochastic intervals
#[derive(Debug)]
pub struct StochasticSpreadingPlant {
    random_chance_checker: Box<dyn RandomChanceChecker>,
    spreading_chance: f64,
}

impl StochasticSpreadingPlant {
    pub fn with_spreading_chance(
        spreading_chance: f64,
        random_chance_checker: Box<dyn RandomChanceChecker>,
    ) -> Self {
        Self {
            spreading_chance,
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
    use std::cell::RefCell;
    const SPREADING_CHANGE: f64 = 1.0 / (60.0 * 30.0);

    #[test]
    fn is_correct_kind() {
        let random_chance_checker = RandomChanceCheckerMock::new();
        let object = StochasticSpreadingPlant::with_spreading_chance(
            SPREADING_CHANGE,
            Box::new(random_chance_checker),
        );
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }

    #[test]
    fn does_nothing_when_chance_is_not_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, false);
        let mut object = StochasticSpreadingPlant::with_spreading_chance(
            SPREADING_CHANGE,
            Box::new(random_chance_checker),
        );
        let actions = object.step();
        assert!(actions.is_empty());
    }

    #[derive(Debug, Default)]
    struct RandomChanceCheckerMock {
        expect_flip_coin_with_probability_and_return: Option<(f64, bool)>,
        flip_coin_with_probability_was_called: RefCell<bool>,
    }

    impl RandomChanceCheckerMock {
        pub(crate) fn new() -> Self {
            Default::default()
        }

        pub(crate) fn expect_flip_coin_with_probability_and_return(
            &mut self,
            probability: f64,
            returned_value: bool,
        ) {
            self.expect_flip_coin_with_probability_and_return = Some((probability, returned_value));
        }
    }

    impl Drop for RandomChanceCheckerMock {
        fn drop(&mut self) {
            if self.expect_flip_coin_with_probability_and_return.is_some() {
                assert!(
                    *self.flip_coin_with_probability_was_called.borrow(),
                    "body() was not called, but was expected"
                )
            }
        }
    }

    impl RandomChanceChecker for RandomChanceCheckerMock {
        fn flip_coin_with_probability(&mut self, probability: f64) -> bool {
            *self.flip_coin_with_probability_was_called.borrow_mut() = true;
            if let Some((ref expected_probability, ref return_value)) =
                self.expect_flip_coin_with_probability_and_return
            {
                if probability == *expected_probability {
                    return_value.clone()
                } else {
                    panic!(
                        "flip_coin_with_probability() was called with {:?}, expected {:?}",
                        probability, expected_probability
                    )
                }
            } else {
                panic!("body() was called unexpectedly")
            }
        }
    }
}
