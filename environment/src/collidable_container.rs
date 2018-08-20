use crate::traits::Collidable;

pub trait CollidableContainer {
    fn collidables(&self) -> Vec<Box<dyn Collidable>>;
    fn add_collidable(&mut self, collidable: Box<dyn Collidable>) -> usize;
    fn remove_collidable(&mut self, collidable: usize);
    fn update_collidable(&mut self, id: usize, collidable: Box<dyn Collidable>);
}
