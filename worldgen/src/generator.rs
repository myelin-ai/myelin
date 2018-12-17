//! Implementations of various world generation algorithms

mod hardcoded_generator;
pub use self::hardcoded_generator::HardcodedGenerator;
use myelin_environment::object::Kind;

/// Provides names for objects
pub trait NameProvider {
    /// Returns a unique name every time it's called
    fn get_name(&mut self, kind: Kind) -> Option<String>;
}
