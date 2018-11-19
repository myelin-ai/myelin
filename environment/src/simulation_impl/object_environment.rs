use crate::object::{Aabb, ObjectEnvironment};
use crate::{Simulation, Snapshot};

/// Default implementation of [`ObjectEnvironment`].
///
/// [`ObjectEnvironment`]: ./../object/trait.ObjectEnvironment.html
#[derive(Debug)]
pub struct ObjectEnvironmentImpl<'a> {
    simulation: &'a dyn Simulation,
}

impl<'a> ObjectEnvironmentImpl<'a> {
    /// Creates a new instance of [`ObjectEnvironmentImpl`].
    ///
    /// [`ObjectEnvironmentImpl`]: ./struct.ObjectEnvironmentImpl.html
    pub fn new(simulation: &'a dyn Simulation) -> Self {
        Self { simulation }
    }
}

impl<'a> ObjectEnvironment for ObjectEnvironmentImpl<'a> {
    fn find_objects_in_area(&self, _area: Aabb) -> Snapshot {
        unimplemented!();
    }
}
