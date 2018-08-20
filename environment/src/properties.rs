use crate::collidable_container::CollidableId;
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

pub trait Collidable: Locatable + Rectangle + Debug {
    fn id(&self) -> CollidableId;
}
