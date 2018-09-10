#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Object {
    pub location: Location,
    pub rectangle: Rectangle,
    pub movement: MovementVector,
    pub kind: Kind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Id(pub(crate) usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
