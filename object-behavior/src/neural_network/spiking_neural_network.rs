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
    neuron_handles: Vec<Handle>,
    last_state: HashMap<Handle, Option<MembranePotential>>,
    incoming_connections: HashMap<Handle, Vec<(Handle, Weight)>>,
}

impl NeuralNetwork for SpikingNeuralNetwork {
    /// Update the state of all neurons
    fn step(
        &mut self,
        time_since_last_step: Milliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    ) {
        for (&handle_of_neuron_receiving_input, &input) in external_inputs {
            let mut inputs =
                self.cached_incoming_connection_inputs(handle_of_neuron_receiving_input);
            const EXTERNAL_CONNECTION_WEIGHT: Weight = Weight(1.0);
            inputs.push((input, EXTERNAL_CONNECTION_WEIGHT));

            let neuron = self
                .neurons
                .get_mut(handle_of_neuron_receiving_input.0)
                .ok_or(())
                .unwrap();
            let state = neuron.step(time_since_last_step, &inputs);
            self.last_state
                .insert(handle_of_neuron_receiving_input, state);
        }
        for &neuron_handle in self
            .neuron_handles
            .iter()
            .filter(|handle| !external_inputs.contains_key(handle))
        {
            let inputs = self.cached_incoming_connection_inputs(neuron_handle);
            let neuron = self.neurons.get_mut(neuron_handle.0).ok_or(()).unwrap();
            let state = neuron.step(time_since_last_step, &inputs);
            self.last_state.insert(neuron_handle, state);
        }
    }

    /// Returns the last calculated state of the neuron referenced by `handle`
    fn membrane_potential_of_neuron(&self, neuron: Handle) -> Result<Option<MembranePotential>> {
        self.last_state.get(&neuron).cloned().ok_or(())
    }

    /// Add a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle {
        let handle = Handle(self.neurons.insert(SpikingNeuron::default()));
        self.last_state.insert(handle, None);
        handle
    }

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<()> {
        let is_origin_same_as_destination = connection.from == connection.to;
        let contains_origin = self.neurons.contains(connection.from.0);
        let contains_destination = self.neurons.contains(connection.to.0);
        if is_origin_same_as_destination || !contains_origin || !contains_destination {
            Err(())
        } else {
            self.incoming_connections
                .entry(connection.to)
                .or_default()
                .push((connection.from, connection.weight));
            Ok(())
        }
    }
}

impl SpikingNeuralNetwork {
    fn cached_incoming_connection_inputs(
        &self,
        neuron_handle: Handle,
    ) -> Vec<(MembranePotential, Weight)> {
        if let Some(incoming_connections) = self.incoming_connections.get(&neuron_handle) {
            incoming_connections
                .iter()
                .filter_map(|(handle_of_connection, weight)| {
                    let &state_of_connection = self.last_state.get(handle_of_connection)?;
                    if let Some(state_of_connection) = state_of_connection {
                        Some((state_of_connection, *weight))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
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
    fn can_push_neurons() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let neuron_handle = neural_network.push_neuron();
        let sensor_handle = neural_network.push_neuron();
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
    fn new_neuron_emits_no_potential() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let neuron_handle = neural_network.push_neuron();
        let membrane_potential = neural_network
            .membrane_potential_of_neuron(neuron_handle)
            .unwrap();
        assert!(membrane_potential.is_none());
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
        let sensor_handle = neural_network.push_neuron();
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
        let sensor_handle = neural_network.push_neuron();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(sensor_handle.0),
            to: Handle(neuron_handle.0),
            weight: Weight(1.0),
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_ok());
    }

    #[test]
    fn step_works_on_empty_network() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let elapsed_time = Milliseconds(1.0);
        let inputs = HashMap::new();
        neural_network.step(elapsed_time, &inputs);
    }

    #[test]
    fn step_on_unconnected_neurons_emits_no_potential() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();
        let neuron_handle = neural_network.push_neuron();

        let elapsed_time = Milliseconds(1.0);
        let inputs = HashMap::new();
        neural_network.step(elapsed_time, &inputs);

        let neuron_membrane_potential = neural_network
            .membrane_potential_of_neuron(neuron_handle)
            .unwrap();
        let sensor_membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();

        assert!(neuron_membrane_potential.is_none());
        assert!(sensor_membrane_potential.is_none());
    }
}
