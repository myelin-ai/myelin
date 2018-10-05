use std::error::Error;
use std::fmt::Debug;

pub(crate) trait ViewModelTransmitter: Debug {
    fn send_view_model(&self, view_model: Vec<u8>) -> Result<(), Box<dyn Error>>;
}
