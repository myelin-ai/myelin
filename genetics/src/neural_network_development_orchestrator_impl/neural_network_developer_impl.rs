//! Various implementations of [`NeuralNetworkDeveloper`]
//!
//! [`NeuralNetworkDeveloper`]: ../trait.NeuralNetworkDeveloper.html

pub use self::flat::*;
pub use self::genetic::*;

mod flat;
mod genetic;
