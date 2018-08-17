pub trait Locatable {
    fn x(&self) -> u32;
    fn y(&self) -> u32;
}

pub trait Rectangle {
    fn length(&self) -> u32;
    fn width(&self) -> u32;
}

pub trait Collidable: Locatable + Rectangle {}

pub trait CollidableContainer {
    fn collidables() -> Vec<Box<Collidable>>;
    fn add_collidable(collidable: Box<Collidable>) -> usize;
    fn remove_collidable(collidable: usize);
    fn update_collidable(id: usize, collidable: Box<Collidable>);
}
