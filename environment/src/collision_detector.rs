use crate::properties::{Object, ObjectContainer};
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
    fn gather_collisions<'a>(&self, container: &'a ObjectContainer) -> Box<CollisionIter<'a>>;
}

///
/// Gathers potential collisions that must be checked by the [`CollisionGatherer`].
///
/// [`CollisionGatherer`]: ./trait.CollisionGatherer.html
///
pub trait PotentialCollisionGatherer {
    fn possible_collisions<'a>(&self, container: &'a ObjectContainer) -> Box<CollisionIter<'a>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::properties::{Kind, Location, MovementVector, Rectangle};
    use slab::Slab;

    #[test]
    fn test_quadtree_splits_on_max_capacity() {
        const WORLD_SIZE: u32 = 100;
        const FIRST_SUBBUCKET_SIZE: u32 = 50;

        let expected = Bucket {
            rectangle: Rectangle::new(WORLD_SIZE, WORLD_SIZE),
            location: Location::new(0, 0),
            objects: vec![],
            buckets: vec![
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(0, 0),
                    object: vec![Object::new(
                        Location::new(10, 10),
                        Rectangle::new(14, 14),
                        MovementVector::default(),
                        Kind::Organism,
                    )],
                    buckets: vec![],
                },
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(FIRST_SUBBUCKET_SIZE, 0),
                    object: vec![Object::new(
                        Location::new(80, 30),
                        Rectangle::new(12, 12),
                        MovementVector::default(),
                        Kind::Organism,
                    )],
                    buckets: vec![],
                },
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(0, FIRST_SUBBUCKET_SIZE),
                    object: vec![Object::new(
                        Location::new(10, 60),
                        Rectangle::new(14, 14),
                        MovementVector::default(),
                        Kind::Organism,
                    )],
                    buckets: vec![],
                },
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    object: vec![Object::new(
                        Location::new(70, 70),
                        Rectangle::new(14, 14),
                        MovementVector::default(),
                        Kind::Organism,
                    )],
                    buckets: vec![],
                },
            ],
        };

        let container = {
            let mut container = Slab::new();

            container.insert(Object::new(
                Location::new(10, 10),
                Rectangle::new(14, 14),
                MovementVector::default(),
                Kind::Organism,
            ));

            container.insert(Object::new(
                Location::new(80, 30),
                Rectangle::new(12, 12),
                MovementVector::default(),
                Kind::Organism,
            ));

            container.insert(Object::new(
                Location::new(10, 60),
                Rectangle::new(14, 14),
                MovementVector::default(),
                Kind::Organism,
            ));

            container.insert(Object::new(
                Location::new(70, 70),
                Rectangle::new(14, 14),
                MovementVector::default(),
                Kind::Organism,
            ));

            container
        };

        let quad_tree = QuadTreeBuilder::default().max_capacity(4).build(container);

        assert_eq!(expected, quad_tree.root_bucket);
    }
}
