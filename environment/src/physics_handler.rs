use crate::collision_detector::CollisionIter;
use crate::properties::Object;
use slab::Slab;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(
        &self,
        collisions: Box<CollisionIter<'a>>,
        container: &mut Slab<Object>,
    );

    fn apply_movement(container: &mut Slab<Object>);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
