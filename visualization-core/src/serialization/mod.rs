pub use self::json::*;
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

mod json;

pub trait ViewModelSerializer: Debug {
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub trait ViewModelDeserializer: Debug {
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>>;
}
