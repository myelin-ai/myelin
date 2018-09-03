use super::PhysicsHandler;
use crate::object::Object;

use std::fmt::Debug;
//pub mod collision_handler_impl;
//pub mod movement_applier_impl;

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
    use std::fmt;

    struct MovementApplierMock {
        apply_movement_fn: Option<Box<dyn Fn(&Object) -> Object>>,
    }
    impl fmt::Debug for MovementApplierMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.apply_movement_fn.is_some() {
                write!(f, "apply_movement_fn has been set")
            } else {
                write!(f, "apply_movement_fn has not been set")
            }
        }
    }

    impl MovementApplierMock {
        fn new() -> Self {
            Self {
                apply_movement_fn: None,
            }
        }
        fn expect_apply_movement(&mut self, apply_movement_fn: Box<dyn Fn(&Object) -> Object>) {
            self.apply_movement_fn = Some(apply_movement_fn);
        }
    }
    impl MovementApplier for MovementApplierMock {
        fn apply_movement(&self, object: &Object) -> Object {
            if let Some(apply_movement_fn) = &self.apply_movement_fn {
                apply_movement_fn(object)
            } else {
                panic!("apply_movement has been called unexpectedly")
            }
        }
    }

    struct CollisionHandlerMock {
        handle_collisions_fn: Option<Box<dyn Fn(&Object, &[&Object]) -> Object>>,
    }
    impl fmt::Debug for CollisionHandlerMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.handle_collisions_fn.is_some() {
                write!(f, "apply_movement_fn has been set")
            } else {
                write!(f, "apply_movement_fn has not been set")
            }
        }
    }

    impl CollisionHandlerMock {
        fn new() -> Self {
            Self {
                handle_collisions_fn: None,
            }
        }

        fn expect_handle_collisions(
            &mut self,
            handle_collisions_fn: Box<dyn Fn(&Object, &[&Object]) -> Object>,
        ) {
            self.handle_collisions_fn = Some(handle_collisions_fn);
        }
    }

    impl CollisionHandler for CollisionHandlerMock {
        fn handle_collisions<'a>(&self, object: &Object, collisions: &[&Object]) -> Object {
            if let Some(handle_collisions_fn) = &self.handle_collisions_fn {
                handle_collisions_fn(object, collisions)
            } else {
                panic!("handle_collisions_fn has been called unexpectedly")
            }
        }
    }

    fn expected_object() -> Object {
        Object {
            rectangle: Rectangle {
                width: 1,
                length: 2,
            },
            location: Location { x: 3, y: 5 },
            movement: MovementVector { x: 1, y: 32 },
            kind: Kind::Undefined,
        }
    }

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
    fn passes_container_to_movement_applier() {
        let mut movement_applier = Box::new(MovementApplierMock::new());
        let expected_object = expected_object();
        let passed_object = any_object();
        {
            let passed_object = passed_object.clone();
            let expected_object = expected_object.clone();
            movement_applier.expect_apply_movement(Box::new(move |object| {
                assert_eq!(passed_object, *object);
                expected_object.clone()
            }));
        }
        let collision_handler = Box::new(CollisionHandlerMock::new());

        let physics_handler_facade = PhysicsHandlerFacade::new(movement_applier, collision_handler);
        let actual_object = physics_handler_facade.apply_movement(&any_object());
        assert_eq!(expected_object, actual_object);
    }

    #[test]
    fn passes_collider_and_container_to_collision_handler() {
        let movement_applier = Box::new(MovementApplierMock::new());
        let mut collision_handler = Box::new(CollisionHandlerMock::new());
        let passed_object = any_object();
        let first_collision = Object {
            rectangle: Rectangle {
                length: 12,
                width: 4,
            },
            ..passed_object.clone()
        };
        let second_collision = Object {
            location: Location { y: 12, x: 4 },
            ..passed_object.clone()
        };
        let collisions = vec![&first_collision, &second_collision];
        let expected_object = expected_object();
        {
            let expected_object = expected_object.clone();
            let passed_object = passed_object.clone();
            let collisions = collisions.clone();
            collision_handler.expect_handle_collisions(Box::new(
                move |actual_object, actual_collisions| {
                    assert_eq!(passed_object, *actual_object);
                    collisions
                        .iter()
                        .zip(actual_collisions)
                        .for_each(|(&expected, &actual)| assert_eq!(*expected, *actual));

                    expected_object.clone()
                },
            ));
        }

        let physics_handler_facade = PhysicsHandlerFacade::new(movement_applier, collision_handler);
        let actual_object = physics_handler_facade.handle_collisions(&passed_object, &collisions);
        assert_eq!(expected_object, actual_object);
    }
}
