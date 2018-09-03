pub mod physics_handler_facade;

use crate::object::Object;

pub trait PhysicsHandler {
    fn handle_collisions(&self, object: &Object, collisions: &[&Object]) -> Object;
    fn apply_movement(&self, object: &Object) -> Object;
}
