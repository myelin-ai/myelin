//! Types relating to a behavior that reproduces at random intervals

use myelin_environment::object::*;
use myelin_geometry::*;
use std::fmt;

mod random_chance_checker_impl;
pub use self::random_chance_checker_impl::RandomChanceCheckerImpl;

/// An [`ObjectBehavior`] that spreads itself in random intervals.
/// The spreading has a chance to occur in every step
/// if there is space available in an area around it
#[derive(Debug)]
pub struct StochasticSpreading {
    random_chance_checker: Box<dyn RandomChanceChecker>,
    spreading_probability: f64,
}

impl Clone for StochasticSpreading {
    fn clone(&self) -> StochasticSpreading {
        Self {
            random_chance_checker: self.random_chance_checker.clone_box(),
            spreading_probability: self.spreading_probability,
        }
    }
}

impl StochasticSpreading {
    /// Returns a plant that has a probability of `spreading_probability`
    /// `spreading_sensor` is the area around the object, in which it will try to
    /// find a vacant spot to spread.
    pub fn new(
        spreading_probability: f64,
        random_chance_checker: Box<dyn RandomChanceChecker>,
    ) -> Self {
        Self {
            spreading_probability,
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
        environment: &dyn ObjectEnvironment,
    ) -> Option<Action> {
        let possible_spreading_locations =
            calculate_possible_spreading_locations(&own_description.shape);

        let first_try_index = self
            .random_chance_checker
            .random_number_in_range(0, possible_spreading_locations.len() as i32)
            as usize;

        // Take an iterator over the possible locations, starting at a random index
        let spreading_location = possible_spreading_locations
            .iter()
            .cycle()
            .skip(first_try_index)
            .take(possible_spreading_locations.len())
            .map(|&point| own_description.location + point)
            .find(|&location| can_spread_at_location(&own_description, location, environment));
        if let Some(spreading_location) = spreading_location {
            let object_description = ObjectBuilder::from(own_description.clone())
                .location(spreading_location.x, spreading_location.y)
                .build()
                .unwrap();
            let object_behavior = Box::new(self.clone());
            Some(Action::Reproduce(object_description, object_behavior))
        } else {
            None
        }
    }
}

/// Draws a bounding box around the polygon and returns the 8 adjacend positions
/// to the box, factoring in some padding:
///```other
///  Upper Left | Upper Middle | Upper Right
/// -----------------------------------------
/// Middle Left |    Polygon   | Middle Right
/// -----------------------------------------
///  Lower Left | Lower Middle | Lower Right
/// ```
fn calculate_possible_spreading_locations(polygon: &Polygon) -> Vec<Point> {
    let Aabb {
        upper_left: Point { x: min_x, y: min_y },
        lower_right: Point { x: max_x, y: max_y },
    } = polygon.aabb();

    /// Arbitrary number in meters representing the space
    /// between the spread objects
    const PADDING: f64 = 1.0;

    let width = max_x - min_x + PADDING;
    let height = max_y - min_y + PADDING;

    vec![
        Point {
            x: -width,
            y: -height,
        },
        Point { x: 0.0, y: -height },
        Point {
            x: width,
            y: -height,
        },
        Point { x: width, y: 0.0 },
        Point {
            x: width,
            y: height,
        },
        Point { x: 0.0, y: height },
        Point {
            x: -width,
            y: height,
        },
        Point { x: -width, y: 0.0 },
    ]
}

fn can_spread_at_location(
    own_description: &ObjectDescription,
    location: Point,
    environment: &dyn ObjectEnvironment,
) -> bool {
    let target_area = own_description
        .shape
        .translate(location)
        .rotate_around_point(own_description.rotation, own_description.location)
        .aabb();

    let objects_in_area = environment.find_objects_in_area(target_area);

    objects_in_area.is_empty()
}

impl ObjectBehavior for StochasticSpreading {
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        environment: &dyn ObjectEnvironment,
    ) -> Option<Action> {
        if self.should_spread() {
            self.spread(own_description, environment)
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

///
/// ```other
/// ┼─────────────────────────────────────────────────── x
/// |        Padding: 1       Width: 10
/// |           ←→           ←―――――――→
/// │  ┌───────┐  ┌───────┐  ┌───────┐  ↑
/// │  │ x: 39 │  │ x: 50 │  │ x: 61 │  | Height: 10
/// │  │ y: 39 │  │ y: 39 │  │ y: 39 │  │
/// │  └───────┘  └───────┘  └───────┘  ↓
/// │  ┌───────┐  ┌───────┐  ┌───────┐
/// │  │ x: 39 │  │ x: 50 │  │ x: 61 │
/// │  │ y: 50 │  │ y: 50 │  │ y: 50 │
/// │  └───────┘  └───────┘  └───────┘
/// │  ┌───────┐  ┌───────┐  ┌───────┐
/// │  │ x: 39 │  │ x: 50 │  │ x: 61 │
/// │  │ y: 61 │  │ y: 61 │  │ y: 61 │
/// │  └───────┘  └───────┘  └───────┘
/// y
/// ```
#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use myelin_environment::object::ObjectEnvironmentMock;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::thread::panicking;

    const SPREADING_CHANGE: f64 = 1.0 / (60.0 * 30.0);
    const EXPECTED_PADDING: f64 = 1.0;

    #[test]
    fn does_nothing_when_chance_is_not_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, false);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);
        let action = object.step(&own_description, &ObjectEnvironmentMock::new());
        assert!(action.is_none());
    }

    #[test]
    fn spreads_when_chance_is_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 0);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);
        let mut environment = ObjectEnvironmentMock::new();
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((34.0, 34.0), (44.0, 44.0))))
            .times(1)
            .returns(HashMap::new());
        let action = object.step(&own_description, &environment);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                let expected_object_description = object_description_at_location(
                    40.0 - EXPECTED_PADDING,
                    40.0 - EXPECTED_PADDING,
                );
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    #[test]
    fn does_not_spread_when_surrounded() {
        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 0);

        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mut environment = ObjectEnvironmentMock::new();
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((34.0, 34.0), (44.0, 44.0))))
            .times(1)
            .returns(hashmap! {
                0 => object_description_at_location(39.0, 39.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((45.0, 34.0), (55.0, 44.0))))
            .times(1)
            .returns(hashmap! {
                1 => object_description_at_location(50.0, 39.0)
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 34.0), (66.0, 44.0))))
            .times(1)
            .returns(hashmap! {
               2 => object_description_at_location(60.0, 39.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 45.0), (66.0, 55.0))))
            .times(1)
            .returns(hashmap! {
               3 => object_description_at_location(61.0, 50.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 56.0), (66.0, 66.0))))
            .times(1)
            .returns(hashmap! {
               4 => object_description_at_location(61.0, 61.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((45.0, 56.0), (55.0, 66.0))))
            .times(1)
            .returns(hashmap! {
               5 => object_description_at_location(50.0, 61.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((34.0, 56.0), (44.0, 66.0))))
            .times(1)
            .returns(hashmap! {
               6 => object_description_at_location(39.0, 61.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((34.0, 45.0), (44.0, 55.0))))
            .times(1)
            .returns(hashmap! {
               7 => object_description_at_location(39.0, 50.0),
            });

        let action = object.step(&own_description, &environment);
        assert!(action.is_none());
    }

    #[test]
    fn spreads_on_first_available_space_clockwise() {
        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 0);

        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mut environment = ObjectEnvironmentMock::new();
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((34.0, 34.0), (44.0, 44.0))))
            .times(1)
            .returns(hashmap! {
                0 => object_description_at_location(39.0, 39.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((45.0, 34.0), (55.0, 44.0))))
            .times(1)
            .returns(hashmap! {
                1 => object_description_at_location(50.0, 39.0)
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 34.0), (66.0, 44.0))))
            .times(1)
            .returns(hashmap! {
               2 => object_description_at_location(60.0, 39.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 45.0), (66.0, 55.0))))
            .times(1)
            .returns(HashMap::new());

        let action = object.step(&own_description, &environment);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(60.0 + EXPECTED_PADDING, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    #[test]
    fn can_spread_in_vertically() {
        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 1);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mut environment = ObjectEnvironmentMock::new();
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((45.0, 34.0), (55.0, 44.0))))
            .times(1)
            .returns(hashmap! {
                1 => object_description_at_location(50.0, 39.0)
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 34.0), (66.0, 44.0))))
            .times(1)
            .returns(hashmap! {
               2 => object_description_at_location(60.0, 39.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 45.0), (66.0, 55.0))))
            .times(1)
            .returns(hashmap! {
               3 => object_description_at_location(61.0, 50.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 56.0), (66.0, 66.0))))
            .times(1)
            .returns(hashmap! {
               4 => object_description_at_location(61.0, 61.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((45.0, 56.0), (55.0, 66.0))))
            .times(1)
            .returns(HashMap::new());

        let action = object.step(&own_description, &environment);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(50.0, 60.0 + EXPECTED_PADDING);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
    }

    #[test]
    fn can_spread_horizontally() {
        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin_with_probability_and_return(SPREADING_CHANGE, true);
        random_chance_checker.expect_random_number_in_range_and_return(0, 8, 1);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mut environment = ObjectEnvironmentMock::new();
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((45.0, 34.0), (55.0, 44.0))))
            .times(1)
            .returns(hashmap! {
                1 => object_description_at_location(50.0, 39.0)
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 34.0), (66.0, 44.0))))
            .times(1)
            .returns(hashmap! {
               2 => object_description_at_location(60.0, 39.0),
            });
        environment
            .expect_find_objects_in_area(partial_eq(Aabb::new((56.0, 45.0), (66.0, 55.0))))
            .times(1)
            .returns(HashMap::new());

        let action = object.step(&own_description, &environment);
        match action {
            Some(Action::Reproduce(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(60.0 + EXPECTED_PADDING, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Reproduce, got {:#?}", action),
        }
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

    #[derive(Debug, Default, Clone)]
    struct RandomChanceCheckerMock {
        expect_flip_coin_with_probability_and_return: Option<(f64, bool)>,
        expect_random_number_in_range_and_return: Option<(i32, i32, i32)>,

        flip_coin_with_probability_was_called: RefCell<bool>,
        random_number_in_range_was_called: RefCell<bool>,
    }

    impl RandomChanceCheckerMock {
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
                        "random_number_in_range() was called with {:?} and {:?}, expected {:?} \
                         and {:?}",
                        min, max, expected_min, expected_max
                    )
                }
            } else {
                panic!("random_number_in_range() was called unexpectedly")
            }
        }
    }
}
