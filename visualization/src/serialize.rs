use crate::view_model::ViewModel;
use std::error::Error;
use std::fmt::Debug;

pub(crate) trait ViewModelSerializer: Debug {
    fn serialize_view_model(&self, view_model: &ViewModel) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub(crate) trait ViewModelDeserializer: Debug {
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModel, Box<dyn Error>>;
}
