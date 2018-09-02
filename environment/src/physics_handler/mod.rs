use crate::collision_detector::CollisionIter;
use crate::properties::Object;

pub mod physics_handler_facade;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(&self, collisions: CollisionIter<'a>);
    fn apply_movement(&self, container: &mut [Object]);
}
