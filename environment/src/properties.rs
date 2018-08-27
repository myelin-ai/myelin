use slab::Slab;

pub type ObjectContainer = Slab<Object>;

#[derive(Debug, Eq, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

impl Location {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rectangle {
    /// Corresponds to x
    pub width: u32,
    /// Corresponds to y
    pub length: u32,
}

impl Rectangle {
    pub fn new(width: u32, length: u32) -> Self {
        Self { width, length }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MovementVector {
    pub x: i32,
    pub y: i32,
}

pub type Id = usize;

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Organism,
    Wall,
    Plant,
    Water,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Object {
    location: Location,
    rectangle: Rectangle,
    movement_vector: MovementVector,
    kind: Kind,
}

impl Object {
    pub fn new(
        location: Location,
        rectangle: Rectangle,
        movement_vector: MovementVector,
        kind: Kind,
    ) -> Self {
        Self {
            location,
            rectangle,
            movement_vector,
            kind,
        }
    }
}
