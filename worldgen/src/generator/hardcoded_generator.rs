use crate::WorldGenerator;
use myelin_environment::world::World;

#[derive(Debug, Default)]
pub struct HardcodedGenerator;

impl HardcodedGenerator {
    pub fn new(_world_factory: Box<dyn Fn() -> Box<dyn World>>) -> Self {
        Self {}
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&self) -> Box<dyn World> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{GlobalObject, LocalObject};

    #[derive(Debug, Default)]
    struct WorldMock {
        objects: Vec<LocalObject>,
    }

    impl World for WorldMock {
        fn step(&mut self) {
            panic!("step() called unexpectedly")
        }
        fn add_object(&mut self, object: LocalObject) {
            self.objects.push(object)
        }
        fn objects(&self) -> Vec<GlobalObject> {
            panic!("objects() called unexpectedly")
        }
    }
    impl Drop for WorldMock {
        fn drop(&mut self) {
            assert!(self.objects.len() > 0);
        }
    }

    #[test]
    fn generates_world() {
        let world_factory = || -> Box<dyn World> { Box::new(WorldMock::default()) };
        let generator = HardcodedGenerator::new(Box::new(world_factory));

        let _world = generator.generate();
    }
}
