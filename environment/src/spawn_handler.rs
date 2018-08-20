use traits::*;

pub trait SpawnHandler {
    fn spawn(&mut self, collidable: Box<Collidable>) -> usize;
    fn despawn(&mut self, id: usize);
}

pub struct SpawnHandlerImpl;

impl SpawnHandler for SpawnHandlerImpl {
    fn spawn(&mut self, collidable: Box<Collidable>) -> usize {
        unimplemented!()
    }
    fn despawn(&mut self, id: usize) {
        unimplemented!()
    }
}
