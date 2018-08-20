use crate::traits::*;

pub enum SpawnHandlerJob {
    Spawn(Box<dyn Collidable>),
    DespawnId(usize),
}

pub trait SpawnList {
    fn spawn(&mut self, collidable: Box<dyn Collidable>) -> usize;
    fn despawn(&mut self, id: usize);
}

pub struct SpawnListImpl;

impl SpawnList for SpawnListImpl {
    fn spawn(&mut self, _collidable: Box<dyn Collidable>) -> usize {
        unimplemented!()
    }
    fn despawn(&mut self, _id: usize) {
        unimplemented!()
    }
}
