//! Implementation of a spiking neural network

pub(crate) mod constant;
mod neuron;
pub use self::neuron::*;
use crate::*;

use slab::Slab;
use std::collections::HashMap;

/// A spiking neural network
#[derive(Debug, Default)]
pub struct SpikingNeuralNetwork {
    neurons: Slab<SpikingNeuron>,
    neuron_handles: Vec<Handle>,
    incoming_connections: HashMap<Handle, Vec<(Handle, Weight)>>,
}

impl NeuralNetwork for SpikingNeuralNetwork {
    /// Update the state of all neurons
    fn step(
        &mut self,
        time_since_last_step: Milliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    ) {
        self.update_neurons_connected_to_external_inputs(time_since_last_step, external_inputs);
        self.update_neurons_not_connected_to_external_inputs(time_since_last_step, external_inputs);
    }

    /// Returns the last calculated state of the neuron referenced by `handle`
    fn membrane_potential_of_neuron(&self, neuron: Handle) -> Result<Option<MembranePotential>> {
        self.neurons
            .get(neuron.0)
            .ok_or(())
            .map(|neuron| neuron.membrane_potential())
    }

    /// Add a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle {
        let handle = Handle(self.neurons.insert(SpikingNeuron::default()));
        self.neuron_handles.push(handle);
        handle
    }

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<()> {
        let is_origin_same_as_destination = connection.from == connection.to;
        let valid_origin = self.neurons.contains(connection.from.0);
        let valid_destination = self.neurons.contains(connection.to.0);
        if is_origin_same_as_destination || !valid_origin || !valid_destination {
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
        self.incoming_connections
            .get(&neuron_handle)
            .map(|incoming_connections| {
                incoming_connections
                    .iter()
                    .filter_map(|(handle_of_connection, weight)| {
                        self.membrane_potential_of_neuron(*handle_of_connection)
                            .expect(
                                "Internal error: Stored connection handle does not correspond to \
                                 any neuron",
                            )
                            .map(|state_of_connection| (state_of_connection, *weight))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn update_neurons_connected_to_external_inputs(
        &mut self,
        time_since_last_step: Milliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    ) {
        for (&handle_of_neuron_receiving_input, &input) in external_inputs {
            let mut inputs =
                self.cached_incoming_connection_inputs(handle_of_neuron_receiving_input);
            const EXTERNAL_CONNECTION_WEIGHT: Weight = 1.0;
            inputs.push((input, EXTERNAL_CONNECTION_WEIGHT));

            let neuron = self
                .neurons
                .get_mut(handle_of_neuron_receiving_input.0)
                .ok_or(())
                .unwrap();
            neuron.step(time_since_last_step, &inputs);
        }
    }

    fn update_neurons_not_connected_to_external_inputs(
        &mut self,
        time_since_last_step: Milliseconds,
        external_inputs: &HashMap<Handle, MembranePotential>,
    ) {
        for &neuron_handle in self
            .neuron_handles
            .iter()
            .filter(|handle| !external_inputs.contains_key(handle))
        {
            let inputs = self.cached_incoming_connection_inputs(neuron_handle);
            let neuron = self.neurons.get_mut(neuron_handle.0).ok_or(()).unwrap();
            neuron.step(time_since_last_step, &inputs);
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
            weight: 1.0,
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
            weight: 1.0,
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
            weight: 1.0,
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
            weight: 1.0,
        };
        let result = neural_network.add_connection(connection);
        assert!(result.is_ok());
    }

    #[test]
    fn step_works_on_empty_network() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let elapsed_time = 1.0;
        let inputs = HashMap::new();
        neural_network.step(elapsed_time, &inputs);
    }

    #[test]
    fn step_on_unconnected_neurons_emits_no_potential() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();
        let neuron_handle = neural_network.push_neuron();

        let elapsed_time = 1.0;
        let inputs = HashMap::new();
        neural_network.step(elapsed_time, &inputs);

        let sensor_membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        let neuron_membrane_potential = neural_network
            .membrane_potential_of_neuron(neuron_handle)
            .unwrap();

        assert!(sensor_membrane_potential.is_none());
        assert!(neuron_membrane_potential.is_none());
    }

    #[test]
    fn connected_neurons_with_no_input_do_not_fire() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(sensor_handle.0),
            to: Handle(neuron_handle.0),
            weight: 1.0,
        };
        neural_network.add_connection(connection).unwrap();

        let elapsed_time = 1.0;
        let inputs = HashMap::new();

        for _ in 0..100 {
            neural_network.step(elapsed_time, &inputs);

            let sensor_membrane_potential = neural_network
                .membrane_potential_of_neuron(sensor_handle)
                .unwrap();
            let neuron_membrane_potential = neural_network
                .membrane_potential_of_neuron(neuron_handle)
                .unwrap();

            assert!(sensor_membrane_potential.is_none());
            assert!(neuron_membrane_potential.is_none());
        }
    }

    #[test]
    fn high_input_causes_firing() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();

        let elapsed_time = 1.0;
        let inputs = hashmap! {
            sensor_handle => self::constant::THRESHOLD_POTENTIAL
        };
        neural_network.step(elapsed_time, &inputs);

        let sensor_membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        assert!(sensor_membrane_potential.is_some());
    }

    #[test]
    fn weak_connection_does_not_propagate_firing() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(sensor_handle.0),
            to: Handle(neuron_handle.0),
            weight: 0.1,
        };
        neural_network.add_connection(connection).unwrap();

        let elapsed_time = 1.0;
        let inputs = hashmap! {
            sensor_handle => self::constant::THRESHOLD_POTENTIAL
        };
        neural_network.step(elapsed_time, &inputs);

        let sensor_membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        let neuron_membrane_potential = neural_network
            .membrane_potential_of_neuron(neuron_handle)
            .unwrap();

        assert!(sensor_membrane_potential.is_some());
        assert!(neuron_membrane_potential.is_none());
    }

    #[test]
    fn strong_connection_propagates_firing() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();
        let neuron_handle = neural_network.push_neuron();
        let connection = Connection {
            from: Handle(sensor_handle.0),
            to: Handle(neuron_handle.0),
            weight: 1.0,
        };
        neural_network.add_connection(connection).unwrap();

        let elapsed_time = 1.0;
        let inputs = hashmap! {
            sensor_handle => self::constant::THRESHOLD_POTENTIAL
        };
        neural_network.step(elapsed_time, &inputs);

        let sensor_membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        let neuron_membrane_potential = neural_network
            .membrane_potential_of_neuron(neuron_handle)
            .unwrap();

        assert!(sensor_membrane_potential.is_some());
        assert!(neuron_membrane_potential.is_some());
    }

    #[test]
    fn spike_ends_after_many_small_time_steps() {
        let mut neural_network = SpikingNeuralNetwork::default();
        let sensor_handle = neural_network.push_neuron();

        const SMALL_TIMESTEP: Milliseconds = 0.001;
        let steps = f64::ceil(constant::SPIKE_DURATION / SMALL_TIMESTEP) as u32;

        let inputs = hashmap! {
            sensor_handle => self::constant::THRESHOLD_POTENTIAL
        };
        neural_network.step(SMALL_TIMESTEP, &inputs);

        let no_inputs = HashMap::new();
        for _ in 0..steps {
            neural_network.step(SMALL_TIMESTEP, &no_inputs);
        }

        let sensor_membrane_potential = neural_network
            .membrane_potential_of_neuron(sensor_handle)
            .unwrap();
        assert!(sensor_membrane_potential.is_none());;
    }
}
