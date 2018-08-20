use crate::collidable_container::CollidableContainer;
use crate::properties::SpawnEvent;

pub trait CollisionHandler {
    fn resolve_collisions(&self, container: &dyn CollidableContainer) -> Vec<SpawnEvent>;
}

pub struct QuadTree {}

impl CollisionHandler for QuadTree {
    fn resolve_collisions(&self, _container: &dyn CollidableContainer) -> Vec<SpawnEvent> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
