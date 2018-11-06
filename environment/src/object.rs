//! Objects that can be placed in a world and their components.
//! You can construct a [`ObjectDescription`] by using an [`ObjectBuilder`].
//!
//! [`ObjectBuilder`]: ../object_builder/struct.ObjectBuilder.html
//! [`ObjectDescription`]: ./struct.ObjectDescription.html

use std::f64::consts::PI;
use std::fmt::Debug;

/// Behaviour of an object that can never be moved
pub trait ObjectBehavior: Debug + ObjectBehaviorClone {
    /// Returns all actions performed by the object
    /// in the current simulation tick
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        sensor_collisions: &[ObjectDescription],
    ) -> Option<Action>;
}

/// Possible actions performed by an [`Object`]
/// during a simulation step
///
/// [`Object`]: ./trait.Object.html
#[derive(Debug)]
pub enum Action {
    /// Apply the specified force to the object
    ApplyForce(Force),
    /// Create a new object at the specified location
    Reproduce(ObjectDescription, Box<dyn ObjectBehavior>),
    /// Destroy the object
    Die,
}

impl Clone for Action {
    fn clone(&self) -> Action {
        match self {
            Action::ApplyForce(force) => Action::ApplyForce(force.clone()),
            Action::Reproduce(object_description, object_behavior) => {
                Action::Reproduce(object_description.clone(), object_behavior.clone_box())
            }
            Action::Die => Action::Die,
        }
    }
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
#[non_exhaustive]
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

    /// The object's sensor
    pub sensor: Option<Sensor>,

    /// Whether the object is passable or not
    pub passable: bool,
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
    /// The x component of the [`Vertex`]
    pub x: i32,
    /// The y component of the [`Vertex`]
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
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Creates a new instance of [`Radians`].
    /// Returns `None` if the given value is outside the range [0.0; 2π)
    ///
    /// ### Examples
    /// ```
    /// use myelin_environment::object::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let rotation = Radians::new(PI).expect("Value was outside the range [0.0; 2π)")
    /// ```
    pub fn try_new(value: f64) -> Option<Radians> {
        if value >= 0.0 && value < 2.0 * PI {
            Some(Radians { value })
        } else {
            None
        }
    }

    /// Returns the underlying value
    pub fn value(self) -> f64 {
        self.value
    }
}

/// An absolute location within the world, where
/// [0; 0] is defined as the upper left corner of
/// the [`Simulation`]
///
/// [`Simulation`]: ../simulation/trait.Simulation.html
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Location {
    /// The x component of the [`Location`]
    pub x: u32,
    /// The y component of the [`Location`]
    pub y: u32,
}

/// The velocity of an object, measured as
/// a two dimensional vector
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Velocity {
    /// The x component of the [`Velocity`]
    pub x: i32,
    /// The y component of the [`Velocity`]
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
    /// The linear component of the [`Force`]
    pub linear: LinearForce,
    /// The torque (rotation) component of the [`Force`]
    pub torque: Torque,
}

/// Vector describing linear force
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct LinearForce {
    /// The x component of the [`LinearForce`]
    pub x: i32,
    /// The y component of the [`LinearForce`]
    pub y: i32,
}

/// Force of rotation
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Torque(pub f64);

/// Supertrait used to make sure that all implementors
/// of [`ObjectBehavior`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`ObjectBehavior`]: ./trait.ObjectBehavior.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait ObjectBehaviorClone {
    fn clone_box(&self) -> Box<dyn ObjectBehavior>;
}

impl<T> ObjectBehaviorClone for T
where
    T: ObjectBehavior + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn ObjectBehavior> {
        box self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn radians_new_with_negative_0_point_1_is_none() {
        let radians = Radians::try_new(-0.1);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_new_with_0_is_some() {
        let radians = Radians::try_new(0.0);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_1_point_9_pi_is_some() {
        let radians = Radians::try_new(1.9 * PI);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_2_pi_is_none() {
        let radians = Radians::try_new(2.0 * PI);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_value_returns_1_when_given_1() {
        let value = 1.0;
        let radians = Radians::try_new(value).unwrap();
        assert_eq!(value, radians.value())
    }

}
