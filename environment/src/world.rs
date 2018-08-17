use super::traits::*;

pub trait World: Rectangle {
    fn collidables() -> Vec<Box<Collidable>>;
}

pub struct WorldImpl {
    width: u32,
    length: u32,
}

impl Rectangle for WorldImpl {
    fn width(&self) -> u32 {
        self.width
    }
    fn length(&self) -> u32 {
        self.length
    }
}

impl World for WorldImpl {
    fn collidables() -> Vec<Box<Collidable>> {
        unimplemented!()
    }
}
