//! Prelude reexporting essential types
//! When using the myelin engine, you can always start your code with
//! ```
//! use myelin_engine::prelude::*;
//! ```

pub use crate::object::*;
pub use crate::object_builder::*;
pub use crate::simulation::{Id, Simulation, Snapshot};
pub use crate::world_interactor::WorldInteractor;
pub use myelin_geometry::*;

#[cfg(any(test, feature = "use-mocks"))]
pub use self::mocks::*;

#[cfg(any(test, feature = "use-mocks"))]
mod mocks {
    pub use crate::simulation::SimulationMock;
    pub use crate::world_interactor::WorldInteractorMock;
}
