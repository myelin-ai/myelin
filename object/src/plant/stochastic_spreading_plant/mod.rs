use myelin_environment::object::*;
use std::fmt;

mod random_chance_checker_impl;
pub use self::random_chance_checker_impl::RandomChanceCheckerImpl;

/// Plant that spreads in stochastic intervals
#[derive(Debug)]
pub struct StochasticSpreadingPlant {
    random_chance_checker: Box<dyn RandomChanceChecker>,
    spreading_sensor: Sensor,
    spreading_probability: f64,
}

impl StochasticSpreadingPlant {
    /// Returns a plant that has a probability of `spreading_probability`
    /// in every step to spawn a new plant, using the injected [`RandomChanceChecker`]
    /// to check if the probability was met.
    /// `spreading_sensor` is the area around the plant, in which it will try to
    /// find a vacant spot to spread
    ///
    /// [`RandomChanceChecker`]: ./trait.RandomChanceChecker.html
    pub fn new(
        spreading_probability: f64,
        spreading_sensor: Sensor,
        random_chance_checker: Box<dyn RandomChanceChecker>,
    ) -> Self {
        Self {
            spreading_probability,
            spreading_sensor,
            random_chance_checker,
        }
    }

    fn should_spread(&mut self) -> bool {
        self.random_chance_checker
            .flip_coin_with_probability(self.spreading_probability)
    }

    fn spread(&self, _sensor_collisions: &[ObjectDescription]) -> Vec<ImmovableAction> {
        unimplemented!()
    }
}

impl ImmovableObject for StochasticSpreadingPlant {
    fn step(&mut self, sensor_collisions: &[ObjectDescription]) -> Vec<ImmovableAction> {
        if self.should_spread() {
            self.spread(sensor_collisions)
        } else {
            Vec::new()
        }
    }

    fn kind(&self) -> Kind {
        Kind::Plant
    }

    fn sensor(&self) -> Option<Sensor> {
        Some(self.spreading_sensor.clone())
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
    use myelin_environment::object_builder::PolygonBuilder;
    use std::cell::RefCell;

    const SPREADING_CHANGE: f64 = 1.0 / (60.0 * 30.0);

    #[test]
    fn is_correct_kind() {
        let random_chance_checker = RandomChanceCheckerMock::new();
        let object = StochasticSpreadingPlant::new(
            SPREADING_CHANGE,
            sensor(),
            Box::new(random_chance_checker),
        );
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }

    #[test]
    fn does_nothing_when_chance_is_not_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, false);
        let mut object = StochasticSpreadingPlant::new(
            SPREADING_CHANGE,
            sensor(),
            Box::new(random_chance_checker),
        );
        let actions = object.step(&[]);
        assert!(actions.is_empty());
    }

    #[test]
    fn returns_injected_sensor() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, false);
        let expected_sensor = sensor();
        let object = StochasticSpreadingPlant::new(
            SPREADING_CHANGE,
            expected_sensor.clone(),
            Box::new(random_chance_checker),
        );
        let sensor = object.sensor().expect("Object has no sensor");
        assert_eq!(expected_sensor, sensor);
    }

    fn sensor() -> Sensor {
        Sensor {
            shape: PolygonBuilder::new()
                .vertex(-5, -5)
                .vertex(5, -5)
                .vertex(5, 5)
                .vertex(-5, 5)
                .build()
                .unwrap(),
            position: Position::default(),
        }
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
                    "flip_coin_with_probability() was not called, but was expected"
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
                panic!("flip_coin_with_probability() was called unexpectedly")
            }
        }
    }
}
