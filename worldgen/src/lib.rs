#![deny(rust_2018_idioms, missing_debug_implementations)]

use myelin_environment::world::World;

pub mod generator;

pub trait WorldGenerator: std::fmt::Debug {
    fn generate(&self) -> Box<dyn World>;
}
