//! Types relating to a behavior that reproduces at random intervals

use myelin_environment::object::*;
use myelin_environment::object_builder::ObjectBuilder;
use std::fmt;
use std::ops::RangeBounds;

mod random_chance_checker_impl;
pub use self::random_chance_checker_impl::RandomChanceCheckerImpl;
mod accumulative_deterministic;
pub use self::accumulative_deterministic::AccumulativeDeterministicChanceChecker;

/// An [`ObjectBehavior`] that spreads itself in random intervals.
/// The spreading has a chance to occur in every step
/// if there is space available in an area around it
#[derive(Debug)]
pub struct StochasticSpreading {
    random_chance_checker: Box<dyn RandomChanceChecker>,
    spreading_sensor: Sensor,
    spreading_probability: f64,
}

impl Clone for StochasticSpreading {
    fn clone(&self) -> StochasticSpreading {
        Self {
            random_chance_checker: self.random_chance_checker.clone_box(),
            spreading_sensor: self.spreading_sensor.clone(),
            spreading_probability: self.spreading_probability.clone(),
        }
    }
}

impl StochasticSpreading {
    /// Returns a plant that has a probability of `spreading_probability`
    /// `spreading_sensor` is the area around the plant, in which it will try to
    /// find a vacant spot to spread.
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

    fn spread(
        &self,
        own_description: &ObjectDescription,
        _sensor_collisions: &[ObjectDescription],
    ) -> Option<Action> {
        let object_description = ObjectBuilder::default()
            .location(
                own_description.position.location.x + 10,
                own_description.position.location.y + 10,
            )
            .rotation(own_description.position.rotation)
            .shape(own_description.shape.clone())
            .kind(own_description.kind)
            .mobility(own_description.mobility.clone())
            .build()
            .unwrap();
        let object_behavior = Box::new(self.clone());
        Some(Action::Reproduce(object_description, object_behavior))
    }
}

impl ObjectBehavior for StochasticSpreading {
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        sensor_collisions: &[ObjectDescription],
    ) -> Option<Action> {
        if self.should_spread() {
            self.spread(own_description, sensor_collisions)
        } else {
            None
        }
    }
}

/// Dedicated random number generator
pub trait RandomChanceChecker: fmt::Debug + RandomChanceCheckerClone {
    /// Returns a random boolean with a given probability of returning true.
    /// The probability is defined in the range `[0.0; 1.0]` where `0.0` means
    /// always return `false` and `1.0` means always return `true`.
    /// # Errors
    /// Is allowed to panic if `probability` is outside the range [0.0; 1.0]
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool;

    /// Returns a random element from the specified range [min; max)
    fn random_number_in_range(&mut self, min: i32, max: i32) -> i32;
}

/// Supertrait used to make sure that all implementors
/// of [`RandomChanceChecker`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`ObjectBehavior`]: ./trait.RandomChanceChecker.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait RandomChanceCheckerClone {
    fn clone_box(&self) -> Box<dyn RandomChanceChecker>;
}

impl<T> RandomChanceCheckerClone for T
where
    T: RandomChanceChecker + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn RandomChanceChecker> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use std::cell::RefCell;
    use std::thread::panicking;

    const SPREADING_CHANGE: f64 = 1.0 / (60.0 * 30.0);

    #[test]
    fn does_nothing_when_chance_is_not_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, false);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();
        let action = object.step(&own_description, &[]);
        assert!(action.is_none());
    }

    #[test]
    fn spreads_when_chance_is_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();
        let action = object.step(&own_description, &[]);
        match action {
            Some(Action::Reproduce(_, _)) => {}
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    #[test]
    fn does_not_spread_when_surrounded() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();

        let collisions = vec![
            object_description_at_location(45, 45),
            object_description_at_location(50, 45),
            object_description_at_location(55, 45),
            object_description_at_location(55, 50),
            object_description_at_location(55, 55),
            object_description_at_location(50, 55),
            object_description_at_location(45, 55),
            object_description_at_location(45, 50),
        ];

        let action = object.step(&own_description, &collisions);
        assert!(action.is_none());
    }

    #[test]
    fn spreads_on_only_available_space() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();

        let collisions = vec![
            object_description_at_location(45, 45),
            object_description_at_location(50, 45),
            object_description_at_location(55, 45),
            object_description_at_location(55, 55),
            object_description_at_location(50, 55),
            object_description_at_location(45, 55),
            object_description_at_location(45, 50),
        ];

        let action = object.step(&own_description, &collisions);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                // To do: Adjust for padding
                let expected_object_description = object_description_at_location(55, 50);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    fn object_description() -> ObjectDescription {
        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-5, -5)
                    .vertex(5, -5)
                    .vertex(5, 5)
                    .vertex(-5, 5)
                    .build()
                    .unwrap(),
            )
            .location(50, 50)
            .mobility(Mobility::Immovable)
            .kind(Kind::Plant)
            .build()
            .unwrap()
    }

    fn object_description_at_location(x: u32, y: u32) -> ObjectDescription {
        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-5, -5)
                    .vertex(5, -5)
                    .vertex(5, 5)
                    .vertex(-5, 5)
                    .build()
                    .unwrap(),
            )
            .location(x, y)
            .mobility(Mobility::Immovable)
            .kind(Kind::Plant)
            .build()
            .unwrap()
    }

    fn sensor() -> Sensor {
        Sensor {
            shape: PolygonBuilder::default()
                .vertex(-20, -20)
                .vertex(20, -20)
                .vertex(20, 20)
                .vertex(-20, 20)
                .build()
                .unwrap(),
            position: Position::default(),
        }
    }

    #[derive(Debug, Default, Clone)]
    struct RandomChanceCheckerMock {
        expect_flip_coin_with_probability_and_return: Option<(f64, bool)>,
        expect_random_number_in_range_and_return: Option<(i32, i32, i32)>,

        flip_coin_with_probability_was_called: RefCell<bool>,
        random_number_in_range_was_called: RefCell<bool>,
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

        pub(crate) fn expect_random_number_in_range_and_return(
            &mut self,
            min: i32,
            max: i32,
            returned_value: i32,
        ) {
            self.expect_random_number_in_range_and_return = Some((min, max, returned_value));
        }
    }

    impl Drop for RandomChanceCheckerMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }

            if self.expect_flip_coin_with_probability_and_return.is_some() {
                assert!(
                    *self.flip_coin_with_probability_was_called.borrow(),
                    "flip_coin_with_probability() was not called, but was expected"
                )
            }

            if self.expect_random_number_in_range_and_return.is_some() {
                assert!(
                    *self.random_number_in_range_was_called.borrow(),
                    "random_number_in_range() was not called, but was expected"
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

        fn random_number_in_range(&mut self, min: i32, max: i32) -> i32 {
            *self.random_number_in_range_was_called.borrow_mut() = true;
            if let Some((ref expected_min, ref expected_max, ref return_value)) =
                self.expect_random_number_in_range_and_return
            {
                if min == *expected_min && max == *expected_max {
                    return_value.clone()
                } else {
                    panic!(
                        "random_number_in_range() was called with {:?} and {:?}, expected {:?} and {:?}",
                        min, max, expected_min, expected_max
                    )
                }
            } else {
                panic!("random_number_in_range() was called unexpectedly")
            }
        }
    }
}
