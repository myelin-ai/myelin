use crate::collision_detector::CollisionIter;
use crate::object_container::ObjectContainer;

pub trait CollisionHandler {
    fn handle_collisions<'a>(
        &self,
        collisions: Box<CollisionIter<'a>>,
        container: &mut dyn ObjectContainer,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
