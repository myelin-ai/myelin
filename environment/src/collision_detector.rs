use crate::properties::{Location, Object, Rectangle};
use std::fmt::Debug;

pub trait CollisionDetector: Debug {
    fn are_colliding(&self, first: &Object, second: &Object) -> bool;
}

#[derive(Debug)]
pub struct Collision<'a> {
    pub first: &'a Object,
    pub second: &'a Object,
}

#[allow(missing_debug_implementations)]
pub struct CollisionIter<'a>(pub(crate) Box<dyn Iterator<Item = Collision<'a>>>);

pub fn gather_collisions<'a>(
    _container: &'a [Object],
    _potential_collision_gatherer: &'a dyn PotentialCollisionGatherer,
    _collision_detector: &'a dyn CollisionDetector,
) -> Box<CollisionIter<'a>> {
    unimplemented!()
}

///
/// Gathers potential collisions that must be checked by the [`CollisionGatherer`].
///
/// [`CollisionGatherer`]: ./trait.CollisionGatherer.html
///
pub trait PotentialCollisionGatherer {
    fn possible_collisions<'a>(&self, container: &'a [Object]) -> Box<CollisionIter<'a>>;
}
