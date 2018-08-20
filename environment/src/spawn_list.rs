use traits::*;

pub enum SpawnHandlerJob {
    Spawn(Box<Collidable>),
    DespawnId(usize),
}

pub trait SpawnList {
    fn spawn(&mut self, collidable: Box<Collidable>) -> usize;
    fn despawn(&mut self, id: usize);
}

pub struct SpawnListImpl;

impl SpawnList for SpawnListImpl {
    fn spawn(&mut self, _collidable: Box<Collidable>) -> usize {
        unimplemented!()
    }
    fn despawn(&mut self, _id: usize) {
        unimplemented!()
    }
}
