//! Implementations of various world generation algorithms

mod hardcoded_generator;
pub use self::hardcoded_generator::HardcodedGenerator;
use myelin_object_data::Kind;

#[cfg(test)]
use mockiato::mockable;

/// Provides names for objects
#[cfg_attr(test, mockable)]
pub trait NameProvider {
    /// Returns a unique name every time it's called
    fn get_name(&mut self, kind: Kind) -> Option<String>;
}
