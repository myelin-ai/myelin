use crate::collidable_container::CollidableContainer;
use crate::properties::Collidable;

pub trait CollisionDetector {
    fn are_colliding(&self, first: &dyn Collidable, second: &dyn Collidable) -> bool;
}

pub struct Collision<'a> {
    pub first: &'a dyn Collidable,
    pub second: &'a dyn Collidable,
}

pub trait CollisionGatherer {
    fn gather_collisions<'a>(
        &self,
        container: &'a dyn CollidableContainer,
    ) -> Box<dyn Iterator<Item = Collision<'a>>>;
}
