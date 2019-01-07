//! Implementation of a spiking neural network

pub(crate) mod constant;
mod neuron;
pub use self::neuron::*;
use crate::neural_network::*;

/// A spiking neural network
#[derive(Debug, Default)]
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
    fn membrane_potential_of_neuron(&self, neuron: Handle) -> Result<MembranePotential> {
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
    fn add_connection(&mut self, connection: Connection) -> Result<()> {
        unimplemented!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn empty_network_has_no_membrane_potential() {
        let invalid_handle = Handle(1337);
        let neural_network = SpikingNeuralNetwork::default();
        let result = neural_network.membrane_potential_of_neuron(invalid_handle);
        assert!(result.is_err());
    }

    #[test]
    fn can_push_sensors_and_neurons() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let neuron_handle = neural_network.push_neuron();
        let sensor_handle = neural_network.push_sensor();
        assert_ne!(neuron_handle.0, sensor_handle.0);
    }

    #[test]
    fn invalid_handle_has_no_membrane_potential() {
        let invalid_handle = Handle(1337);
        let mut neural_network = SpikingNeuralNetwork::default();
        let _valid_handle = neural_network.push_neuron();
        let result = neural_network.membrane_potential_of_neuron(invalid_handle);
        assert!(result.is_err());
    }

    #[test]
    fn can_retrieve_membrane_potential_from_valid_handle() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let neuron_handle = neural_network.push_neuron();
        let result = neural_network.membrane_potential_of_neuron(neuron_handle);
        assert!(result.is_ok());
    }

    #[test]
    fn new_neuron_is_at_resting_potential() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let neuron_handle = neural_network.push_neuron();
        let membrane_potential = neural_network
            .membrane_potential_of_neuron(neuron_handle)
            .unwrap();
        assert_eq!(membrane_potential.0, constant::RESTING_POTENTIAL);
    }

    #[test]
    fn new_sensor_is_at_resting_potential() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_sensor();
        let membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        assert_eq!(membrane_potential.0, constant::RESTING_POTENTIAL);
    }
}
