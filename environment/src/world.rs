use traits::*;

pub trait World: Rectangle + CollidableContainer {}

pub struct WorldImpl {
    width: u32,
    length: u32,
}

impl Rectangle for WorldImpl {
    fn width(&self) -> u32 {
        self.width
    }

    fn length(&self) -> u32 {
        self.length
    }
}

impl CollidableContainer for WorldImpl {
    fn collidables() -> Vec<Box<Collidable>> {
        unimplemented!()
    }

    fn add_collidable(collidable: Box<Collidable>) -> usize {
        unimplemented!()
    }

    fn remove_collidable(collidable: usize) {
        unimplemented!()
    }

    fn update_collidable(id: usize, collidable: Box<Collidable>) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
