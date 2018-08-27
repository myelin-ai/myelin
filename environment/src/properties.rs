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
    fn location(&self) -> &Location;
    fn location_mut(&mut self) -> &mut Location;

    fn rectangle(&self) -> &Rectangle;
    fn rectangle_mut(&mut self) -> &mut Rectangle;

    fn movement_vector(&self) -> &MovementVector;
    fn movement_vector_mut(&mut self) -> &mut MovementVector;

    fn kind(&self) -> &Kind;
}
