use crate::properties::Object;
use std::fmt::Debug;

pub trait CollisionDetector: Debug {
    fn are_colliding(&self, first: &Object, second: &Object) -> bool;
}

#[derive(Debug, Clone)]
pub struct Collision<'a> {
    pub first: &'a Object,
    pub second: &'a Object,
}

#[allow(missing_debug_implementations)]
pub struct CollisionIter<'a>(pub(crate) Box<dyn Iterator<Item = Collision<'a>> + 'a>);

pub trait CollisionGatherer {
    fn gather_collisions<'a>(&self, container: &'a [Object]) -> CollisionIter<'a>;
}

impl<'a> Iterator for CollisionIter<'a> {
    type Item = Collision<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
