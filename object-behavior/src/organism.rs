//! Behavior of an organism that can interact with its surroundings

use myelin_engine::prelude::*;
use myelin_genetics::{
    DevelopedNeuralNetwork, Genome, NeuralNetworkDeveloper, NeuralNetworkDevelopmentConfiguration,
};
use myelin_neural_network::{Handle, Milliseconds, NeuralNetwork};
use std::any::Any;
use std::collections::HashMap;

/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
#[derive(Debug, Clone)]
pub struct OrganismBehavior {
    previous_velocity: Vector,
    developed_neural_network: DevelopedNeuralNetwork,
    neural_network_developer: Box<dyn NeuralNetworkDeveloper>,
}

impl OrganismBehavior {
    /// Create a new `OrganismBehavior` from a pair of parent [`Genome`]s.
    /// The [`NeuralNetworkDeveloper`] is used to create this organism's [`NeuralNetwork`]
    /// and its eventual offspring.
    ///
    /// [`Genome`]: ../myelin-genetics/struct.Genome.html
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    pub fn new(
        parent_genomes: (Genome, Genome),
        neural_network_developer: Box<dyn NeuralNetworkDeveloper>,
    ) -> Self {
        /// 1. Average acceleration since last step (x in positive direction)
        /// 2. Average acceleration since last step (x in negative direction)
        /// 3. Average acceleration since last step (y in positive direction)
        /// 4. Average acceleration since last step (y in negative direction)
        const INPUT_NEURON_COUNT: u32 = 4;

        /// 1. Linear force x in positive direction
        /// 2. Linear force x in negative direction
        /// 3. Linear force y in positive direction
        /// 4. Linear force y in negative direction
        /// 5. Torque in positive direction
        /// 6. Torque in negative direction
        const OUTPUT_NEURON_COUNT: u32 = 6;

        let metadata = NeuralNetworkDevelopmentConfiguration {
            parent_genomes,
            input_neuron_count: INPUT_NEURON_COUNT,
            output_neuron_count: OUTPUT_NEURON_COUNT,
        };

        Self {
            previous_velocity: Vector::default(),
            developed_neural_network: neural_network_developer.develop_neural_network(metadata),
            neural_network_developer,
        }
    }
}

impl ObjectBehavior for OrganismBehavior {
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        world_interactor: &dyn WorldInteractor,
    ) -> Option<Action> {
        let elapsed_time = world_interactor.elapsed_time_in_update().as_millis() as Milliseconds;

        let input_neurons = &self.developed_neural_network.input_neuron_handles;
        let output_neurons = &self.developed_neural_network.output_neuron_handles;
        let neural_network = &mut self.developed_neural_network.neural_network;

        let acceleration_x_positive_input_neuron = get_neuron_handle(0, input_neurons);
        let acceleration_x_negative_input_neuron = get_neuron_handle(1, input_neurons);
        let acceleration_y_positive_input_neuron = get_neuron_handle(2, input_neurons);
        let acceleration_y_negative_input_neuron = get_neuron_handle(3, input_neurons);

        let current_velocity = match own_description.mobility {
            Mobility::Immovable => Vector::default(),
            Mobility::Movable(velocity) => velocity,
        };

        // TODO: Implement `scale` in geometry
        let acceleration_x = (current_velocity.x - self.previous_velocity.x) / elapsed_time;
        let acceleration_y = (current_velocity.y - self.previous_velocity.y) / elapsed_time;

        let mut inputs = HashMap::with_capacity(2);

        inputs.insert(
            if acceleration_x >= 0.0 {
                acceleration_x_positive_input_neuron
            } else {
                acceleration_x_negative_input_neuron
            },
            acceleration_x,
        );

        inputs.insert(
            if acceleration_y >= 0.0 {
                acceleration_y_positive_input_neuron
            } else {
                acceleration_y_negative_input_neuron
            },
            acceleration_y,
        );

        self.previous_velocity = current_velocity;

        neural_network.step(
            world_interactor.elapsed_time_in_update().as_millis() as Milliseconds,
            &inputs,
        );

        let linear_force_x_positive_output_neuron = get_neuron_handle(0, output_neurons);
        let linear_force_x_negative_output_neuron = get_neuron_handle(1, output_neurons);
        let linear_force_y_positive_output_neuron = get_neuron_handle(2, output_neurons);
        let linear_force_y_negative_output_neuron = get_neuron_handle(3, output_neurons);
        let torque_positive_output_neuron = get_neuron_handle(4, output_neurons);
        let torque_negative_output_neuron = get_neuron_handle(5, output_neurons);

        let linear_force_x = get_normalized_potential(
            linear_force_x_positive_output_neuron,
            neural_network.as_ref(),
        ) + get_normalized_potential(
            linear_force_x_negative_output_neuron,
            neural_network.as_ref(),
        );

        let linear_force_y = get_normalized_potential(
            linear_force_y_positive_output_neuron,
            neural_network.as_ref(),
        ) + get_normalized_potential(
            linear_force_y_negative_output_neuron,
            neural_network.as_ref(),
        );

        let torque =
            get_normalized_potential(torque_positive_output_neuron, neural_network.as_ref())
                + get_normalized_potential(torque_negative_output_neuron, neural_network.as_ref());

        if linear_force_x != 0.0 || linear_force_y != 0.0 || torque != 0.0 {
            Some(Action::ApplyForce(Force {
                linear: Vector {
                    x: linear_force_x,
                    y: linear_force_y,
                },
                torque: Torque(torque),
            }))
        } else {
            None
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn get_neuron_handle(index: usize, handles: &[Handle]) -> Handle {
    *handles.get(index).expect("Neuron not found in network")
}

fn get_normalized_potential(neuron: Handle, neural_network: &dyn NeuralNetwork) -> f64 {
    neural_network
        .normalized_potential_of_neuron(neuron)
        .expect("Invalid neuron handle")
}
