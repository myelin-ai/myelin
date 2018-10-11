use myelin_environment::object::*;

/// This step's object deltas
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ViewModelDelta {
    /// Deltas of the objects in the world
    objects: Vec<ObjectDescriptionDelta>,
    /// Ids of objects that have been removed from the world
    deleted_objects: Vec<Id>,
}

/// The delta of a [`ObjectDescription`].
///
/// [`ObjectDescription`]: ../../environment/object/struct.ObjectDescription.html
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ObjectDescriptionDelta {
    /// Unique identifier
    pub id: Id,

    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Option<Polygon>,

    /// The current position of the object
    pub position: Option<Position>,

    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Option<Mobility>,

    /// The object's kind
    pub kind: Option<Kind>,

    /// The object's sensor
    pub sensor: Option<Option<Sensor>>,
}

/// Unique identifier of an [`ObjectDescriptionDelta`]
///
/// [`ObjectDescriptionDelta`]: ./struct.ObjectDescriptionDelta.html
pub type Id = usize;
