use super::Snapshot;
use myelin_environment::Id;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub(crate) enum DeltaApplierError {
    AlreadyExistingObjectCreated(Id),
    NonExistingObjectUpdated(Id),
    NonExistingObjectDeleted(Id),
}

impl Display for DeltaApplierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeltaApplierError::AlreadyExistingObjectCreated(id) => write!(
                f,
                "An object with id {} exist in snapshot, but was created in delta",
                id
            ),
            DeltaApplierError::NonExistingObjectUpdated(id) => write!(
                f,
                "An object with id {} does not exist in snapshot, but was updated in delta",
                id
            ),
            DeltaApplierError::NonExistingObjectDeleted(id) => write!(
                f,
                "An object with id {} does not exist in snapshot, but was deleted in delta",
                id
            ),
        }
    }
}

impl Error for DeltaApplierError {}

pub(crate) trait DeltaApplier: Debug {
    fn apply_delta(
        &self,
        snapshot: &mut Snapshot,
        delta: ViewModelDelta,
    ) -> Result<(), DeltaApplierError>;
}
