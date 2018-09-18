//! This module contains all objects that can be
//! placed in a world and their components.
//! # Examples
//! The following defines a stationary square terrain:
//! ```
//! use myelin_environment::object::*;
//!
//! let square = LocalBody {
//!     shape: LocalPolygon {
//!         vertices: vec![
//!             LocalVertex { x: -50, y: -50 },
//!             LocalVertex { x: -50, y: 50 },
//!             LocalVertex { x: 50, y: 50 },
//!             LocalVertex { x: 50, y: -50 },
//!         ],
//!     },
//!     orientation: Radians(0.0),
//!     location: Location { x: 100, y: 100 },
//!     kind: Kind::Terrain,
//! };
//! ```
//! The prefered way of constructing a [`LocalBody`] however
//! is by using an [`ObjectBuilder`].
//!
//! The global analog to the last example looks like this:
//! ```
//! use myelin_environment::object::*;
//!
//! let square = GlobalBody {
//!     shape: GlobalPolygon {
//!         vertices: vec![
//!             GlobalVertex { x: 50, y: 50 },
//!             GlobalVertex { x: 150, y: 50 },
//!             GlobalVertex { x: 150, y: 150 },
//!             GlobalVertex { x: 50, y: 150 },
//!         ],
//!     },
//!     orientation: Radians(0.0),
//!     velocity: Velocity { x: 0, y: 0 },
//!     kind: Kind::Terrain,
//! };
//! ```
//!
//! [`ObjectBuilder`]: ../object_builder/struct.ObjectBuilder.html
//! [`LocalBody`]: ./struct.LocalBody.html

/// An objects that can be placed in the world.
/// Its coordinates are relative to the center of
/// the object, which is determined by the [`location`]
///
/// [`location`]: ./struct.LocalBody.html#structfield.location
#[derive(Debug, PartialEq, Clone)]
pub struct LocalBody {
    /// The vertices defining the shape of the object
    /// in relation to its [`location`]
    ///
    /// [`location`]: ./struct.LocalBody.html#structfield.location
    pub shape: LocalPolygon,
    /// The global position of the center of the object
    pub location: Location,
    /// The orientation of the object, measured in
    /// radians within the range [0.0; 2π).
    /// An orientation of 0.0 means that the
    /// object is facing right.
    pub orientation: Radians,
}

/// An object that has already been placed in the world.
/// Its coordinates are global, using the upper left corner of
/// the world as their origin.
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalBody {
    /// The global vertices defining the shape of the object
    /// in relation to the upper left corner of the world
    pub shape: GlobalPolygon,
    /// The orientation of the object, measured in
    /// radians within the range [0.0; 2π).
    /// An orientation of 0.0 means that the
    /// object is facing right.
    pub orientation: Radians,
    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub velocity: Velocity,
}

/// This type holds the vertices of an object
/// in relation to its center, i.e. [0; 0] means
/// the exact center of the object.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalPolygon {
    /// The vertices defining the shape of the object
    pub vertices: Vec<LocalVertex>,
}

/// This type holds the vertices of an object
/// in relation to the world, i.e. [0; 0] means
/// the upper left corner of the world.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalPolygon {
    /// The vertices defining the shape of the object
    pub vertices: Vec<GlobalVertex>,
}

/// The coordinates representing a corner
/// of a polygon in relation to its center
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalVertex {
    pub x: i32,
    pub y: i32,
}

/// The coordinates representing a corner
/// of a polygon in relation to the world origin
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalVertex {
    pub x: u32,
    pub y: u32,
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

#[derive(Debug)]
pub struct LocalObject {
    pub body: LocalBody,
    pub behavior: Box<dyn ObjectBehavior>,
}

#[derive(Debug)]
pub struct GlobalObject<'a> {
    pub body: GlobalBody,
    pub behavior: &'a dyn ObjectBehavior,
}

pub trait ObjectBehavior: std::fmt::Debug {
    fn step(&mut self) -> Vec<Action>;
    fn is_movable(&self) -> bool;
    fn kind(&self) -> Kind;
}

#[derive(Debug)]
pub enum Action {
    Move,
    Rotate,
    Die,
    Reproduce,
}
