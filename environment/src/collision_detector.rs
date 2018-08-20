use crate::collidable_container::CollidableContainer;
use crate::properties::Collidable;
use std::fmt::Debug;

pub trait CollisionDetector: Debug {
    fn are_colliding(&self, first: &dyn Collidable, second: &dyn Collidable) -> bool;
}

#[derive(Debug)]
pub struct Collision<'a> {
    pub first: &'a dyn Collidable,
    pub second: &'a dyn Collidable,
}

pub type CollisionIter<'a> = dyn Iterator<Item = Collision<'a>>;

pub trait CollisionGatherer {
    fn gather_collisions<'a>(
        &self,
        container: &'a dyn CollidableContainer,
    ) -> Box<CollisionIter<'a>>;
}
