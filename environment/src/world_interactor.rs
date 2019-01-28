//! Trait and implementation for [`WorldInteractor`]

use crate::Snapshot;
use mockiato::mockable;
use myelin_geometry::Aabb;
use std::fmt::Debug;

mod world_interactor_impl;
pub use self::world_interactor_impl::*;

/// Provides information to an [`ObjectBehavior`] about
/// the world it is placed in.
///
/// [`ObjectBehavior`]: ./trait.ObjectBehavior.html
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait WorldInteractor: Debug {
    /// Scans for objects in the area defined by an [`Aabb`].
    ///
    /// Returns all objects either completely contained or intersecting
    /// with the area.
    ///
    /// [`Aabb`]: ./struct.Aabb.html
    fn find_objects_in_area(&self, area: Aabb) -> Snapshot;
}
