use crate::collision_detector::CollisionIter;
use crate::properties::Object;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(&self, collisions: Box<CollisionIter<'a>>, container: &mut [Object]);
    fn apply_movement(&self, container: &mut [Object]);
}
