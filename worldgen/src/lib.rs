use myelin_environment::world::World;

pub mod generator;

pub trait WorldGenerator {
    fn generate(&self, world: &mut dyn World);
}
