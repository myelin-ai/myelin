#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

impl Location {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MovementVector {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    Organism,
    Wall,
    Plant,
    Water,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Object {
    pub location: Location,
    pub rectangle: Rectangle,
    pub movement: MovementVector,
    pub kind: Kind,
}

#[derive(Debug, Copy, Clone)]
pub struct ObjectId(pub(crate) usize);
