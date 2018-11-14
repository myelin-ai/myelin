//! Types relating to a behavior that reproduces at random intervals

use myelin_environment::object::*;
use myelin_environment::Id;
use myelin_geometry::*;
use std::collections::HashMap;
use std::fmt;

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
        &mut self,
        own_description: &ObjectDescription,
        sensor_collisions: &HashMap<Id, ObjectDescription>,
    ) -> Option<Action> {
        const POSSIBLE_SPREADING_LOCATIONS: [Point; 8] = [
            Point { x: -10.0, y: -10.0 },
            Point { x: 0.0, y: -10.0 },
            Point { x: 10.0, y: -10.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
            Point { x: -10.0, y: 10.0 },
            Point { x: -10.0, y: 0.0 },
        ];

        let first_try_index = self
            .random_chance_checker
            .random_number_in_range(0, POSSIBLE_SPREADING_LOCATIONS.len() as i32)
            as usize;

        let spreading_location = POSSIBLE_SPREADING_LOCATIONS
            .iter()
            .skip(first_try_index as usize)
            .chain(POSSIBLE_SPREADING_LOCATIONS.iter().take(first_try_index))
            .map(|&point| own_description.location + point)
            .find(|&location| can_spread_at_location(location, sensor_collisions));
        if let Some(spreading_location) = spreading_location {
            let object_description = ObjectBuilder::default()
                .location(spreading_location.x, spreading_location.y)
                .rotation(own_description.rotation)
                .shape(own_description.shape.clone())
                .kind(own_description.kind)
                .mobility(own_description.mobility.clone())
                .build()
                .unwrap();
            let object_behavior = Box::new(self.clone());
            Some(Action::Reproduce(object_description, object_behavior))
        } else {
            None
        }
    }
}

fn can_spread_at_location(
    location: Point,
    sensor_collisions: &HashMap<Id, ObjectDescription>,
) -> bool {
    !sensor_collisions
        .values()
        .map(|object_description| {
            object_description
                .shape
                .translate(object_description.location)
                .rotate_around_point(object_description.rotation, object_description.location)
        })
        .any(|polygon| polygon.contains_point(location))
}

impl ObjectBehavior for StochasticSpreading {
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        sensor_collisions: &HashMap<Id, ObjectDescription>,
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
        let action = object.step(&own_description, &HashMap::new());
        assert!(action.is_none());
    }

    #[test]
    fn spreads_when_chance_is_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 0);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();
        let action = object.step(&own_description, &HashMap::new());
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                // To do: Adjust for padding
                let expected_object_description = object_description_at_location(40.0, 40.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    #[test]
    fn does_not_spread_when_surrounded() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 0);

        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();

        let collisions = hashmap![
            0 => object_description_at_location(40.0, 40.0),
            1 => object_description_at_location(50.0, 40.0),
            2 => object_description_at_location(60.0, 40.0),
            3 => object_description_at_location(60.0, 50.0),
            4 => object_description_at_location(60.0, 60.0),
            5 => object_description_at_location(50.0, 60.0),
            6 => object_description_at_location(40.0, 60.0),
            7 => object_description_at_location(40.0, 50.0),
        ];

        let action = object.step(&own_description, &collisions);
        assert!(action.is_none());
    }

    #[test]
    fn spreads_on_only_available_space() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 0);

        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();

        let collisions = hashmap![
            0 => object_description_at_location(40.0, 40.0),
            1 => object_description_at_location(50.0, 40.0),
            2 => object_description_at_location(60.0, 40.0),
            3 => object_description_at_location(60.0, 60.0),
            4 => object_description_at_location(50.0, 60.0),
            5 => object_description_at_location(40.0, 60.0),
            6 => object_description_at_location(40.0, 50.0),
        ];

        let action = object.step(&own_description, &collisions);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                // To do: Adjust for padding
                let expected_object_description = object_description_at_location(60.0, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    #[test]
    fn spreads_on_available_space_treating_random_location_clockwise() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 1);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, sensor(), Box::new(random_chance_checker));
        let own_description = object_description();

        let collisions = hashmap![
            0 => object_description_at_location(40.0, 40.0),
            1 => object_description_at_location(60.0, 40.0),
            2 => object_description_at_location(60.0, 50.0),
            3 => object_description_at_location(50.0, 60.0),
            4 => object_description_at_location(40.0, 50.0),
        ];

        let action = object.step(&own_description, &collisions);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                // To do: Adjust for padding
                let expected_object_description = object_description_at_location(50.0, 40.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    fn object_description() -> ObjectDescription {
        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(5.0, -5.0)
                    .vertex(5.0, 5.0)
                    .vertex(-5.0, 5.0)
                    .build()
                    .unwrap(),
            )
            .location(50.0, 50.0)
            .mobility(Mobility::Immovable)
            .kind(Kind::Plant)
            .build()
            .unwrap()
    }

    fn object_description_at_location(x: f64, y: f64) -> ObjectDescription {
        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(5.0, -5.0)
                    .vertex(5.0, 5.0)
                    .vertex(-5.0, 5.0)
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
                .vertex(-20.0, -20.0)
                .vertex(20.0, -20.0)
                .vertex(20.0, 20.0)
                .vertex(-20.0, 20.0)
                .build()
                .unwrap(),
            location: Point::default(),
            rotation: Radians::default(),
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