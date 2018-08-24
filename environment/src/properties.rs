use std::fmt::Debug;

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

pub trait Object: Debug {
    fn location(&self) -> Location;
    fn set_location(&mut self, location: Location);

    fn rectangle(&self) -> Rectangle;
    fn set_rectangle(&mut self, rectangle: Rectangle);

    fn movement_vector(&self) -> MovementVector;
    fn set_movement_vector(&mut self, movement_vector: MovementVector);

    fn kind(&self) -> Kind;
}
