use myelin_environment::object::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ViewModelDelta {
    objects: Vec<ObjectDescriptionDelta>,
}

/// The dehaviourless description of an object that has
/// been placed inside a [`Simulation`].
///
/// [`Simulation`]: ../simulation/trait.Simulation.html
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ObjectDescriptionDelta {
    // To do: Add some kind of ID
    // To do: Support object deletion
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
