//! Prelude reexporting essential types
//! When using the myelin engine, you can always start your code with
//! ```
//! use myelin_environment::prelude::*;
//! ```

pub use crate::object::*;
pub use crate::object_builder::*;
pub use crate::world_interactor::WorldInteractor;
#[cfg(any(test, feature = "use-mocks"))]
pub use crate::world_interactor::WorldInteractorMock;
pub use crate::*;
