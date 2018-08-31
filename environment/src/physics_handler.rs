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
        let originalRectangle = Rectangle {
            length: 5,
            width: 4,
        };
        let originalMovement = MovementVector {
            x: 6,
            y: 7
        };
        let mut container = vec![
            Object {
                location: Location {
                    x: 10,
                    y: 20
                },
                rectangle: originalRectangle.clone(),
                movement: originalMovement.clone(),
                kind: Kind::Organism,
            }
        ];
        let physics_handler = PhysicsHandlerImpl::new();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());
        let object = &container[0];
        let expectedLocation = Location {
            x: 16,
            y: 27,
        };
        assert_eq!(expectedLocation, object.location);
        assert_eq!(originalRectangle, object.rectangle);
        assert_eq!(originalMovement, object.movement);
    }
}
