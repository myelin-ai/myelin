//! Behavior of an organism that can interact with its surroundings

use myelin_engine::prelude::*;
use myelin_genetics::{
    DevelopedNeuralNetwork, Genome, NeuralNetworkDeveloper, NeuralNetworkDevelopmentConfiguration,
};
use myelin_neural_network::{Handle, Milliseconds, NeuralNetwork};
use std::any::Any;
use std::collections::HashMap;

/// The hightest relative acceleration an organism can detect.
/// The value was chosen as many sources, [including Wikipedia](https://en.wikipedia.org/wiki/G-LOC#Thresholds) report
/// 5G as a typical threshold for the loss of consciousness in humans.
/// The origins of this number have not been verified.
const MAX_ACCELERATION: f64 = 5.0 * 9.81;

/// The highest possible force emmited by the organism.
/// Calculated as F = μma, where μ = 1, m = 20kg (hardcoded value in engine) and a = 9.8 m/s^2,
/// which is the [maximum acceleration a human can achieve](https://www.wired.com/2012/08/maximum-acceleration-in-the-100-m-dash/)
const MAX_ACCELERATION_FORCE: f64 = 20.0 * 9.8;

const MAX_ANGULAR_FORCE: f64 = MAX_ACCELERATION_FORCE;

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
        let relative_acceleration =
            absolute_acceleration.rotate_clockwise(own_object.description.rotation);

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

        convert_neural_network_output_to_action(
            neuron_handle_mapping,
            neural_network.as_ref(),
            &own_object.description,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn convert_neural_network_output_to_action(
    neuron_handle_mapping: NeuronHandleMapping,
    neural_network: &dyn NeuralNetwork,
    object_description: &ObjectDescription,
) -> Option<Action> {
    let axial_force = get_combined_potential(
        neuron_handle_mapping.output.axial_acceleration.forward,
        neuron_handle_mapping.output.axial_acceleration.backward,
        neural_network,
    );

    let lateral_force = get_combined_potential(
        neuron_handle_mapping.output.lateral_acceleration.right,
        neuron_handle_mapping.output.lateral_acceleration.left,
        neural_network,
    );

    let angular_acceleration_force = get_combined_potential(
        neuron_handle_mapping.output.torque.counterclockwise,
        neuron_handle_mapping.output.torque.clockwise,
        neural_network,
    );

    let aabb = object_description.shape.aabb();
    let width = aabb.lower_right.y - aabb.upper_left.y;

    let position_vector = Vector {
        x: 0.0,
        y: width / 2.0,
    }
    .rotate(object_description.rotation);

    let angular_force = angular_acceleration_force
        .map(|force| position_vector.normal().unit() * MAX_ANGULAR_FORCE * force);

    let torque = angular_force.map(|force| position_vector.cross_product(force));

    if axial_force.is_some() || lateral_force.is_some() || torque.is_some() {
        let relative_linear_force = Vector {
            x: axial_force.unwrap_or_default(),
            y: lateral_force.unwrap_or_default(),
        };
        let global_linear_force = relative_linear_force.rotate(object_description.rotation);
        let scaled_linear_force = global_linear_force * MAX_ACCELERATION_FORCE;
        Some(Action::ApplyForce(Force {
            linear: scaled_linear_force,
            torque: Torque(torque.unwrap_or_default()),
        }))
    } else {
        None
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
    add_input_fn(
        axial_acceleration_handle(
            acceleration.x,
            input_neuron_handle_mapping.axial_acceleration,
        ),
        acceleration.x.abs().min(MAX_ACCELERATION) / MAX_ACCELERATION,
    );

    add_input_fn(
        lateral_acceleration_handle(
            acceleration.y,
            input_neuron_handle_mapping.lateral_acceleration,
        ),
        acceleration.y.abs().min(MAX_ACCELERATION) / MAX_ACCELERATION,
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

fn get_normalized_potential(neuron: Handle, neural_network: &dyn NeuralNetwork) -> Option<f64> {
    neural_network
        .normalized_potential_of_neuron(neuron)
        .expect("Invalid neuron handle")
}

fn get_combined_potential(
    positive_neuron: Handle,
    negative_neuron: Handle,
    neural_network: &dyn NeuralNetwork,
) -> Option<f64> {
    let positive_potential = get_normalized_potential(positive_neuron, neural_network);
    let negative_potential = get_normalized_potential(negative_neuron, neural_network);

    match positive_potential {
        Some(positive_potential) => match negative_potential {
            Some(negative_potential) => Some(positive_potential - negative_potential),
            None => Some(positive_potential),
        },
        None => match negative_potential {
            Some(negative_potential) => Some(-negative_potential),
            None => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use myelin_neural_network::NeuralNetworkMock;
    use nearly_eq::assert_nearly_eq;
    use std::f64::consts::PI;

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

    // Axial expected handles: (0 forward, 1 backward)
    // Lateral expected handles: (2 left, 3 right)
    struct AddAccelerationInputsTestConfiguration {
        input_acceleration: Vector,
        axial_expected_value: (Handle, f64),
        lateral_expected_value: (Handle, f64),
    }

    fn add_acceleration_inputs_test(configuration: AddAccelerationInputsTestConfiguration) {
        let mapping = InputNeuronHandleMapping {
            axial_acceleration: AxialAccelerationHandleMapping {
                forward: Handle(0),
                backward: Handle(1),
            },
            lateral_acceleration: LateralAccelerationHandleMapping {
                left: Handle(2),
                right: Handle(3),
            },
        };

        let mut values = HashMap::with_capacity(2);
        add_acceleration_inputs(
            configuration.input_acceleration,
            mapping,
            |handle, value| {
                values.insert(handle, value);
            },
        );

        assert_eq!(2, values.len());
        assert_nearly_eq!(
            configuration.axial_expected_value.1,
            *values
                .get(&configuration.axial_expected_value.0)
                .expect("Axial input was None")
        );
        assert_nearly_eq!(
            configuration.lateral_expected_value.1,
            *values
                .get(&configuration.lateral_expected_value.0)
                .expect("Lateral input was None")
        );
    }

    #[test]
    fn add_acceleration_inputs_with_no_acceleration() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector { x: 0.0, y: 0.0 },
            axial_expected_value: (Handle(0), 0.0),
            lateral_expected_value: (Handle(0), 0.0),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn add_acceleration_inputs_with_forward_acceleration() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector {
                x: MAX_ACCELERATION / 5.0,
                y: 0.0,
            },
            axial_expected_value: (Handle(0), 0.2),
            lateral_expected_value: (Handle(2), 0.0),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn add_acceleration_inputs_with_backward_acceleration() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector {
                x: -MAX_ACCELERATION / 5.0,
                y: 0.0,
            },
            axial_expected_value: (Handle(1), 0.2),
            lateral_expected_value: (Handle(2), 0.0),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn add_acceleration_inputs_with_lateral_acceleration_to_the_left() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector {
                x: 0.0,
                y: -MAX_ACCELERATION / 5.0,
            },
            axial_expected_value: (Handle(0), 0.0),
            lateral_expected_value: (Handle(2), 0.2),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn add_acceleration_inputs_with_lateral_acceleration_to_the_right() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector {
                x: 0.0,
                y: MAX_ACCELERATION / 5.0,
            },
            axial_expected_value: (Handle(0), 0.0),
            lateral_expected_value: (Handle(3), 0.2),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn add_acceleration_inputs_with_too_fast_forward_acceleration() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector {
                x: MAX_ACCELERATION * 5.0,
                y: 0.0,
            },
            axial_expected_value: (Handle(0), 1.0),
            lateral_expected_value: (Handle(2), 0.0),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn add_acceleration_inputs_with_too_fast_backward_acceleration() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector {
                x: -MAX_ACCELERATION * 5.0,
                y: 0.0,
            },
            axial_expected_value: (Handle(1), 1.0),
            lateral_expected_value: (Handle(2), 0.0),
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn convert_neural_network_output_to_action_test() {
        let mapping = NeuronHandleMapping {
            input: InputNeuronHandleMapping {
                axial_acceleration: AxialAccelerationHandleMapping {
                    forward: Handle(0),
                    backward: Handle(1),
                },
                lateral_acceleration: LateralAccelerationHandleMapping {
                    left: Handle(2),
                    right: Handle(3),
                },
            },
            output: OutputNeuronHandleMapping {
                axial_acceleration: AxialAccelerationHandleMapping {
                    forward: Handle(4),
                    backward: Handle(5),
                },
                lateral_acceleration: LateralAccelerationHandleMapping {
                    left: Handle(6),
                    right: Handle(7),
                },
                torque: TorqueHandleMapping {
                    counterclockwise: Handle(8),
                    clockwise: Handle(9),
                },
            },
        };

        let mut network = NeuralNetworkMock::new();
        network
            .expect_normalized_potential_of_neuron(partial_eq(
                mapping.output.axial_acceleration.forward,
            ))
            .returns(Ok(Some(0.5)));
        network
            .expect_normalized_potential_of_neuron(partial_eq(
                mapping.output.axial_acceleration.backward,
            ))
            .returns(Ok(Some(0.0)));
        network
            .expect_normalized_potential_of_neuron(partial_eq(
                mapping.output.lateral_acceleration.left,
            ))
            .returns(Ok(Some(0.2)));
        network
            .expect_normalized_potential_of_neuron(partial_eq(
                mapping.output.lateral_acceleration.right,
            ))
            .returns(Ok(Some(0.0)));
        network
            .expect_normalized_potential_of_neuron(partial_eq(
                mapping.output.torque.counterclockwise,
            ))
            .returns(Ok(Some(0.0)));
        network
            .expect_normalized_potential_of_neuron(partial_eq(mapping.output.torque.clockwise))
            .returns(Ok(Some(0.4)));

        let object_description = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 10.0)
                    .vertex(10.0, 10.0)
                    .vertex(10.0, 0.0)
                    .build()
                    .unwrap(),
            )
            .mobility(Mobility::Movable(Vector::default()))
            .location(0.0, 0.0)
            .rotation(Radians::try_new(PI).unwrap())
            .build()
            .unwrap();

        let expected_force = Force {
            linear: Vector {
                x: -MAX_ACCELERATION_FORCE * 0.5,
                y: MAX_ACCELERATION_FORCE * 0.2,
            },
            torque: Torque(-392.0),
        };

        let action =
            convert_neural_network_output_to_action(mapping, &network, &object_description)
                .unwrap();

        match action {
            Action::ApplyForce(force) => {
                assert_nearly_eq!(expected_force.linear.x, force.linear.x);
                assert_nearly_eq!(expected_force.linear.y, force.linear.y);
                assert_nearly_eq!(expected_force.torque.0, force.torque.0);
            }
            _ => panic!("Unexpected action"),
        }
    }
}
