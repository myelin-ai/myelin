//! Objects that can be placed in a world and their components.
//! # Examples
//! The following defines a stationary square terrain:
//! ```
//! use myelin_environment::object::*;
//!
//! let square = ObjectDescription {
//!     shape: Polygon {
//!         vertices: vec![
//!             Vertex { x: -50, y: -50 },
//!             Vertex { x: -50, y: 50 },
//!             Vertex { x: 50, y: 50 },
//!             Vertex { x: 50, y: -50 },
//!         ],
//!     },
//!     position: Position {
//!         rotation: Radians(0.0),
//!         location: Location { x: 100, y: 100 },
//!     },
//!     kind: Kind::Terrain,
//!     mobility: Mobility::Immovable
//! };
//! ```
//! The prefered way of constructing a [`ObjectDescription`] however
//! is by using an [`ObjectBuilder`].
//!
//! [`ObjectBuilder`]: ../object_builder/struct.ObjectBuilder.html
//! [`ObjectDescription`]: ./struct.ObjectDescription.html

use std::fmt::Debug;

/// A new object that is going to be placed in the [`Simulation`]
///
/// [`Simulation`]: ../trait.Simulation.html
#[derive(Debug, Clone)]
pub struct Object {
    /// The object's behavior, which determines its kind and what the object is going to do every step
    pub object_behavior: ObjectBehavior,
    /// The object's initial position
    pub position: Position,
    /// The object's shape
    pub shape: Polygon,
}

/// Custom behaviour of an object,
/// defining its interactions and whether it is
/// able to be moved by the physics engine or not
#[derive(Debug)]
pub enum ObjectBehavior {
    /// The behaviour of an object that can be moved
    Movable(Box<dyn MovableObject>),
    /// The behaviour of an object that can never be moved
    Immovable(Box<dyn ImmovableObject>),
}

impl Clone for ObjectBehavior {
    fn clone(&self) -> Self {
        match self {
            ObjectBehavior::Movable(object) => ObjectBehavior::Movable(object.clone_box()),
            ObjectBehavior::Immovable(object) => ObjectBehavior::Immovable(object.clone_box()),
        }
    }
}

impl ObjectBehavior {
    /// Returns the object's kind.
    /// This information is arbitrary and is only used
    /// as a tag for visualizers
    pub fn kind(&self) -> Kind {
        match self {
            ObjectBehavior::Movable(object) => object.kind(),
            ObjectBehavior::Immovable(object) => object.kind(),
        }
    }

    /// Returns if a sensor is attached to the object
    pub fn sensor(&self) -> Option<Sensor> {
        match self {
            ObjectBehavior::Movable(object) => object.sensor(),
            ObjectBehavior::Immovable(object) => object.sensor(),
        }
    }
}

/// Behaviour of an object that can be moved
pub trait MovableObject: MovableObjectClone + Debug {
    /// Returns all actions performed by the object
    /// in the current simulation tick
    fn step(&mut self, sensor_collisions: &[ObjectDescription]) -> Option<MovableAction>;

    /// Returns the object's kind.
    /// This information is arbitrary and is only used
    /// as a tag for visualizers
    fn kind(&self) -> Kind;

    /// Returns if a sensor is attached to the object
    fn sensor(&self) -> Option<Sensor>;
}

/// Possible actions performed by a [`MovableObject`]
/// during a simulation step
///
/// [`MovableObject`]: ./trait.MovableObject.html
#[derive(Debug, Clone)]
pub enum MovableAction {
    /// Apply the specified force to the object
    ApplyForce(Force),
    /// Create a new object at the specified location
    Reproduce(Object),
    /// Destroy the object
    Die,
}

/// Behaviour of an object that can never be moved
pub trait ImmovableObject: ImmovableObjectClone + Debug {
    /// Returns all actions performed by the object
    /// in the current simulation tick
    fn step(&mut self, sensor_collisions: &[ObjectDescription]) -> Option<ImmovableAction>;
    /// Returns the object's kind.
    /// This information is arbitrary and is only used
    /// as a tag for visualizers
    fn kind(&self) -> Kind;

    /// Returns if a sensor is attached to the object
    fn sensor(&self) -> Option<Sensor>;
}

/// Possible actions performed by a [`ImmovableObject`]
/// during a simulation step
///
/// [`ImmovableObject`]: ./trait.ImmovableObject.html
#[derive(Debug, Clone)]
pub enum ImmovableAction {
    /// Create a new object at the specified location
    Reproduce(Object),
    /// Destroy the object
    Die,
}

/// A sensor that can be attached to an [`Object`],
/// which will report any collisions to it.
///
/// [`Object`]: ./enum.Object.html
#[derive(Debug, PartialEq, Clone)]
pub struct Sensor {
    /// The shape of the sensor
    pub shape: Polygon,
    /// The shape's position in relation to its
    /// parent [`Object`]
    ///
    /// [`Object`]: ./enum.Object.html
    pub position: Position,
}

/// The dehaviourless description of an object that has
/// been placed inside a [`Simulation`].
///
/// [`Simulation`]: ../simulation/trait.Simulation.html
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectDescription {
    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Polygon,
    /// The current position of the object
    pub position: Position,
    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Mobility,
    /// The object's kind
    pub kind: Kind,
}

/// An object's mobility and, if present, its
/// current [`Velocity`]
///
/// [`Velocity`]: ./struct.Velocity.html
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Mobility {
    /// The object cannot have any velocity as
    /// it cannot be moved. Corresponds to [`ImmovableObject`]
    ///
    /// [`ImmovableObject`]: ./trait.ImmovableObject.html
    Immovable,
    /// A movable object's current velocity. Corresponds to [`MovableObject`]
    ///
    /// [`MovableObject`]: ./trait.MovableObject.html
    Movable(Velocity),
}

/// This type holds the vertices of an object
/// in relation to its center, i.e. [0; 0] means
/// the exact center of the object.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Polygon {
    /// The vertices defining the shape of the object
    pub vertices: Vec<Vertex>,
}

/// The coordinates representing a corner
/// of a polygon in relation to its center
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

/// A position within the world, defined as a combination
/// of location and rotation
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position {
    /// An absolute location
    pub location: Location,
    /// A rotation defined in radians
    pub rotation: Radians,
}

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Radians(pub f64);

/// An absolute location within the world, where
/// [0; 0] is defined as the upper left corner of
/// the [`Simulation`]
///
/// [`Simulation`]: ../simulation/trait.Simulation.html
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

/// The velocity of an object, measured as
/// a two dimensional vector
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

/// The part of an object that is responsible for custom
/// behavior and interactions
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Kind {
    /// An intelligent organism featuring a neural network
    Organism,
    /// A self-spreading plant, ready for consumption
    Plant,
    /// A stationary body of water
    Water,
    /// Impassable terrain
    Terrain,
}

/// Combination of a linear force and its torque,
/// resulting in a rotated force applied to an object
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Force {
    pub linear: LinearForce,
    pub torque: Torque,
}

/// Vector describing linear force
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct LinearForce {
    pub x: i32,
    pub y: i32,
}

/// Force of rotation
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Torque(f64);

pub trait MovableObjectClone {
    fn clone_box(&self) -> Box<dyn MovableObject>;
}

impl<T> MovableObjectClone for T
where
    T: MovableObject + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn MovableObject> {
        Box::new(self.clone())
    }
}

pub trait ImmovableObjectClone {
    fn clone_box(&self) -> Box<dyn ImmovableObject>;
}

impl<T> ImmovableObjectClone for T
where
    T: ImmovableObject + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn ImmovableObject> {
        Box::new(self.clone())
    }
}
