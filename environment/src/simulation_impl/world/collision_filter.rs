use super::to_body_handle;
use alga::general::Real;
use crate::simulation_impl::BodyHandle;
use ncollide2d::broad_phase::BroadPhasePairFilter;
use ncollide2d::world::CollisionObject;
use nphysics2d::object::ColliderData;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub trait IgnoringCollisionFilter<N>: BroadPhasePairFilter<N, ColliderData<N>> + Debug
where
    N: Real,
{
    fn add_ignored_body_handle(&mut self, body_handle: BodyHandle);
    fn is_body_ignored(&self, body_handle: BodyHandle) -> bool;
    fn remove_ignored_body_handle(&mut self, body_handle: BodyHandle);
}

#[derive(Debug, Default)]
pub struct IgnoringCollisionFilterImpl {
    ignored_body_handles: HashSet<BodyHandle>,
}

impl IgnoringCollisionFilterImpl {}

impl<N> IgnoringCollisionFilter<N> for IgnoringCollisionFilterImpl
where
    N: Real,
{
    fn add_ignored_body_handle(&mut self, body_handle: BodyHandle) {
        self.ignored_body_handles.insert(body_handle);
    }

    fn is_body_ignored(&self, body_handle: BodyHandle) -> bool {
        self.ignored_body_handles.contains(&body_handle)
    }

    fn remove_ignored_body_handle(&mut self, body_handle: BodyHandle) {
        self.ignored_body_handles.remove(&body_handle);
    }
}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilterImpl
where
    N: Real,
{
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<N, ColliderData<N>>,
        b2: &CollisionObject<N, ColliderData<N>>,
    ) -> bool {
        let body_handle1 = to_body_handle(b1.handle());
        let body_handle2 = to_body_handle(b2.handle());

        !(self.ignored_body_handles.contains(&body_handle1)
            || self.ignored_body_handles.contains(&body_handle2))
    }
}

#[derive(Debug)]
pub struct IgnoringCollisionFilterWrapper<N> {
    pub(crate) collision_filter: Arc<RwLock<dyn IgnoringCollisionFilter<N>>>,
}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilterWrapper<N>
where
    N: Real,
{
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<N, ColliderData<N>>,
        b2: &CollisionObject<N, ColliderData<N>>,
    ) -> bool {
        self.collision_filter
            .read()
            .expect("Lock was poisoned")
            .is_pair_valid(b1, b2)
    }
}
