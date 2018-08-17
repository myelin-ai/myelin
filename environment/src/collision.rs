use traits::*;

pub trait CollisionHandler: CollidableContainer {}

pub struct QuadTree {}

impl CollidableContainer for QuadTree {
    fn collidables() -> Vec<Box<Collidable>> {
        unimplemented!()
    }
    fn add_collidable(_collidable: Box<Collidable>) -> usize {
        unimplemented!()
    }
    fn remove_collidable(_collidable: usize) {
        unimplemented!()
    }
    fn update_collidable(_id: usize, _collidable: Box<Collidable>) {
        unimplemented!()
    }
}

impl CollisionHandler for QuadTree {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("quad_tree", "quad_tree")
    }
}
