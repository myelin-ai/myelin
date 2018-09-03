use crate::object::*;

pub trait World {
    fn rectangle(&self) -> Rectangle;
}

#[derive(Debug)]
pub struct WorldImpl {
    width: u32,
    length: u32,
    object_ids: Vec<Id>,
}
