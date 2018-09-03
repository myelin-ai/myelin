use crate::properties::Object;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(&self, object: &Object, collisions: &[&Object]) -> Object;
    fn apply_movement(&self, object: &Object) -> Object;
}
