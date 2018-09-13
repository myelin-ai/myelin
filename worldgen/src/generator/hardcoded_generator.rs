use crate::WorldGenerator;
use myelin_environment::world::World;

#[derive(Debug, Default)]
pub struct HardcodedGenerator;

impl HardcodedGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&self, _world: &mut dyn World) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{GlobalObject, LocalObject};

    #[derive(Debug, Default)]
    struct WorldMock {
        pub objects: Vec<LocalObject>,
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

    #[test]
    fn generates_world() {
        let mut world = WorldMock::default();
        let generator = HardcodedGenerator::new();

        generator.generate(&mut world);

        let passed_objects = world.objects;
        assert!(passed_objects.len() > 0);
    }
}
