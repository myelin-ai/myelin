use crate::properties::*;

pub trait World: Rectangle {}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
