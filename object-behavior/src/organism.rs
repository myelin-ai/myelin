//! Behavior of an organism that can interact with its surroundings

use maplit::hashmap;
use myelin_engine::prelude::*;
use myelin_genetics::{
    DevelopedNeuralNetwork, Genome, NeuralNetworkDeveloper, NeuralNetworkDevelopmentConfiguration,
};
use myelin_neural_network::{Handle, MembranePotential, Milliseconds, NeuralNetwork};
use std::f64::consts::PI;

/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
#[derive(Debug, Clone)]
pub struct OrganismBehavior {
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
        /// 1. Rotation
        const INPUT_NEURON_COUNT: u32 = 1;

        /// 1. Linear force x
        /// 2. Linear force y
        /// 3. Torque
        const OUTPUT_NEURON_COUNT: u32 = 3;

        let metadata = NeuralNetworkDevelopmentConfiguration {
            parent_genomes,
            input_neuron_count: INPUT_NEURON_COUNT,
            output_neuron_count: OUTPUT_NEURON_COUNT,
        };

        Self {
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
        let input_neurons = &self.developed_neural_network.input_neuron_handles;
        let output_neurons = &self.developed_neural_network.output_neuron_handles;
        let neural_network = &mut self.developed_neural_network.neural_network;

        let rotation_input_neuron = *input_neurons.get(0).expect("Neuron not found in network");

        let inputs = hashmap! {
            rotation_input_neuron => own_description.rotation.value() / PI - 1.0,
        };

        neural_network.step(
            world_interactor.elapsed_time_in_update().as_millis() as Milliseconds,
            &inputs,
        );

        let linear_force_x_output_neuron = get_neuron_handle(0, output_neurons);
        let linear_force_y_output_neuron = get_neuron_handle(1, output_neurons);
        let torque_output_neuron = get_neuron_handle(2, output_neurons);

        let linear_force_x = get_membrane_potential(linear_force_x_output_neuron, neural_network);
        let linear_force_y = get_membrane_potential(linear_force_y_output_neuron, neural_network);
        let torque = get_membrane_potential(torque_output_neuron, neural_network);

        linear_force_x.or(linear_force_y).or(torque).map(|_| {
            Action::ApplyForce(Force {
                linear: Vector {
                    x: linear_force_x.unwrap_or_default(),
                    y: linear_force_y.unwrap_or_default(),
                },
                torque: Torque(torque.unwrap_or_default()),
            })
        })
    }
}

fn get_neuron_handle(index: usize, handles: &[Handle]) -> Handle {
    *handles.get(index).expect("Neuron not found in network")
}

fn get_membrane_potential(
    neuron: Handle,
    neural_network: &dyn NeuralNetwork,
) -> Option<MembranePotential> {
    neural_network
        .membrane_potential_of_neuron(neuron)
        .expect("Invalid neuron handle")
}
