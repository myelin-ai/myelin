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
    #[test]
    fn ignores_empty_iterator() {}
}
