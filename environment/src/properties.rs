#[derive(Debug)]
pub struct Location {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
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
    location: Location,
    rectangle: Rectangle,
    movement_vector: MovementVector,
    kind: Kind,
}
