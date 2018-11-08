//! Types dealing with serialization and deserialization

#[cfg(feature = "use-bincode")]
pub use self::bincode::*;
#[cfg(feature = "use-json")]
pub use self::json::*;
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::fmt::Debug;

#[cfg(feature = "use-bincode")]
mod bincode;
#[cfg(feature = "use-json")]
mod json;

/// A Serializer for [`ViewModelDelta`]s.
/// There should be an accompanying [`ViewModelDeserializer`] for each implementation of this trait.
pub trait ViewModelSerializer: Debug {
    /// Serializes a [`ViewModelDelta`] into a binary representation which can
    /// be deserialized using [`ViewModelDeserializer::deserialize_view_model_delta`].
    ///
    /// [`ViewModelDelta`]: ../view_model_delta/type.ViewModelDelta.html
    /// [`ViewModelDeserializer::deserialize_view_model_delta`]: ./trait.ViewModelDeserializer.html#tymethod.deserialize_view_model_delta
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Deserializes [`ViewModelDelta`]s which were previously serialized with a [`ViewModelSerializer`].
pub trait ViewModelDeserializer: Debug {
    /// Deserializes a [`ViewModelDelta`] from its binary representation.
    ///
    /// [`ViewModelDelta`]: ../view_model_delta/type.ViewModelDelta.html
    fn deserialize_view_model_delta(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>>;
}
