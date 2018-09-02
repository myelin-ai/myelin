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
    use crate::physics_handler::{CollisionIterMut, CollisionMut};
    use crate::properties::{Kind, Location, MovementVector, Object, Rectangle};

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
        let (container_a, container_b) = container.split_at_mut(1);
        {
            let collisions = vec![CollisionMut {
                first: &mut container_a[0],
                second: &mut container_b[0],
            }];
            let iter = CollisionIterMut(Box::new(collisions.into_iter()));
            let collision_handler = CollisionHandlerImpl::new();
            collision_handler.handle_collisions(iter);
        }
        original_container
            .into_iter()
            .zip(container)
            .for_each(|(expected, actual)| assert_eq!(expected, actual));
    }
}
