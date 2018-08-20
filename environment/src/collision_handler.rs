use crate::collidable_container::CollidableContainer;

pub trait CollisionHandler {
    fn resolve_collisions(&mut self, container: &mut dyn CollidableContainer);
}

pub struct QuadTree {}

impl CollisionHandler for QuadTree {
    fn resolve_collisions(&mut self, _container: &mut dyn CollidableContainer) {
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
