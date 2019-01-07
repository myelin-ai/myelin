//! Neural networks and their components

mod connection;
pub mod spiking_neural_network;
pub use self::connection::*;

#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use std::collections::HashMap;
use std::fmt::Debug;

/// A handle to a [`Neuron`]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Handle(pub usize);

/// A modifier of incoming spikes
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Weight(pub f64);

/// A representation of time
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TimeInMilliseconds(pub f64);

/// The state of a neuron at a given time
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MembranePotential(pub f64);

/// The result type used when working with handles
pub type Result<T> = std::result::Result<T, ()>;

/// A neural network that supports construction from multiple neurons and arbitrary connections between them
#[cfg_attr(test, mockable)]
pub trait NeuralNetwork: Debug {
    /// Update the state of all neurons
    fn step(
        &mut self,
        time_since_last_update: TimeInMilliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    );

    /// Returns the last calculated state of the neuron referenced by `handle`
    fn membrane_potential_of_neuron(&self, neuron: Handle) -> Result<MembranePotential>;

    /// Add a new unconnected sensor to the network
    fn push_sensor(&mut self) -> Handle;

    /// Add a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle;

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<()>;
}
