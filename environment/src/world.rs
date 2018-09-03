use crate::properties::*;

pub trait World {
    fn rectangle(&self) -> Rectangle;
}

#[derive(Debug)]
pub struct WorldImpl {
    width: u32,
    length: u32,
    object_ids: Vec<ObjectId>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
