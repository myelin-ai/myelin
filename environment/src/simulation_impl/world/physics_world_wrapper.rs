use nphysics2d::world::World as PhysicsWorld;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

/// Wrapper struct around a [`PhysicsWorld`] providing a [`Debug`] implementation.
pub(super) struct PhysicsWorldWrapper(pub(super) PhysicsWorld<f64>);

impl Debug for PhysicsWorldWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(PhysicsWorldWrapper)).finish()
    }
}

impl Deref for PhysicsWorldWrapper {
    type Target = PhysicsWorld<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PhysicsWorldWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
