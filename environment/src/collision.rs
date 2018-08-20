use traits::*;

pub trait Collidable: Locatable + Rectangle {
    fn on_collision(&mut self, other: &Collidable);
}

pub trait CollidableContainer {
    fn collidables(&self) -> Vec<Box<Collidable>>;
    fn add_collidable(&mut self, collidable: Box<Collidable>) -> usize;
    fn remove_collidable(&mut self, collidable: usize);
    fn update_collidable(&mut self, id: usize, collidable: Box<Collidable>);
}

pub trait CollisionHandler: CollidableContainer {
    fn resolve_collisions(&mut self);
}

pub struct QuadTree {}

impl CollidableContainer for QuadTree {
    fn collidables(&self) -> Vec<Box<Collidable>> {
        unimplemented!()
    }
    fn add_collidable(&mut self, _collidable: Box<Collidable>) -> usize {
        unimplemented!()
    }
    fn remove_collidable(&mut self, _collidable: usize) {
        unimplemented!()
    }
    fn update_collidable(&mut self, _id: usize, _collidable: Box<Collidable>) {
        unimplemented!()
    }
}

impl CollisionHandler for QuadTree {
    fn resolve_collisions(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
