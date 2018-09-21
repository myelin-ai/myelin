//! This module contains all objects that can be
//! placed in a world and their components.
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
//!
#[derive(Debug)]
pub enum ObjectBehavior {
    Movable(Box<dyn MovableObject>),
    Immovable(Box<dyn ImmovableObject>),
}

pub trait MovableObject: std::fmt::Debug {
    fn step(&mut self) -> Vec<MovableAction>;
    fn kind(&self) -> Kind;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MovableAction {
    Move,
    Rotate,
    Die,
    Reproduce,
}

pub trait ImmovableObject: std::fmt::Debug {
    fn step(&mut self) -> Vec<ImmovableAction>;
    fn kind(&self) -> Kind;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImmovableAction {
    Die,
    Reproduce,
}

/// An objects that can be placed in the world.
/// Its coordinates are relative to the center of
/// the object, which is determined by the [`location`]
///
/// [`location`]: ./struct.ObjectDescription.html#structfield.location
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectDescription {
    /// The vertices defining the shape of the object
    /// in relation to its [`location`]
    ///
    /// [`location`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Polygon,
    pub position: Position,
    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Mobility,
    pub kind: Kind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Mobility {
    Immovable,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub location: Location,
    pub rotation: Radians,
}

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Radians(pub f64);

/// A location within the world
#[derive(Debug, Eq, PartialEq, Clone)]
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
#[derive(Debug, Eq, PartialEq, Clone)]
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
