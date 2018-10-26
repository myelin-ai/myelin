use alga::general::Real;
use crate::object::Kind;
use ncollide2d::broad_phase::BroadPhasePairFilter;
use ncollide2d::world::CollisionObject;
use nphysics2d::object::ColliderData;
use std::fmt::Debug;

pub trait CollisionFilter<N>: BroadPhasePairFilter<N, ColliderData<N>> + Debug
where
    N: Real,
{
}

#[derive(Debug)]
pub struct IgnoringCollisionFilter {
    ignored_kind: Kind,
}

impl IgnoringCollisionFilter {
    pub fn new(ignored_kind: Kind) -> Self {
        Self { ignored_kind }
    }
}

impl<N> CollisionFilter<N> for IgnoringCollisionFilter where N: Real {}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilter
where
    N: Real,
{
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<N, ColliderData<N>>,
        b2: &CollisionObject<N, ColliderData<N>>,
    ) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct CollisionFilterWrapper<N> {
    pub(crate) collision_filter: Box<dyn CollisionFilter<N>>,
}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for CollisionFilterWrapper<N>
where
    N: Real,
{
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<N, ColliderData<N>>,
        b2: &CollisionObject<N, ColliderData<N>>,
    ) -> bool {
        self.collision_filter.is_pair_valid(b1, b2)
    }
}
