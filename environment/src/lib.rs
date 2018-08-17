pub trait Locatable {
    fn x() -> u32;
    fn y() -> u32;
}

pub trait Rectangle {
    fn height() -> u32;
    fn width() -> u32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
