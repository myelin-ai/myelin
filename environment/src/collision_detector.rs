use crate::object_container::ObjectContainer;
use crate::properties::Object;
use std::fmt::Debug;

pub trait CollisionDetector: Debug {
    fn are_colliding(&self, first: &dyn Object, second: &dyn Object) -> bool;
}

#[derive(Debug)]
pub struct Collision<'a> {
    pub first: &'a dyn Object,
    pub second: &'a dyn Object,
}

pub type CollisionIter<'a> = dyn Iterator<Item = Collision<'a>>;

pub trait CollisionGatherer {
    fn gather_collisions<'a>(&self, container: &'a dyn ObjectContainer) -> Box<CollisionIter<'a>>;
}
