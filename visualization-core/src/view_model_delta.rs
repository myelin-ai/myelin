//! The delta of the world's state, i.e. the properties
//! that changed since the last snapshot

use myelin_engine::prelude::*;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

/// This step's object deltas
pub type ViewModelDelta = HashMap<Id, ObjectDelta>;

/// Describes what happened to an individual object in this
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjectDelta {
    /// The object has been added to the world
    Created(ObjectDescription),
    /// At least one property of the object has changed
    Updated(ObjectDescriptionDelta),
    /// The object has been removed from the world
    Deleted,
}

/// The delta of a [`ObjectDescription`].
///
/// [`ObjectDescription`]: ../../engine/object/struct.ObjectDescription.html
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectDescriptionDelta {
    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Option<Polygon>,

    /// The current location of the object
    pub location: Option<Point>,

    /// The current rotation of the object
    pub rotation: Option<Radians>,

    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Option<Mobility>,

    /// Arbitrary data associated with this object
    pub associated_data: Option<Vec<u8>>,
}
