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
    next_spreading_location: Option<Aabb>,
}

impl Clone for StochasticSpreading {
    fn clone(&self) -> StochasticSpreading {
        Self {
            random_chance_checker: self.random_chance_checker.clone_box(),
            spreading_probability: self.spreading_probability,
            next_spreading_location: self.next_spreading_location.clone(),
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
            next_spreading_location: None,
        }
    }

    /// Returns the position, if any, where this behavior will spread to in the next step
    pub fn next_spreading_location(&self) -> Option<Aabb> {
        self.next_spreading_location
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
        possible_spreading_locations
            .iter()
            .cycle()
            .skip(first_try_index)
            .take(possible_spreading_locations.len())
            .map(|&point| own_description.location + point)
            .find(|&location| can_spread_at_location(&own_description, location, world_interactor))
            .and_then(|spreading_location| {
                let object_description = ObjectBuilder::from(own_description.clone())
                    .location(spreading_location.x, spreading_location.y)
                    .build()
                    .unwrap();

                self.next_spreading_location =
                    Some(spreading_location_aabb(own_description, spreading_location));

                let object_behavior = Box::new(self.clone());
                Some(Action::Spawn(object_description, object_behavior))
            })
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
    let (width, height) = target_area_width_and_height(polygon.aabb());

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
    let target_area = spreading_location_aabb(own_description, location);

    let objects_in_area = world_interactor.find_objects_in_area(target_area);

    if !objects_in_area.is_empty() {
        return false;
    }

    let (target_area_width, target_area_height) = target_area_width_and_height(target_area);

    let possible_other_spreaders = Aabb::try_new(
        (
            target_area.upper_left.x - target_area_width,
            target_area.upper_left.y - target_area_height,
        ),
        (
            target_area.lower_right.x + target_area_width,
            target_area.lower_right.y + target_area_height,
        ),
    )
    .expect("");
    world_interactor
        .find_objects_in_area(possible_other_spreaders)
        .into_iter()
        // Todo: Check IDs instead, once engine hands out this object's ID
        .filter(|object| object.description != *own_description)
        .filter_map(|object| {
            object
                .behavior
                .as_any()
                .downcast_ref::<StochasticSpreading>()
        })
        .filter_map(|stochastic_spreading| stochastic_spreading.next_spreading_location)
        .all(|other_target_location| !target_area.intersects(other_target_location))
}

/// Arbitrary number in meters representing the space
/// between the spread objects
const PADDING: f64 = 1.0;

fn target_area_width_and_height(area: Aabb) -> (f64, f64) {
    let Aabb {
        upper_left: Point { x: min_x, y: min_y },
        lower_right: Point { x: max_x, y: max_y },
    } = area;

    let width = max_x - min_x + PADDING;
    let height = max_y - min_y + PADDING;

    (width, height)
}

fn spreading_location_aabb(own_description: &ObjectDescription, spreading_location: Point) -> Aabb {
    own_description
        .shape
        .translate(spreading_location)
        .rotate_around_point(own_description.rotation, own_description.location)
        .aabb()
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
#[cfg_attr(test, mockiato::mockable(static_references))]
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
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((34.0, 34.0), (44.0, 44.0)).unwrap(),
            ))
            .returns(Vec::new());
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((23.0, 23.0), (55.0, 55.0)).unwrap(),
            ))
            .returns(Vec::new());

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
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((34.0, 34.0), (44.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 0,
                description: object_description_at_location(39.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 3,
                description: object_description_at_location(61.0, 50.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 56.0), (66.0, 66.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 4,
                description: object_description_at_location(61.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 56.0), (55.0, 66.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 5,
                description: object_description_at_location(50.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((34.0, 56.0), (44.0, 66.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 6,
                description: object_description_at_location(39.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((34.0, 45.0), (44.0, 55.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 7,
                description: object_description_at_location(39.0, 50.0),
                behavior: mock_behavior.as_ref(),
            }]);

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
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((34.0, 34.0), (44.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 0,
                description: object_description_at_location(39.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap(),
            ))
            .returns(Vec::new());
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap(),
            ))
            .returns(Vec::new());

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
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 3,
                description: object_description_at_location(61.0, 50.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 56.0), (66.0, 66.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 4,
                description: object_description_at_location(61.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 56.0), (55.0, 66.0)).unwrap(),
            ))
            .returns(Vec::new());

        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((34.0, 45.0), (66.0, 77.0)).unwrap(),
            ))
            .returns(Vec::new());

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
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap(),
            ))
            .returns(Vec::new());

        world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap(),
            ))
            .returns(Vec::new());

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
    fn does_not_spread_on_space_that_another_behavior_will_spread_to() {
        let mock_behavior = mock_behavior();

        let mut first_random_chance_checker = RandomChanceCheckerMock::new();
        first_random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        first_random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(3);
        let mut first_object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(first_random_chance_checker));
        let first_description = object_description_at_location(50.0, 50.0);

        let mut first_world_interactor = WorldInteractorMock::new();
        first_world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap(),
            ))
            .returns(Vec::new());

        first_world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap(),
            ))
            .returns(Vec::new());

        let first_action = first_object.step(&first_description.clone(), &first_world_interactor);
        match first_action {
            Some(Action::Spawn(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(60.0 + EXPECTED_PADDING, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
        }

        let first_object = first_object;

        let mut second_random_chance_checker = RandomChanceCheckerMock::new();
        second_random_chance_checker
            .expect_flip_coin_with_probability(partial_eq(SPREADING_CHANGE))
            .returns(true);
        second_random_chance_checker
            .expect_random_number_in_range(partial_eq(0), partial_eq(8))
            .returns(7);
        let mut second_object =
            StochasticSpreading::new(SPREADING_CHANGE, Box::new(second_random_chance_checker));
        let second_description = object_description_at_location(70.0, 50.0);

        let mut second_world_interactor = WorldInteractorMock::new();

        second_world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((54.0, 34.0), (64.0, 44.0)).unwrap(),
            ))
            .returns(Vec::new());
        second_world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((54.0, 45.0), (64.0, 55.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(
                    60.0 - EXPECTED_PADDING,
                    40.0 - EXPECTED_PADDING,
                ),
                behavior: mock_behavior.as_ref(),
            }]);
        second_world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((43.0, 23.0), (75.0, 55.0)).unwrap(),
            ))
            .returns(Vec::new());
        second_world_interactor
            .expect_find_objects_in_area(partial_eq(
                Aabb::try_new((43.0, 34.0), (75.0, 66.0)).unwrap(),
            ))
            .returns(vec![Object {
                id: 0,
                description: first_description,
                behavior: &first_object,
            }]);

        let second_action = second_object.step(&second_description, &second_world_interactor);
        match second_action {
            Some(Action::Spawn(object_description, _)) => {
                // Expected order of operations for the second behavior:
                // 1. Try to spread left, which fails, because the first behavior already spreads there
                // 2. Try to spread to the upper left, which is already occupied
                // 3. Spread to the top
                let expected_object_description =
                    object_description_at_location(70.0, 40.0 - EXPECTED_PADDING);
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

    fn mock_behavior() -> Box<dyn ObjectBehavior> {
        Box::new(ObjectBehaviorMock::new())
    }
}
