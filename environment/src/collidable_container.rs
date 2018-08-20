use crate::properties::Collidable;

pub type CollidableId = usize;

pub trait CollidableContainer {
    fn collidables(&self) -> Vec<Box<dyn Collidable>>;
    fn add_collidable(&mut self, collidable: Box<dyn Collidable>) -> CollidableId;
    fn remove_collidable(&mut self, id: CollidableId);
    fn update_collidable(&mut self, id: CollidableId, collidable: Box<dyn Collidable>);
}
