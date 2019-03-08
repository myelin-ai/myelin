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
        /// 1. Average linear acceleration since last step (forward)
        /// 2. Average linear acceleration since last step (backward)
        /// 3. Average angular acceleration since last step (clockwise)
        /// 4. Average angular acceleration since last step (counterclockwise)
        const INPUT_NEURON_COUNT: usize = 4;

        /// 1. linear force (forward)
        /// 2. linear force (backward)
        /// 3. angular force (clockwise)
        /// 4. angular force (counterclockwise)
        const OUTPUT_NEURON_COUNT: usize = 4;

        let configuration = NeuralNetworkDevelopmentConfiguration {
            parent_genomes,
            input_neuron_count: INPUT_NEURON_COUNT,
            output_neuron_count: OUTPUT_NEURON_COUNT,
        };

        Self {
            previous_velocity: Vector::default(),
            developed_neural_network: neural_network_developer
                .develop_neural_network(&configuration),
            neural_network_developer,
        }
    }
}

impl ObjectBehavior for OrganismBehavior {
    fn step(&mut self, world_interactor: &dyn WorldInteractor) -> Option<Action> {
        let elapsed_time = world_interactor.elapsed_time_in_update().as_millis() as Milliseconds;
        let own_object = world_interactor.own_object();

        let neuron_handle_mapping = map_handles(&self.developed_neural_network);

        let current_velocity = velocity(&own_object.description);
        let absolute_acceleration = (current_velocity - self.previous_velocity) / elapsed_time;
        let relative_acceleration = absolute_acceleration.rotate(own_object.description.rotation);

        self.previous_velocity = current_velocity;

        let mut inputs = HashMap::with_capacity(2);
        add_acceleration_inputs(
            relative_acceleration,
            neuron_handle_mapping.input,
            |key, value| {
                inputs.insert(key, value);
            },
        );

        let neural_network = &mut self.developed_neural_network.neural_network;
        neural_network.step(
            world_interactor.elapsed_time_in_update().as_millis() as Milliseconds,
            &inputs,
        );

        let linear_force = get_combined_potential(
            neuron_handle_mapping.output.linear_acceleration.forward,
            neuron_handle_mapping.output.linear_acceleration.backward,
            neural_network.as_ref(),
        );

        let angular_force = get_combined_potential(
            neuron_handle_mapping.output.angular_acceleration.clockwise,
            neuron_handle_mapping
                .output
                .angular_acceleration
                .counterclockwise,
            neural_network.as_ref(),
        );

        if linear_force != 0.0 || angular_force != 0.0 {
            Some(Action::ApplyForce(Force {
                // Todo: Translate forward linear force to global linear force
                linear: Vector {
                    x: linear_force,  // Wrong value
                    y: -linear_force, // Wrong value
                },
                torque: Torque(angular_force),
            }))
        } else {
            None
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn velocity(object_description: &ObjectDescription) -> Vector {
    match object_description.mobility {
        Mobility::Immovable => Vector::default(),
        Mobility::Movable(velocity) => velocity,
    }
}

fn add_acceleration_inputs(
    acceleration: Vector,
    input_neuron_handle_mapping: InputNeuronHandleMapping,
    mut add_input_fn: impl FnMut(Handle, f64),
) {
    const MAX_ACCELERATION_FORCE: f64 = 5.0 * 9.81;

    add_input_fn(
        linear_acceleration_handle(
            acceleration.x,
            input_neuron_handle_mapping.linear_acceleration,
        ),
        acceleration.x.min(MAX_ACCELERATION_FORCE) / MAX_ACCELERATION_FORCE,
    );

    add_input_fn(
        angular_acceleration_handle(
            acceleration.y,
            input_neuron_handle_mapping.angular_acceleration,
        ),
        acceleration.y.min(MAX_ACCELERATION_FORCE) / MAX_ACCELERATION_FORCE,
    );
}

fn linear_acceleration_handle(
    linear_acceleration: f64,
    linear_acceleration_handle_mapping: LinearAccelerationHandleMapping,
) -> Handle {
    if linear_acceleration >= 0.0 {
        linear_acceleration_handle_mapping.forward
    } else {
        linear_acceleration_handle_mapping.backward
    }
}

fn angular_acceleration_handle(
    angular_acceleration: f64,
    angular_acceleration_handle_mapping: AngularAccelerationHandleMapping,
) -> Handle {
    if angular_acceleration >= 0.0 {
        angular_acceleration_handle_mapping.clockwise
    } else {
        angular_acceleration_handle_mapping.counterclockwise
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct NeuronHandleMapping {
    input: InputNeuronHandleMapping,
    output: OutputNeuronHandleMapping,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct InputNeuronHandleMapping {
    linear_acceleration: LinearAccelerationHandleMapping,
    angular_acceleration: AngularAccelerationHandleMapping,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct LinearAccelerationHandleMapping {
    forward: Handle,
    backward: Handle,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct AngularAccelerationHandleMapping {
    counterclockwise: Handle,
    clockwise: Handle,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct OutputNeuronHandleMapping {
    linear_acceleration: LinearAccelerationHandleMapping,
    angular_acceleration: AngularAccelerationHandleMapping,
}

fn map_handles(developed_neural_network: &DevelopedNeuralNetwork) -> NeuronHandleMapping {
    let input_neurons = &developed_neural_network.input_neuron_handles;
    let output_neurons = &developed_neural_network.output_neuron_handles;

    NeuronHandleMapping {
        input: InputNeuronHandleMapping {
            linear_acceleration: LinearAccelerationHandleMapping {
                forward: get_neuron_handle(input_neurons, 0),
                backward: get_neuron_handle(input_neurons, 1),
            },
            angular_acceleration: AngularAccelerationHandleMapping {
                counterclockwise: get_neuron_handle(input_neurons, 2),
                clockwise: get_neuron_handle(input_neurons, 3),
            },
        },
        output: OutputNeuronHandleMapping {
            linear_acceleration: LinearAccelerationHandleMapping {
                forward: get_neuron_handle(output_neurons, 0),
                backward: get_neuron_handle(output_neurons, 1),
            },
            angular_acceleration: AngularAccelerationHandleMapping {
                counterclockwise: get_neuron_handle(output_neurons, 2),
                clockwise: get_neuron_handle(output_neurons, 3),
            },
        },
    }
}

fn get_neuron_handle(handles: &[Handle], index: usize) -> Handle {
    *handles.get(index).expect("Neuron not found in network")
}

fn get_normalized_potential(neuron: Handle, neural_network: &dyn NeuralNetwork) -> f64 {
    neural_network
        .normalized_potential_of_neuron(neuron)
        .expect("Invalid neuron handle")
}

fn get_combined_potential(
    positive_neuron: Handle,
    negative_neuron: Handle,
    neural_network: &dyn NeuralNetwork,
) -> f64 {
    get_normalized_potential(positive_neuron, neural_network)
        + get_normalized_potential(negative_neuron, neural_network)
}
