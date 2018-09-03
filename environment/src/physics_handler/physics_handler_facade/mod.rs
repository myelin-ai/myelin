use super::PhysicsHandler;
use crate::object::Object;

use std::fmt::Debug;
pub mod collision_handler_impl;
pub mod movement_applier_impl;

#[derive(Debug)]
pub struct PhysicsHandlerFacade {
    movement_applier: Box<dyn MovementApplier>,
    collision_handler: Box<dyn CollisionHandler>,
}

impl PhysicsHandlerFacade {
    pub fn new(
        movement_applier: Box<dyn MovementApplier>,
        collision_handler: Box<dyn CollisionHandler>,
    ) -> Self {
        Self {
            movement_applier,
            collision_handler,
        }
    }
}

impl PhysicsHandler for PhysicsHandlerFacade {
    fn handle_collisions<'a>(&self, object: &Object, collisions: &[&Object]) -> Object {
        self.collision_handler.handle_collisions(object, collisions)
    }
    fn apply_movement(&self, object: &Object) -> Object {
        self.movement_applier.apply_movement(object)
    }
}

pub trait CollisionHandler: Debug {
    fn handle_collisions<'a>(&self, object: &Object, collisions: &[&Object]) -> Object;
}

pub trait MovementApplier: Debug {
    fn apply_movement(&self, object: &Object) -> Object;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{Kind, Location, MovementVector, Object, Rectangle};

    #[derive(Debug)]
    struct MovementApplierMock {
        pub passed_container: Vec<Object>,
    }
    impl MovementApplierMock {
        fn new() -> Self {
            Self {
                passed_container: Vec::new(),
            }
        }
        fn expect_apply_movement(&mut self, container: &[Object]) {
            self.passed_container.extend_from_slice(container);
        }
    }
    impl MovementApplier for MovementApplierMock {
        fn apply_movement(&self, object: &Object) -> Object {
            self.passed_container
                .iter()
                .zip(container)
                .for_each(|(expected, actual)| {
                    assert_eq!(expected, actual);
                });
        }
    }

    #[derive(Debug)]
    struct CollisionHandlerMock {
        pub passed_collisions: Vec<(Object, Object)>,
        pub passed_container: Vec<Object>,
    }
    impl CollisionHandlerMock {
        fn new() -> Self {
            Self {
                passed_collisions: Vec::new(),
                passed_container: Vec::new(),
            }
        }

        fn expect_handle_collisions(
            &mut self,
            collisions: &[(Object, Object)],
            container: &[Object],
        ) {
            self.passed_collisions.extend_from_slice(collisions);
            self.passed_container.extend_from_slice(container);
        }
    }
    impl CollisionHandler for CollisionHandlerMock {
        fn handle_collisions<'a>(&self, object: &Object, collisions: &[&Object]) -> Object {
            let collisions_as_tuple: Vec<_> = collisions
                .0
                .map(|collision| (collision.first.clone(), collision.second.clone()))
                .collect();
            self.passed_collisions
                .iter()
                .zip(collisions_as_tuple)
                .for_each(|(expected, actual)| {
                    assert_eq!(expected.0, actual.0);
                    assert_eq!(expected.1, actual.1);
                });
        }
    }

    fn container() -> Vec<Object> {
        vec![
            Object {
                rectangle: Rectangle {
                    width: 6,
                    length: 5,
                },
                location: Location { x: 10, y: 20 },
                movement: MovementVector { x: 6, y: 7 },
                kind: Kind::Undefined,
            },
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
        ]
    }

    #[test]
    fn passes_container_to_movement_applier() {
        let mut movement_applier = Box::new(MovementApplierMock::new());
        let mut container = container();
        movement_applier.expect_apply_movement(&container);
        let collision_handler = Box::new(CollisionHandlerMock::new());

        let physics_handler_facade = PhysicsHandlerFacade::new(movement_applier, collision_handler);
        physics_handler_facade.apply_movement(&mut container);
    }

    #[test]
    fn passes_collider_and_container_to_collision_handler() {
        let movement_applier = Box::new(MovementApplierMock::new());
        let mut collision_handler = Box::new(CollisionHandlerMock::new());
        let mut container = container();
        let expected_collisions = vec![(container[0].clone(), container[1].clone())];
        collision_handler.expect_handle_collisions(&expected_collisions, &container);

        let physics_handler_facade = PhysicsHandlerFacade::new(movement_applier, collision_handler);
        let collisions = vec![collision_mut_from_container_at(&mut container, 0, 0)];
        physics_handler_facade
            .handle_collisions(CollisionIterMut(Box::new(collisions.into_iter())));
    }
}
