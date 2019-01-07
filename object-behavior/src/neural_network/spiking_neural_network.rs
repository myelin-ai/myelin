//! Implementation of a spiking neural network

pub(crate) mod constant;
mod neuron;
pub use self::neuron::*;
use crate::neural_network::*;

use slab::Slab;
use std::collections::HashMap;

/// A spiking neural network
#[derive(Debug, Default)]
pub struct SpikingNeuralNetwork {
    neurons: Slab<SpikingNeuron>,
    last_state: HashMap<Handle, MembranePotential>,
    connections: HashMap<Handle, Vec<(Handle, Weight)>>,
    sensors: Vec<Handle>,
}

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
        self.last_state.get(&neuron).map(|&state| state).ok_or(())
    }

    /// Add a new unconnected sensor to the network
    fn push_sensor(&mut self) -> Handle {
        let handle = self.push_neuron();
        self.sensors.push(handle);
        handle
    }

    /// Add a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle {
        let handle = Handle(self.neurons.insert(SpikingNeuron::default()));
        self.last_state.insert(handle, MembranePotential(constant::RESTING_POTENTIAL));
        handle
    }

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<()> {
        let is_origin_same_as_destination = connection.from == connection.to;
        let contains_origin = self.neurons.contains(connection.from.0);
        let contains_destination = self.neurons.contains(connection.to.0);
        let is_destination_a_sensor = self.sensors.contains(&connection.to);
        if is_origin_same_as_destination
            || !contains_origin
            || !contains_destination
            || is_destination_a_sensor
        {
            Err(())
        } else {
            self.connections
                .entry(connection.to)
                .or_default()
                .push((connection.from, connection.weight));
            Ok(())
        }
    }
}

#[cfg(test)]
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
        let mut neural_network = SpikingNeuralNetwork::default();
        let valid_handle = neural_network.push_neuron();
        let invalid_handle = Handle(valid_handle.0 + 1);
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
        assert_eq!(constant::RESTING_POTENTIAL, membrane_potential.0);
    }

    #[test]
    fn new_sensor_is_at_resting_potential() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_sensor();
        let membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        assert_eq!(constant::RESTING_POTENTIAL, membrane_potential.0);
    }

    #[test]
    fn returns_err_when_adding_connection_on_empty_network() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let connection = Connection {
            from: Handle(0),
            to: Handle(1),
            weight: Weight(1.0),
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_err());
    }

    #[test]
    fn returns_err_when_adding_connection_with_invalid_handles() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_sensor();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(sensor_handle.0 + 1),
            to: Handle(neuron_handle.0 + 1),
            weight: Weight(1.0),
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_err());
    }

    #[test]
    fn returns_err_when_adding_connection_sensor_as_destination() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_sensor();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(neuron_handle.0),
            to: Handle(sensor_handle.0),
            weight: Weight(1.0),
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_err());
    }

    #[test]
    fn returns_err_when_adding_connection_with_same_origin_as_destination() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(neuron_handle.0),
            to: Handle(neuron_handle.0),
            weight: Weight(1.0),
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_err());
    }

    #[test]
    fn returns_ok_when_adding_connection_with_valid_handles() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_sensor();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(sensor_handle.0),
            to: Handle(neuron_handle.0),
            weight: Weight(1.0),
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_ok());
    }
}
