use crate::collidable_container::CollidableId;

pub trait Locatable {
    fn x(&self) -> u32;
    fn y(&self) -> u32;
}

pub trait Rectangle {
    fn length(&self) -> u32;
    fn width(&self) -> u32;
}

pub struct Vector {
    pub x: i32,
    pub y: i32,
}

pub enum SpawnEvent {
    Spawn(Box<dyn Fn(CollidableId) -> Box<dyn Collidable>>),
    Despawn(CollidableId),
    Move(CollidableId, Vector),
}

pub trait Collidable: Locatable + Rectangle {
    fn id(&self) -> CollidableId;
}
