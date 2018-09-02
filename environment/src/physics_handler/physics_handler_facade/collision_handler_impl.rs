use super::CollisionHandler;
use crate::collision_detector::CollisionIter;

#[derive(Debug, Default)]
pub struct CollisionHandlerImpl;

impl CollisionHandlerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for CollisionHandlerImpl {
    fn handle_collisions<'a>(&self, _collisions: CollisionIter<'a>) {
        unimplemented!()
    }
}
