#![deny(rust_2018_idioms)]

use myelin_environment::world::World;

pub mod generator;

pub trait WorldGenerator {
    fn generate(&self) -> Box<dyn World>;
}
