use std::fmt::Debug;

pub(crate) ViewModelTransmitter: Debug {
    fn send_view_model(&self, view_model: Vec<u8>) -> Result<(), Box<Error>>;
}
