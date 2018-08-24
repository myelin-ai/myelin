use std::fmt::Debug;

pub trait Locatable {
    fn x(&self) -> u32;
    fn y(&self) -> u32;
}

pub trait Rectangle {
    fn length(&self) -> u32;
    fn width(&self) -> u32;
}

#[derive(Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

pub type Id = usize;

pub trait Identifiable {
    fn id(&self) -> Id;
}

#[derive(Debug)]
pub enum Kind {
    Organism,
    Wall,
}

pub trait Object: Locatable + Rectangle + Identifiable + Debug {
    fn kind(&self) -> Kind;
}
