use crate::serialization::JsonSerializer;
use crate::serialization::ViewModelSerializer;
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::marker::PhantomData;

impl JsonSerializer {
    pub fn new() -> Self {
        JsonSerializer(PhantomData)
    }
}

impl ViewModelSerializer for JsonSerializer {
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let serialized = serde_json::to_string(view_model_delta)?;

        Ok(serialized.into())
    }
}
