use crate::prelude::*;
use std::fmt::Debug;

#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;

/// Trait used by [`WorldInteractor`].
/// Implementors of this trait provide the actual code used for the performed actions
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait Interactable: Debug {
    /// Returns read-only descriptions for all objects either completely
    /// contained or intersecting with the given area.
    fn objects_in_area(&self, area: Aabb) -> Snapshot;
}
