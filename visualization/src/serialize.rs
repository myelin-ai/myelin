use crate::view_model::ViewModel;
use std::error::Error;

pub(crate) trait ViewModelSerializer {
    fn serialize_view_model(&self, view_model: &ViewModel) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub(crate) trait ViewModelDeserializer {
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModel, Box<dyn Error>>;
}
