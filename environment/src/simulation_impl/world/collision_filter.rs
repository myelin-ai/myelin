use ncollide2d::broad_phase::BroadPhasePairFilter;
use ncollide2d::world::CollisionObject;
use nphysics2d::object::ColliderData;
use std::fmt::Debug;

pub trait CollisionFilter: BroadPhasePairFilter<f64, ColliderData<f64>> + Debug {}

#[derive(Debug)]
pub struct CollisionFilterImpl {}

#[derive(Debug)]
pub struct CollisionFilterWrapper {
    pub(crate) collision_filter: Box<dyn CollisionFilter>,
}

impl BroadPhasePairFilter<f64, ColliderData<f64>> for CollisionFilterWrapper {
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<f64, ColliderData<f64>>,
        b2: &CollisionObject<f64, ColliderData<f64>>,
    ) -> bool {
        self.collision_filter.is_pair_valid(b1, b2)
    }
}

impl CollisionFilter for CollisionFilterImpl {}

impl BroadPhasePairFilter<f64, ColliderData<f64>> for CollisionFilterImpl {
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<f64, ColliderData<f64>>,
        b2: &CollisionObject<f64, ColliderData<f64>>,
    ) -> bool {
        b1.handle().uid() % 2 == b2.handle().uid() % 2
    }
}
