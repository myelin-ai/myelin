use super::{Object, Quadtree};

#[derive(Debug, Eq, PartialEq)]
pub struct QuadtreeBuilder {
    bucket_capacity: u32,
}

impl QuadtreeBuilder {
    pub fn split_at(self, _bucket_capacity: u32) -> Self {
        unimplemented!();
    }

    pub fn build<'a>(self, _objects: &'a [Object]) -> Quadtree<'a> {
        unimplemented!();
    }
}

impl Default for QuadtreeBuilder {
    fn default() -> Self {
        QuadtreeBuilder { bucket_capacity: 4 }
    }
}
