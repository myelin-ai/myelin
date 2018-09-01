use super::CollisionHandler;
use crate::collision_detector::CollisionIter;
use crate::properties::Object;

#[derive(Debug)]
pub struct CollisionHandlerImpl;

impl CollisionHandlerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for CollisionHandlerImpl {
    fn handle_collisions<'a>(
        &self,
        _collisions: Box<CollisionIter<'a>>,
        _container: &mut [Object],
    ) {
        unimplemented!()
    }
}
