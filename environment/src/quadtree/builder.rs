use super::{Object, QuadTree};

#[derive(Debug, Eq, PartialEq)]
pub struct QuadTreeBuilder {
    bucket_capacity: u32,
}

impl QuadTreeBuilder {
    pub fn split_at(self, _bucket_capacity: u32) -> Self {
        unimplemented!();
    }

    pub fn build<'a>(self, _objects: &'a [Object]) -> QuadTree<'a> {
        unimplemented!();
    }
}

impl Default for QuadTreeBuilder {
    fn default() -> Self {
        QuadTreeBuilder { bucket_capacity: 4 }
    }
}
