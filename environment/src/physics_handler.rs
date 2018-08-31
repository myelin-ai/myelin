use crate::collision_detector::CollisionIter;
use crate::properties::Object;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(
        &self,
        collisions: Box<CollisionIter<'a>>,
        container: &mut [Object],
    );

    fn apply_movement(&self, container: &mut [Object]);
}


pub struct PhysicsHandlerImpl;

impl PhysicsHandlerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl PhysicsHandler for PhysicsHandlerImpl {
    fn handle_collisions<'a>(
        &self,
        _collisions: Box<CollisionIter<'a>>,
        _container: &mut [Object],
    ) {
        unimplemented!()
    }

    fn apply_movement(&self, _container: &mut [Object]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::*;

    #[test]
    fn applies_movement() {
        let original_object = Object {
            rectangle: Rectangle {
                length: 5,
                width: 4,
            },
            location: Location {
                x: 10,
                y: 20
            },
            movement: MovementVector {
                x: 6,
                y: 7
            },
            kind: Kind::Undefined
        };
        let mut container = vec![
            original_object.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());

        let expected_object = Object {
            location: Location {
                x: 16,
                y: 27
            }, ..original_object
        };
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
    }

    #[test]
    fn applies_movement_to_all_objects() {
        let original_object_one = Object {
            rectangle: Rectangle {
                length: 5,
                width: 4,
            },
            location: Location {
                x: 10,
                y: 20
            },
            movement: MovementVector {
                x: 6,
                y: 7
            },
            kind: Kind::Undefined
        };

        let original_object_two = Object {
            rectangle: Rectangle {
                length: 6,
                width: 2,
            },
            location: Location {
                x: 120,
                y: 70
            },
            movement: MovementVector {
                x: -52,
                y: 2
            },
            kind: Kind::Undefined
        };
        let mut container = vec![
            original_object_one.clone(),
            original_object_two.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert_eq!(2, container.len());

        let expected_object_one = Object {
            location: Location {
                x: 16,
                y: 27
            }, ..original_object_one
        };
        let actual_object_one = &container[0];
        assert_eq!(expected_object_one, *actual_object_one);

    
        let expected_object_two = Object {
            location: Location {
                x: 68,
                y: 72
            }, ..original_object_two
        };
        let actual_object_two = &container[1];
        assert_eq!(expected_object_two, *actual_object_two);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_movement_panics() {
        let original_object = Object {
            rectangle: Rectangle {
                length: 5,
                width: 4,
            },
            location: Location {
                x: 10,
                y: 20
            },
            movement: MovementVector {
                x: -30,
                y: -20
            },
            kind: Kind::Undefined
        };
        let mut container = vec![
            original_object.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);
    }

    #[test]
    #[should_panic]
    fn panics_when_rectangle_clips_into_negative_position() {
        let original_object = Object {
            rectangle: Rectangle {
                length: 6,
                width: 4,
            },
            location: Location {
                x: 3,
                y: 2
            },
            movement: MovementVector {
                x: 0,
                y: -1
            },
            kind: Kind::Undefined
        };
        let mut container = vec![
            original_object.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);
    }

    #[test]
    fn allows_object_on_exact_origin() {
        let original_object = Object {
            rectangle: Rectangle {
                length: 6,
                width: 4,
            },
            location: Location {
                x: 4,
                y: 2
            },
            movement: MovementVector {
                x: -1,
                y: 0,
            },
            kind: Kind::Undefined
        };
        let mut container = vec![
            original_object.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());

        let expected_object = Object {
            location: Location {
                x: 3,
                y: 2
            }, ..original_object
        };
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
    }

    #[test]
    fn ignores_zeroed_movement_vector() {
        let expected_object = Object {
            rectangle: Rectangle {
                length: 6,
                width: 4,
            },
            location: Location {
                x: 40,
                y: 22
            },
            movement: MovementVector {
                x: 0,
                y: 0,
            },
            kind: Kind::Undefined
        };
        let mut container = vec![
            expected_object.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
    }


    #[test]
    fn ignores_empty_container() {
        let mut container = vec![];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert!(container.is_empty());
    }
    
}
