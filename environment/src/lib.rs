pub trait Locatable {
    fn x(&self) -> u32;
    fn y(&self) -> u32;
}

pub trait Rectangle {
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
