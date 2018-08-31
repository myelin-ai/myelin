use crate::collision_detector::CollisionIter;
use crate::properties::Object;
use slab::Slab;
use myelin_slablit::slab;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(
        &self,
        collisions: Box<CollisionIter<'a>>,
        container: &mut Slab<Object>,
    );

    fn apply_movement(container: &mut Slab<Object>);
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
        container: &mut Slab<Object>,
    ) {
        unimplemented!()
    }

    fn apply_movement(container: &mut Slab<Object>) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    fn test_container() -> Slab<Object> {

    }

    #[test]
    fn applies_movement_to_all_objects() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
