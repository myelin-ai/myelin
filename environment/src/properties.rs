#[derive(Debug)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

#[derive(Debug)]
pub struct MovementVector {
    pub x: i32,
    pub y: i32,
}

pub type Id = usize;

#[derive(Debug)]
pub enum Kind {
    Organism,
    Wall,
    Plant,
    Water,
}

#[derive(Debug)]
pub struct Object {
    pub location: Location,
    pub rectangle: Rectangle,
    pub movement: MovementVector,
    pub kind: Kind,
}
