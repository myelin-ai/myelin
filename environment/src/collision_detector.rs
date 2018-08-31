use crate::properties::Object;
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

pub fn gather_collisions<'a, 'b>(
    container: &'a [Object],
    potential_collision_gatherer: &'b dyn PotentialCollisionGatherer,
    collision_detector: &'b dyn CollisionDetector,
) -> CollisionIter<'a> {
    let potential_collisions = potential_collision_gatherer
        .possible_collisions(container)
        .0;
    CollisionIter(Box::new(potential_collisions.filter(
        |potential_collision| {
            collision_detector
                .are_colliding(&potential_collision.first, &potential_collision.second)
        },
    )))
}

///
/// Gathers potential collisions that must be checked by the [`CollisionGatherer`].
///
/// [`CollisionGatherer`]: ./trait.CollisionGatherer.html
///
pub trait PotentialCollisionGatherer {
    fn possible_collisions<'a>(&self, container: &'a [Object]) -> CollisionIter<'a>;
}
