//! This module containes all objects that can be
//! placed in a world and their components.

/// An objects that can be placed in the world.
/// Its coordinates are relative to the center of
/// the object, which is determined by the `location`
/// The following would define a stationary square terrain:
/// ```
/// use myelin_environment::object::*;
///
/// let square = LocalObject {
///     shape: LocalPolygon {
///         vertices: vec![
///             LocalVertex { x: -50, y: -50 },
///             LocalVertex { x: -50, y: 50 },
///             LocalVertex { x: 50, y: 50 },
///             LocalVertex { x: 50, y: -50 },
///         ],
///     },
///     orientation: Radians(0.0),
///     location: Location { x: 100, y: 100 },
///     velocity: Velocity { x: 0, y: 0 },
///     kind: Kind::Terrain,
/// };
/// ```
/// The prefered way of constructing these however
/// is by using an ObjectBuilder
#[derive(Debug, PartialEq, Clone)]
pub struct LocalObject {
    /// The vertices defining the shape of the object
    /// in relation to its locaion
    pub shape: LocalPolygon,
    /// The global position of center of the object
    pub location: Location,
    /// The orientation of the object, measured in
    /// radians within the range [0.0; 2π).
    /// An orientation of 0.0 means that the
    /// object is facing right.
    pub orientation: Radians,
    /// The initial velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub velocity: Velocity,
    /// The kind of object we are dealing with
    pub kind: Kind,
}

/// An objects that has already been placed in the world.
/// Its coordinates are global, using the upper left corner of
/// the world as their origin.
/// The following would define a stationary square terrain and
/// represents the analog to the example shown in the documentation
/// of LocalObject:
/// ```
/// use myelin_environment::object::*;
///
/// let square = GlobalObject {
///     shape: GlobalPolygon {
///         vertices: vec![
///             GlobalVertex { x: 50, y: 50 },
///             GlobalVertex { x: 150, y: 50 },
///             GlobalVertex { x: 150, y: 150 },
///             GlobalVertex { x: 50, y: 150 },
///         ],
///     },
///     orientation: Radians(0.0),
///     velocity: Velocity { x: 0, y: 0 },
///     kind: Kind::Terrain,
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalObject {
    pub shape: GlobalPolygon,
    pub orientation: Radians,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalPolygon {
    pub vertices: Vec<LocalVertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalPolygon {
    pub vertices: Vec<GlobalVertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalVertex {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalVertex {
    pub x: u32,
    pub y: u32,
}

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Radians(pub f32);

/// A location within the world
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

/// The velocity of an object, measured as
/// a two dimensional vector
#[derive(Debug, Eq, PartialEq, Clone)]
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
