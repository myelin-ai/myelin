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


struct PhysicsHandlerImpl;

impl PhysicsHandlerImpl {
    fn new() -> Self {
        Self {}
    }
}

impl PhysicsHandler for PhysicsHandlerImpl {
    fn handle_collisions<'a>(
        &self,
        collisions: Box<CollisionIter<'a>>,
        container: &mut [Object],
    ) {
        unimplemented!()
    }

    fn apply_movement(&self, container: &mut [Object]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::*;

    #[test]
    fn applies_movement() {
        let originalObject = Object {
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
            originalObject.clone()
        ];

        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());

        let expectedObject = Object {
            location: Location {
                x: 16,
                y: 27
            }, ..originalObject
        };
        let actual_object = &container[0];
        assert_eq!(expectedObject, actual_object);
    }

    
}
