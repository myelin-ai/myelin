use crate::WorldGenerator;
use myelin_environment::object::Kind;
use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
use myelin_environment::world::World;

type WorldFactory = Box<dyn Fn() -> Box<dyn World>>;

pub struct HardcodedGenerator {
    world_factory: WorldFactory,
}

impl HardcodedGenerator {
    pub fn new(world_factory: WorldFactory) -> Self {
        Self { world_factory }
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&self) -> Box<dyn World> {
        let mut world = (self.world_factory)();

        world.add_object(
            ObjectBuilder::new()
                .shape(
                    PolygonBuilder::new()
                        .vertex(0, 0)
                        .vertex(10, 0)
                        .vertex(0, 10)
                        .vertex(10, 10)
                        .build()
                        .expect("Generated an invalid vertex"),
                ).location(30, 40)
                .velocity(4, 5)
                .kind(Kind::Organism)
                .build()
                .expect("Generated an invalid object"),
        );
        world
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
