use crate::collidable_container::CollidableContainer;
use crate::collision_detector::CollisionIter;

pub trait CollisionHandler {
    fn handle_collisions<'a>(
        &self,
        collisions: Box<CollisionIter<'a>>,
        container: &mut dyn CollidableContainer,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
