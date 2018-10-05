use myelin_visualization_core::view_model::ViewModel;
use std::error::Error;
use std::fmt::Debug;

pub(crate) trait ViewModelDeserializer: Debug {
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModel, Box<dyn Error>>;
}
