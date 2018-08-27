use crate::properties::Object;
use slab::Slab;
use std::fmt::Debug;

pub trait CollisionDetector: Debug {
    fn are_colliding(&self, first: &Object, second: &Object) -> bool;
}

#[derive(Debug)]
pub struct Collision<'a> {
    pub first: &'a Object,
    pub second: &'a Object,
}

pub type CollisionIter<'a> = dyn Iterator<Item = Collision<'a>>;

pub trait CollisionGatherer {
    fn gather_collisions<'a>(&self, container: &'a Slab<Object>) -> Box<CollisionIter<'a>>;
}
