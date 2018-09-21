#![deny(rust_2018_idioms, missing_debug_implementations)]

mod organism;
mod plant;
mod terrain;
mod water;

pub use self::organism::Organism;
pub use self::plant::Plant;
pub use self::terrain::Terrain;
pub use self::water::Water;
