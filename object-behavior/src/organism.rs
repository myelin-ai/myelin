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
        /// 1. Average axial acceleration since last step (forward)
        /// 2. Average axial acceleration since last step (backward)
        /// 3. Average lateral acceleration since last step (left)
        /// 4. Average lateral acceleration since last step (right)
        const INPUT_NEURON_COUNT: usize = 4;

        /// 1. axial force (forward)
        /// 2. axial force (backward)
        /// 3. lateral force (left)
        /// 4. lateral force (right)
        /// 5. torque (counterclockwise)
        /// 6. torque (clockwise)
        const OUTPUT_NEURON_COUNT: usize = 6;

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
        let relative_acceleration = absolute_acceleration.rotate(own_object.description.rotation); // TODO: This is probably wrong

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

        let axial_force = get_combined_potential(
            neuron_handle_mapping.output.axial_acceleration.forward,
            neuron_handle_mapping.output.axial_acceleration.backward,
            neural_network.as_ref(),
        );

        let lateral_force = get_combined_potential(
            neuron_handle_mapping.output.lateral_acceleration.right,
            neuron_handle_mapping.output.lateral_acceleration.left,
            neural_network.as_ref(),
        );

        let torque = get_combined_potential(
            neuron_handle_mapping.output.torque.counterclockwise,
            neuron_handle_mapping.output.torque.clockwise,
            neural_network.as_ref(),
        );

        if axial_force != 0.0 || angular_force != 0.0 {
            Some(Action::ApplyForce(Force {
                // Todo: Translate linear force to global values
                linear: Vector {
                    x: axial_force,
                    y: lateral_force,
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
        axial_acceleration_handle(
            acceleration.x,
            input_neuron_handle_mapping.axial_acceleration,
        ),
        acceleration.x.min(MAX_ACCELERATION_FORCE) / MAX_ACCELERATION_FORCE,
    );

    add_input_fn(
        lateral_acceleration_handle(
            acceleration.y,
            input_neuron_handle_mapping.lateral_acceleration,
        ),
        acceleration.y.min(MAX_ACCELERATION_FORCE) / MAX_ACCELERATION_FORCE,
    );
}

fn axial_acceleration_handle(
    axial_acceleration: f64,
    axial_acceleration_handle_mapping: AxialAccelerationHandleMapping,
) -> Handle {
    if axial_acceleration >= 0.0 {
        axial_acceleration_handle_mapping.forward
    } else {
        axial_acceleration_handle_mapping.backward
    }
}

fn lateral_acceleration_handle(
    lateral_acceleration: f64,
    lateral_acceleration_handle_mapping: LateralAccelerationHandleMapping,
) -> Handle {
    if lateral_acceleration <= 0.0 {
        lateral_acceleration_handle_mapping.left
    } else {
        lateral_acceleration_handle_mapping.right
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct NeuronHandleMapping {
    input: InputNeuronHandleMapping,
    output: OutputNeuronHandleMapping,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct InputNeuronHandleMapping {
    axial_acceleration: AxialAccelerationHandleMapping,
    lateral_acceleration: LateralAccelerationHandleMapping,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct OutputNeuronHandleMapping {
    axial_acceleration: AxialAccelerationHandleMapping,
    lateral_acceleration: LateralAccelerationHandleMapping,
    torque: TorqueHandleMapping,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct AxialAccelerationHandleMapping {
    forward: Handle,
    backward: Handle,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct LateralAccelerationHandleMapping {
    left: Handle,
    right: Handle,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TorqueHandleMapping {
    counterclockwise: Handle,
    clockwise: Handle,
}

fn map_handles(developed_neural_network: &DevelopedNeuralNetwork) -> NeuronHandleMapping {
    let input_neurons = &developed_neural_network.input_neuron_handles;
    let output_neurons = &developed_neural_network.output_neuron_handles;

    NeuronHandleMapping {
        input: InputNeuronHandleMapping {
            axial_acceleration: AxialAccelerationHandleMapping {
                forward: get_neuron_handle(input_neurons, 0),
                backward: get_neuron_handle(input_neurons, 1),
            },
            lateral_acceleration: LateralAccelerationHandleMapping {
                left: get_neuron_handle(input_neurons, 2),
                right: get_neuron_handle(input_neurons, 3),
            },
        },
        output: OutputNeuronHandleMapping {
            axial_acceleration: AxialAccelerationHandleMapping {
                forward: get_neuron_handle(output_neurons, 0),
                backward: get_neuron_handle(output_neurons, 1),
            },
            lateral_acceleration: LateralAccelerationHandleMapping {
                left: get_neuron_handle(output_neurons, 2),
                right: get_neuron_handle(output_neurons, 3),
            },
            torque: TorqueHandleMapping {
                counterclockwise: get_neuron_handle(output_neurons, 4),
                clockwise: get_neuron_handle(output_neurons, 5),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axial_acceleration_handle_returns_correct_handle_for_minus_one() {
        let mapping = AxialAccelerationHandleMapping {
            forward: Handle(0),
            backward: Handle(1),
        };

        let handle = axial_acceleration_handle(-1.0, mapping);

        assert_eq!(Handle(1), handle);
    }

    #[test]
    fn axial_acceleration_handle_returns_correct_handle_for_zero() {
        let mapping = AxialAccelerationHandleMapping {
            forward: Handle(0),
            backward: Handle(1),
        };

        let handle = axial_acceleration_handle(0.0, mapping);

        assert_eq!(Handle(0), handle);
    }

    #[test]
    fn axial_acceleration_handle_returns_correct_handle_for_one() {
        let mapping = AxialAccelerationHandleMapping {
            forward: Handle(0),
            backward: Handle(1),
        };

        let handle = axial_acceleration_handle(1.0, mapping);

        assert_eq!(Handle(0), handle);
    }

    #[test]
    fn lateral_acceleration_handle_returns_correct_handle_for_minus_one() {
        let mapping = LateralAccelerationHandleMapping {
            left: Handle(0),
            right: Handle(1),
        };

        let handle = lateral_acceleration_handle(-1.0, mapping);

        assert_eq!(Handle(0), handle);
    }

    #[test]
    fn lateral_acceleration_handle_returns_correct_handle_for_zero() {
        let mapping = LateralAccelerationHandleMapping {
            left: Handle(0),
            right: Handle(1),
        };

        let handle = lateral_acceleration_handle(0.0, mapping);

        assert_eq!(Handle(0), handle);
    }

    #[test]
    fn lateral_acceleration_handle_returns_correct_handle_for_one() {
        let mapping = LateralAccelerationHandleMapping {
            left: Handle(0),
            right: Handle(1),
        };

        let handle = lateral_acceleration_handle(1.0, mapping);

        assert_eq!(Handle(1), handle);
    }
}
