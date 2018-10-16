use myelin_environment::object::*;
use myelin_environment::Id;
use std::collections::HashMap;

/// This step's object deltas
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ViewModelDelta {
    /// Objects that have been added to the world
    pub created_objects: HashMap<Id, ObjectDescription>,
    /// Deltas of the updated objects in the world
    pub updated_objects: HashMap<Id, ObjectDescriptionDelta>,
    /// Ids of objects that have been removed from the world
    pub deleted_objects: Vec<Id>,
}

/// The delta of a [`ObjectDescription`].
///
/// [`ObjectDescription`]: ../../environment/object/struct.ObjectDescription.html
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ObjectDescriptionDelta {
    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Option<Polygon>,

    /// The current location of the object
    pub location: Option<Location>,

    /// The current rotation of the object
    pub rotation: Option<Radians>,

    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Option<Mobility>,

    /// The object's kind
    pub kind: Option<Kind>,

    /// The object's sensor
    pub sensor: Option<Option<Sensor>>,
}
