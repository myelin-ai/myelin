use super::CollisionHandler;
use crate::physics_handler::CollisionIterMut;

#[derive(Debug, Default)]
pub struct CollisionHandlerImpl;

impl CollisionHandlerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for CollisionHandlerImpl {
    fn handle_collisions<'a>(&self, _collisions: CollisionIterMut<'a>) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics_handler::CollisionIterMut;
    use crate::properties::{Kind, Location, MovementVector, Object, Rectangle};
    use crate::util::collision_mut_from_container_at;

    #[test]
    fn ignores_empty_iterator() {
        let iter = CollisionIterMut(Box::new(vec![].into_iter()));
        let collision_handler = CollisionHandlerImpl::new();
        collision_handler.handle_collisions(iter);
    }

    #[test]
    fn ignores_incorrect_collisions() {
        let original_container = vec![
            Object {
                rectangle: Rectangle {
                    width: 3,
                    length: 4,
                },
                location: Location { x: 17, y: 23 },
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
        let mut container = original_container.clone();
        let collisions = vec![collision_mut_from_container_at(&mut container, 0, 1)];
        let iter = CollisionIterMut(Box::new(collisions.into_iter()));
        let collision_handler = CollisionHandlerImpl::new();
        collision_handler.handle_collisions(iter);

        original_container
            .into_iter()
            .zip(container)
            .for_each(|(expected, actual)| assert_eq!(expected, actual));
    }

    #[test]
    fn handles_collision_with_stationary_object() {
        let mut original_container = vec![
            Object {
                rectangle: Rectangle {
                    width: 3,
                    length: 4,
                },
                location: Location { x: 5, y: 5 },
                movement: MovementVector { x: 0, y: 5 },
                kind: Kind::Undefined,
            },
            Object {
                rectangle: Rectangle {
                    width: 2,
                    length: 2,
                },
                location: Location { x: 5, y: 10 },
                movement: MovementVector { x: 0, y: 0 },
                kind: Kind::Undefined,
            },
        ];

        let mut expected_container = original_container.clone();
        expected_container[0].location = Location { x: 5, y: 7 };

        let collisions = vec![collision_mut_from_container_at(&mut original_container, 0, 1)];

        let collision_handler = CollisionHandlerImpl::new();
        collision_handler.handle_collisions(CollisionIterMut(Box::new(collisions.into_iter())));

        original_container
            .into_iter()
            .zip(expected_container)
            .for_each(|(expected, actual)| assert_eq!(expected, actual));
    }

    #[test]
    fn handles_collision_when_walking_through_stationary_object() {
        let mut original_container = vec![
            Object {
                rectangle: Rectangle {
                    width: 3,
                    length: 4,
                },
                location: Location { x: 5, y: 5 },
                movement: MovementVector { x: 0, y: 15 },
                kind: Kind::Undefined,
            },
            Object {
                rectangle: Rectangle {
                    width: 2,
                    length: 2,
                },
                location: Location { x: 5, y: 10 },
                movement: MovementVector { x: 0, y: 0 },
                kind: Kind::Undefined,
            },
        ];

        let mut expected_container = original_container.clone();
        expected_container[0].location = Location { x: 5, y: 7 };

        let collisions = vec![collision_mut_from_container_at(&mut original_container, 0, 1)];

        let collision_handler = CollisionHandlerImpl::new();
        collision_handler.handle_collisions(CollisionIterMut(Box::new(collisions.into_iter())));

        original_container
            .into_iter()
            .zip(expected_container)
            .for_each(|(expected, actual)| assert_eq!(expected, actual));
    }
}
