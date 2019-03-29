use super::{Handle, Weight};

/// The synaptic connection between two neurons and its strength.
#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    /// The handle of the origin neuron.
    pub from: Handle,
    /// The handle of the destination neuron.
    pub to: Handle,
    /// The weight of the connection.
    pub weight: Weight,
}
