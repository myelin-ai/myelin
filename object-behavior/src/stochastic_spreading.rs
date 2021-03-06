//! Types relating to a behavior that reproduces at random intervals

use myelin_engine::prelude::*;
use myelin_object_data::{AdditionalObjectDescription, Object, ObjectDescription};
use myelin_random::Random;

/// An [`ObjectBehavior`] that spreads itself in random intervals.
/// The spreading has a chance to occur in every step
/// if there is space available in an area around it
#[derive(Debug)]
pub struct StochasticSpreading {
    random: Box<dyn Random>,
    spreading_probability: f64,
    next_spreading_location: Option<Aabb>,
}

impl Clone for StochasticSpreading {
    fn clone(&self) -> Self {
        Self {
            random: self.random.clone_box(),
            spreading_probability: self.spreading_probability,
            next_spreading_location: self.next_spreading_location,
        }
    }
}

impl StochasticSpreading {
    /// Returns a plant that has a probability of `spreading_probability`
    /// `spreading_sensor` is the area around the object, in which it will try to
    /// find a vacant spot to spread.
    pub fn new(spreading_probability: f64, random: Box<dyn Random>) -> Self {
        Self {
            spreading_probability,
            random,
            next_spreading_location: None,
        }
    }

    /// Returns the position, if any, where this behavior will spread to in the next step
    pub fn next_spreading_location(&self) -> Option<Aabb> {
        self.next_spreading_location
    }

    fn should_spread(&self) -> bool {
        self.random
            .flip_coin_with_probability(self.spreading_probability)
    }

    fn spread(
        &mut self,
        world_interactor: &dyn WorldInteractor<AdditionalObjectDescription>,
    ) -> Option<Action<AdditionalObjectDescription>> {
        let own_object = world_interactor.own_object();
        let possible_spreading_locations =
            calculate_possible_spreading_locations(&own_object.description.shape);

        let first_try_index =
            self.random
                .i32_in_range(0, possible_spreading_locations.len() as i32) as usize;

        // Take an iterator over the possible locations, starting at a random index
        possible_spreading_locations
            .iter()
            .cycle()
            .skip(first_try_index)
            .take(possible_spreading_locations.len())
            .map(|&point| own_object.description.location + point)
            .find(|&location| can_spread_at_location(&own_object, location, world_interactor))
            .and_then(|spreading_location| {
                let object_description = ObjectBuilder::from(own_object.description.clone())
                    .location(spreading_location.x, spreading_location.y)
                    .build()
                    .unwrap();

                self.next_spreading_location = Some(spreading_location_aabb(
                    &own_object.description,
                    spreading_location,
                ));

                let object_behavior = box self.clone();
                Some(Action::Spawn(object_description, object_behavior))
            })
    }
}

/// Draws a bounding box around the polygon and returns the 8 adjacend positions
/// to the box, factoring in some padding:
/// ```other
///  Upper Left | Upper Middle | Upper Right
/// -----------------------------------------
/// Middle Left |    Polygon   | Middle Right
/// -----------------------------------------
///  Lower Left | Lower Middle | Lower Right
/// ```
fn calculate_possible_spreading_locations(polygon: &Polygon) -> [Point; 8] {
    let (width, height) = width_and_height_of_area(polygon.aabb());

    [
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
    own_object: &Object<'_>,
    location: Point,
    world_interactor: &dyn WorldInteractor<AdditionalObjectDescription>,
) -> bool {
    let target_area = spreading_location_aabb(&own_object.description, location);

    let objects_in_area = world_interactor.find_objects_in_area(target_area);

    if !objects_in_area.is_empty() {
        return false;
    }

    let (target_area_width, target_area_height) = width_and_height_of_area(target_area);

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
    .unwrap();

    world_interactor
        .find_objects_in_area(possible_other_spreaders)
        .into_iter()
        .filter(|object| object.id != own_object.id)
        .filter_map(|object| {
            object
                .behavior
                .as_any()
                .downcast_ref::<StochasticSpreading>()
        })
        .filter_map(|stochastic_spreading| stochastic_spreading.next_spreading_location)
        .all(|other_target_location| !target_area.intersects(&other_target_location))
}

/// Arbitrary number in meters representing the space
/// between the spread objects
const PADDING: f64 = 1.0;

fn width_and_height_of_area(area: Aabb) -> (f64, f64) {
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

impl ObjectBehavior<AdditionalObjectDescription> for StochasticSpreading {
    fn step(
        &mut self,
        world_interactor: Box<dyn WorldInteractor<AdditionalObjectDescription> + '_>,
    ) -> Option<Action<AdditionalObjectDescription>> {
        if self.should_spread() {
            self.spread(&*world_interactor)
        } else {
            None
        }
    }
}

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
    use myelin_object_data::Kind;
    use myelin_random::RandomMock;

    const SPREADING_CHANGE: f64 = 1.0 / (60.0 * 30.0);
    const EXPECTED_PADDING: f64 = 1.0;

    #[test]
    fn does_nothing_when_chance_is_not_hit() {
        let mut random = RandomMock::new();
        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(false);
        let mut object = StochasticSpreading::new(SPREADING_CHANGE, box random);
        let action = object.step(box WorldInteractorMock::new());
        assert!(action.is_none());
    }

    #[test]
    fn spreads_when_chance_is_hit() {
        let object_behavior = ObjectBehaviorMock::new();
        let mut random = RandomMock::new();
        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(0);
        let mut object = StochasticSpreading::new(SPREADING_CHANGE, box random);
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((34.0, 34.0), (44.0, 44.0)).unwrap())
            })
            .returns(Vec::new());
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((23.0, 23.0), (55.0, 55.0)).unwrap())
            })
            .returns(Vec::new());
        world_interactor.expect_own_object().returns(Object {
            id: 0,
            description: object_description_at_location(50.0, 50.0),
            behavior: &object_behavior,
        });

        let action = object.step(box world_interactor);
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
        let mut random = RandomMock::new();
        let object_behavior = ObjectBehaviorMock::new();

        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(0);

        let mut object = StochasticSpreading::new(SPREADING_CHANGE, box random);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((34.0, 34.0), (44.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 0,
                description: object_description_at_location(39.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap())
            })
            .returns(vec![Object {
                id: 3,
                description: object_description_at_location(61.0, 50.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 56.0), (66.0, 66.0)).unwrap())
            })
            .returns(vec![Object {
                id: 4,
                description: object_description_at_location(61.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 56.0), (55.0, 66.0)).unwrap())
            })
            .returns(vec![Object {
                id: 5,
                description: object_description_at_location(50.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((34.0, 56.0), (44.0, 66.0)).unwrap())
            })
            .returns(vec![Object {
                id: 6,
                description: object_description_at_location(39.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((34.0, 45.0), (44.0, 55.0)).unwrap())
            })
            .returns(vec![Object {
                id: 7,
                description: object_description_at_location(39.0, 50.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor.expect_own_object().returns(Object {
            id: 0,
            description: object_description_at_location(50.0, 50.0),
            behavior: &object_behavior,
        });

        let action = object.step(box world_interactor);
        assert!(action.is_none());
    }

    #[test]
    fn spreads_on_first_available_space_clockwise() {
        let mut random = RandomMock::new();
        let object_behavior = ObjectBehaviorMock::new();

        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(0);

        let mut object = StochasticSpreading::new(SPREADING_CHANGE, box random);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((34.0, 34.0), (44.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 0,
                description: object_description_at_location(39.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap())
            })
            .returns(Vec::new());
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap())
            })
            .returns(Vec::new());
        world_interactor.expect_own_object().returns(Object {
            id: 0,
            description: object_description_at_location(50.0, 50.0),
            behavior: &object_behavior,
        });

        let action = object.step(box world_interactor);
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
        let mut random = RandomMock::new();
        let object_behavior = ObjectBehaviorMock::new();

        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(1);
        let mut object = StochasticSpreading::new(SPREADING_CHANGE, box random);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap())
            })
            .returns(vec![Object {
                id: 3,
                description: object_description_at_location(61.0, 50.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 56.0), (66.0, 66.0)).unwrap())
            })
            .returns(vec![Object {
                id: 4,
                description: object_description_at_location(61.0, 61.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 56.0), (55.0, 66.0)).unwrap())
            })
            .returns(Vec::new());

        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((34.0, 45.0), (66.0, 77.0)).unwrap())
            })
            .returns(Vec::new());
        world_interactor.expect_own_object().returns(Object {
            id: 0,
            description: object_description_at_location(50.0, 50.0),
            behavior: &object_behavior,
        });

        let action = object.step(box world_interactor);
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
        let mut random = RandomMock::new();
        let object_behavior = ObjectBehaviorMock::new();

        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(1);
        let mut object = StochasticSpreading::new(SPREADING_CHANGE, box random);

        let mock_behavior = mock_behavior();
        let mut world_interactor = WorldInteractorMock::new();
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (55.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(50.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 2,
                description: object_description_at_location(60.0, 39.0),
                behavior: mock_behavior.as_ref(),
            }]);
        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap())
            })
            .returns(Vec::new());

        world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap())
            })
            .returns(Vec::new());
        world_interactor.expect_own_object().returns(Object {
            id: 0,
            description: object_description_at_location(50.0, 50.0),
            behavior: &object_behavior,
        });

        let action = object.step(box world_interactor);
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
        let object_behavior = ObjectBehaviorMock::new();

        let mut first_random = RandomMock::new();
        first_random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        first_random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(3);
        let mut first_object = StochasticSpreading::new(SPREADING_CHANGE, box first_random);
        let first_description = object_description_at_location(50.0, 50.0);

        let mut first_world_interactor = WorldInteractorMock::new();
        first_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap())
            })
            .returns(Vec::new());

        first_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap())
            })
            .returns(Vec::new());
        first_world_interactor.expect_own_object().returns(Object {
            id: 1,
            description: object_description_at_location(50.0, 50.0),
            behavior: &object_behavior,
        });

        let first_action = first_object.step(box first_world_interactor);
        match first_action {
            Some(Action::Spawn(object_description, _)) => {
                let expected_object_description =
                    object_description_at_location(60.0 + EXPECTED_PADDING, 50.0);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
        }

        let first_object = first_object;

        let mut second_random = RandomMock::new();
        second_random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(SPREADING_CHANGE))
            .returns(true);
        second_random
            .expect_i32_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(8))
            .returns(7);
        let mut second_object = StochasticSpreading::new(SPREADING_CHANGE, box second_random);

        let mut second_world_interactor = WorldInteractorMock::new();

        second_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 34.0), (66.0, 44.0)).unwrap())
            })
            .returns(vec![Object {
                id: 1,
                description: object_description_at_location(
                    60.0 - EXPECTED_PADDING,
                    40.0 - EXPECTED_PADDING,
                ),
                behavior: mock_behavior.as_ref(),
            }]);
        second_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 45.0), (66.0, 55.0)).unwrap())
            })
            .returns(Vec::new());
        second_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((67.0, 34.0), (77.0, 44.0)).unwrap())
            })
            .returns(Vec::new());

        let other_spreaders = vec![Object {
            id: 0,
            description: first_description,
            behavior: &first_object,
        }];
        second_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((56.0, 23.0), (88.0, 55.0)).unwrap())
            })
            .returns(other_spreaders.clone());
        second_world_interactor
            .expect_find_objects_in_area(|arg| {
                arg.partial_eq(Aabb::try_new((45.0, 34.0), (77.0, 66.0)).unwrap())
            })
            .returns(other_spreaders);
        second_world_interactor.expect_own_object().returns(Object {
            id: 2,
            description: object_description_at_location(72.0, 50.0),
            behavior: &object_behavior,
        });

        let second_action = second_object.step(box second_world_interactor);
        match second_action {
            Some(Action::Spawn(object_description, _)) => {
                // Expected order of operations for the second behavior:
                // 1. Try to spread left, which fails, because the first behavior already spreads there
                // 2. Try to spread to the upper left, which is already occupied
                // 3. Spread to the top
                let expected_object_description =
                    object_description_at_location(72.0, 40.0 - EXPECTED_PADDING);
                assert_eq!(expected_object_description, object_description);
            }
            action => panic!("Expected Action::Spawn, got {:#?}", action),
        }
    }

    #[test]
    fn can_be_downcast_from_trait() {
        let object_behavior: Box<dyn ObjectBehavior<AdditionalObjectDescription>> =
            box StochasticSpreading::new(SPREADING_CHANGE, box RandomMock::new());
        let object_behavior_as_any = object_behavior.as_any();
        let _downcast_behavior: &StochasticSpreading =
            object_behavior_as_any.downcast_ref().unwrap();
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
            .associated_data(AdditionalObjectDescription {
                name: None,
                kind: Kind::Plant,
                height: 0.0,
            })
            .build()
            .unwrap()
    }

    fn mock_behavior() -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
        box ObjectBehaviorMock::new()
    }
}
