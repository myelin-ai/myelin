//! Neural networks and their components

#![feature(specialization)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

mod connection;
pub mod spiking_neural_network;
pub use self::connection::*;

#[cfg(test)]
use maplit::hashmap;

#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use std::collections::HashMap;
use std::fmt::Debug;

/// A handle to a neuron
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Handle(pub usize);

/// A modifier of incoming spikes
pub type Weight = f64;

/// A representation of time
pub type Milliseconds = f64;

/// The state of a neuron at a given time
pub type MembranePotential = f64;

/// The result type used when working with handles
pub type Result<T> = std::result::Result<T, ()>;

/// A neural network that supports construction from multiple neurons and arbitrary connections between them
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait NeuralNetwork: Debug + NeuralNetworkClone {
    /// Update the state of all neurons
    fn step(
        &mut self,
        time_since_last_step: Milliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    );

    /// Returns the last calculated state of the neuron referenced by `handle`
    fn membrane_potential_of_neuron(&self, neuron: Handle) -> Result<Option<MembranePotential>>;

    /// Add a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle;

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<()>;
}

/// Supertrait used to make sure that all implementors
/// of [`NeuralNetwork`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`NeuralNetwork`]: ./trait.NeuralNetwork.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait NeuralNetworkClone {
    fn clone_box(&self) -> Box<dyn NeuralNetwork>;
}

impl<T> NeuralNetworkClone for T
where
    T: NeuralNetwork + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn NeuralNetwork> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn NeuralNetwork> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
