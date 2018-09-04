use super::CollisionHandler;
use crate::object::Object;

#[derive(Debug, Default)]
pub struct CollisionHandlerImpl;

impl CollisionHandlerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for CollisionHandlerImpl {
    fn handle_collisions<'a>(&self, object: &Object, collisions: &[&Object]) -> Object {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{Kind, Location, MovementVector, Object, Rectangle};

    fn any_object() -> Object {
        Object {
            rectangle: Rectangle {
                width: 3,
                length: 4,
            },
            location: Location { x: 17, y: 23 },
            movement: MovementVector { x: -4, y: -3 },
            kind: Kind::Undefined,
        }
    }

    #[test]
    fn ignores_empty_iterator() {
        let collision_handler = CollisionHandlerImpl::new();
        let expected_object = any_object();
        let actual_object = collision_handler.handle_collisions(&expected_object, &[]);
        assert_eq!(expected_object, actual_object);
    }

    #[test]
    fn ignores_incorrect_collisions() {
        let container = vec![
            Object {
                rectangle: Rectangle {
                    width: 3,
                    length: 4,
                },
                location: Location { x: 174, y: 213 },
                movement: MovementVector { x: -4, y: -3 },
                kind: Kind::Undefined,
            },
            Object {
                rectangle: Rectangle {
                    width: 4,
                    length: 4,
                },
                location: Location { x: 42, y: 11 },
                movement: MovementVector { x: 0, y: -3 },
                kind: Kind::Undefined,
            },
        ];
        let collisions = vec![&container[0], &container[1]];
        let expected_object = any_object();

        let collision_handler = CollisionHandlerImpl::new();
        let actual_object = collision_handler.handle_collisions(&expected_object, &collisions);

        assert_eq!(expected_object, actual_object)
    }

    fn generate_test_for_object_with_collisions<'a, F>(
        object: &'a Object,
        collisions: &'a [&'a Object],
    ) -> impl FnOnce(F) + 'a
    where
        F: FnOnce(&'a Object) -> Object,
    {
        move |original_to_expected_fn: F| {
            let expected_object = original_to_expected_fn(&object);

            let collision_handler = CollisionHandlerImpl::new();
            let actual_object = collision_handler.handle_collisions(object, collisions);
            assert_eq!(expected_object, actual_object);
        }
    }

    fn collision_when_walking_into_stationary_object_behaves_as_expected<F>(
        moving: Kind,
        stationary: Kind,
        original_to_expected_fn: F,
    ) where
        F: FnOnce(&Object) -> Object,
    {
        let object = Object {
            rectangle: Rectangle {
                width: 3,
                length: 4,
            },
            location: Location { x: 5, y: 5 },
            movement: MovementVector { x: 0, y: 5 },
            kind: moving,
        };
        let collision = Object {
            rectangle: Rectangle {
                width: 2,
                length: 2,
            },
            location: Location { x: 5, y: 10 },
            movement: MovementVector { x: 0, y: 0 },
            kind: stationary,
        };
        generate_test_for_object_with_collisions(&object, &[&collision])(original_to_expected_fn);
    }

    fn collision_when_walking_through_stationary_object_behaves_as_expected<F>(
        moving: Kind,
        stationary: Kind,
        original_to_expected_fn: F,
    ) where
        F: FnOnce(&Object) -> Object,
    {
        let object = Object {
            rectangle: Rectangle {
                width: 3,
                length: 4,
            },
            location: Location { x: 5, y: 5 },
            movement: MovementVector { x: 0, y: 15 },
            kind: moving,
        };
        let collision = Object {
            rectangle: Rectangle {
                width: 2,
                length: 2,
            },
            location: Location { x: 5, y: 10 },
            movement: MovementVector { x: 0, y: 0 },
            kind: stationary,
        };
        generate_test_for_object_with_collisions(&object, &[&collision])(original_to_expected_fn);
    }

    fn ignores_walking_into_stationary_object(moving: Kind, stationary: Kind) {
        collision_when_walking_into_stationary_object_behaves_as_expected(
            moving,
            stationary,
            |original_object| original_object.clone(),
        )
    }

    fn handles_collision_when_walking_into_stationary_object(moving: Kind, stationary: Kind) {
        collision_when_walking_into_stationary_object_behaves_as_expected(
            moving,
            stationary,
            |original_object| {
                let mut expected_object = original_object.clone();
                expected_object.location = Location { x: 5, y: 7 };
                expected_object
            },
        )
    }

    #[test]
    fn handles_collision_when_walking_into_organism() {
        handles_collision_when_walking_into_stationary_object(Kind::Undefined, Kind::Organism);
    }

    #[test]
    fn handles_collision_when_walking_into_wall() {
        handles_collision_when_walking_into_stationary_object(Kind::Undefined, Kind::Wall);
    }

    #[test]
    fn handles_collision_when_walking_into_plant() {
        ignores_walking_into_stationary_object(Kind::Undefined, Kind::Plant);
    }

    #[test]
    fn handles_collision_when_walking_into_water() {
        handles_collision_when_walking_into_stationary_object(Kind::Undefined, Kind::Water);
    }

    fn ignores_walking_through_stationary_object(moving: Kind, stationary: Kind) {
        collision_when_walking_through_stationary_object_behaves_as_expected(
            moving,
            stationary,
            |original_object| original_object.clone(),
        );
    }

    fn handles_collision_when_walking_through_stationary_object(moving: Kind, stationary: Kind) {
        collision_when_walking_through_stationary_object_behaves_as_expected(
            moving,
            stationary,
            |original_object| {
                let mut expected_object = original_object.clone();
                expected_object.location = Location { x: 5, y: 7 };
                expected_object
            },
        );
    }

    #[test]
    fn handles_collision_when_walking_through_organism() {
        ignores_walking_through_stationary_object(Kind::Undefined, Kind::Organism);
    }

    #[test]
    fn handles_collision_when_walking_through_wall() {
        handles_collision_when_walking_through_stationary_object(Kind::Undefined, Kind::Wall);
    }

    #[test]
    fn handles_collision_when_walking_through_plant() {
        ignores_walking_through_stationary_object(Kind::Undefined, Kind::Plant);
    }

    #[test]
    fn handles_collision_when_walking_through_water() {
        handles_collision_when_walking_through_stationary_object(Kind::Undefined, Kind::Water);
    }
}
