use std::error::Error;
use std::fmt::Debug;

pub(crate) trait ViewModelTransmitter: Debug {
    fn send_view_model(&self, view_model: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

pub(crate) trait ViewModelReceiver: fmt::Debug {
    fn receive_view_model_delta(&self) -> Result<Vec<u8>, Box<dyn Error>>;
}
