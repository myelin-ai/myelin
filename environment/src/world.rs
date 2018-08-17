use Rectangle;

pub trait World: Rectangle {}

pub struct WorldImpl {
    width: u32,
    height: u32,
}

impl Rectangle for WorldImpl {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}

impl World for WorldImpl {}
