//! Implementation of a spiking neural network

pub(crate) mod constant;
pub mod neuron;
use crate::neural_network::*;

/// A spiking neural network
#[derive(Debug)]
pub struct SpikingNeuralNetwork;

impl NeuralNetwork for SpikingNeuralNetwork {
    /// Update the state of all neurons
    fn step(
        &mut self,
        time_since_last_update: TimeInMilliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    ) {
        unimplemented!()
    }

    /// Returns the last calculated state of the neuron referenced by `handle`
    fn membrane_potential_of_neuron(&self, neuron: Handle) -> Option<MembranePotential> {
        unimplemented!()
    }

    /// Add a new unconnected sensor to the network
    fn push_sensor(&mut self) -> Handle {
        unimplemented!()
    }

    /// Add a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle {
        unimplemented!()
    }

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<(), ()> {
        unimplemented!()
    }
}
