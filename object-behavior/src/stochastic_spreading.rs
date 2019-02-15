//! Types relating to a behavior that reproduces at random intervals

use myelin_engine::prelude::*;
use std::any::Any;
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
        world_interactor: &dyn WorldInteractor,
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
            .find(|&location| can_spread_at_location(&own_description, location, world_interactor));
        if let Some(spreading_location) = spreading_location {
            let object_description = ObjectBuilder::from(own_description.clone())
                .location(spreading_location.x, spreading_location.y)
                .build()
                .unwrap();
            let object_behavior = Box::new(self.clone());
            Some(Action::Spawn(object_description, object_behavior))
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
    world_interactor: &dyn WorldInteractor,
) -> bool {
    let target_area = own_description
        .shape
        .translate(location)
        .rotate_around_point(own_description.rotation, own_description.location)
        .aabb();

    let objects_in_area = world_interactor.find_objects_in_area(target_area);

    objects_in_area.is_empty()
}

impl ObjectBehavior for StochasticSpreading {
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        world_interactor: &dyn WorldInteractor,
    ) -> Option<Action> {
        if self.should_spread() {
            self.spread(own_description, world_interactor)
        } else {
            None
        }
    }

    fn as_any(&self) -> &'_ dyn Any {
        self
    }
}

/// Dedicated random number generator
#[cfg_attr(test, mockiato::mockable)]
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

impl Clone for Box<dyn RandomChanceChecker> {
    fn clone(&self) -> Self {
        self.clone_box()
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
    use std::cell::RefCell;

    const SPREADING_CHANGE: f64 = 1.0 / (60.0 * 30.0);
    const EXPECTED_PADDING: f64 = 1.0;

    #[test]
    fn does_nothing_when_chance_is_not_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(false);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);
        let action = object.step(&own_description, &WorldInteractorMock::new());
        assert!(action.is_none());
    }

    #[test]
    fn spreads_when_chance_is_hit() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(0);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((34.0, 34.0), (44.0, 44.0)),
            Vec::new(),
        );
        let action = object.step(&own_description, &world_interactor);
        match action {
            Some(Action::Spawn(object_description, _)) => {
                let expected_object_description = object_description_at_location(
                    40.0 - EXPECTED_PADDING,
                    40.0 - EXPECTED_PADDING,
                );
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
        }
    }

    #[test]
    fn does_not_spread_when_surrounded() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(0);

        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((34.0, 34.0), (44.0, 44.0)),
            vec![Object {
                id: 0,
                description: object_description_at_location(39.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((45.0, 34.0), (55.0, 44.0)),
            vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 34.0), (66.0, 44.0)),
            vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 45.0), (66.0, 55.0)),
            vec![Object {
                id: 3,
                description: object_description_at_location(61.0, 50.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 56.0), (66.0, 66.0)),
            vec![Object {
                id: 4,
                description: object_description_at_location(61.0, 61.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((45.0, 56.0), (55.0, 66.0)),
            vec![Object {
                id: 5,
                description: object_description_at_location(50.0, 61.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((34.0, 56.0), (44.0, 66.0)),
            vec![Object {
                id: 6,
                description: object_description_at_location(39.0, 61.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((34.0, 45.0), (44.0, 55.0)),
            vec![Object {
                id: 7,
                description: object_description_at_location(39.0, 50.0),
                behavior: mock_behavior.borrow(),
            }],
        );

        let action = object.step(&own_description, &world_interactor);
        assert!(action.is_none());
    }

    #[test]
    fn spreads_on_first_available_space_clockwise() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(0);

        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((34.0, 34.0), (44.0, 44.0)),
            vec![Object {
                id: 0,
                description: object_description_at_location(39.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((45.0, 34.0), (55.0, 44.0)),
            vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 34.0), (66.0, 44.0)),
            vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 45.0), (66.0, 55.0)),
            Vec::new(),
        );

        let action = object.step(&own_description, &world_interactor);
        match action {
            Some(Action::Spawn(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(60.0 + EXPECTED_PADDING, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
        }
    }

    #[test]
    fn can_spread_vertically() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(1);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((45.0, 34.0), (55.0, 44.0)),
            vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 34.0), (66.0, 44.0)),
            vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 45.0), (66.0, 55.0)),
            vec![Object {
                id: 3,
                description: object_description_at_location(61.0, 50.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 56.0), (66.0, 66.0)),
            vec![Object {
                id: 4,
                description: object_description_at_location(61.0, 61.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((45.0, 56.0), (55.0, 66.0)),
            Vec::new(),
        );

        let action = object.step(&own_description, &world_interactor);
        match action {
            Some(Action::Spawn(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(50.0, 60.0 + EXPECTED_PADDING);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
        }
    }

    #[test]
    fn can_spread_horizontally() {
        let mut random_chance_checker = RandomChanceCheckerMock::new();
        random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(1);
        let mut object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(random_chance_checker));
        let own_description = object_description_at_location(50.0, 50.0);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((45.0, 34.0), (55.0, 44.0)),
            vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 34.0), (66.0, 44.0)),
            vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.borrow(),
            }],
        );
        world_interactor.expect_find_objects_in_area_and_return(
            Aabb::new((56.0, 45.0), (66.0, 55.0)),
            Vec::new(),
        );

        let action = object.step(&own_description, &world_interactor);
        match action {
            Some(Action::Spawn(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(60.0 + EXPECTED_PADDING, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
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
            .build()
            .unwrap()
    }

    fn mock_behavior() -> RefCell<Box<dyn ObjectBehavior>> {
        RefCell::new(Box::new(ObjectBehaviorMock::new()))
    }
}
