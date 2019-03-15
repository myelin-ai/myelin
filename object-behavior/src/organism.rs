//! Behavior of an organism that can interact with its surroundings

use myelin_engine::prelude::*;
use myelin_genetics::{
    DevelopedNeuralNetwork, Genome, NeuralNetworkDeveloper, NeuralNetworkDevelopmentConfiguration,
};
use myelin_neural_network::{Handle, Milliseconds, NeuralNetwork};
use std::any::Any;
use std::collections::HashMap;
use std::f64::consts::PI;

/// The hightest relative acceleration an organism can detect.
/// The value was chosen as many sources, [including Wikipedia](https://en.wikipedia.org/wiki/G-LOC#Thresholds) report
/// 5G as a typical threshold for the loss of consciousness in humans.
/// The origins of this number have not been verified.
const MAX_ACCELERATION: f64 = 5.0 * 9.81;

/// The highest possible force emmited by the organism.
/// Calculated as F = μma, where μ = 1, m = 20kg (hardcoded value in engine) and a = 9.8 m/s^2,
/// which is the [maximum acceleration a human can achieve](https://www.wired.com/2012/08/maximum-acceleration-in-the-100-m-dash/)
const MAX_ACCELERATION_FORCE: f64 = 20.0 * 9.8;

const RAYCAST_COUNT: usize = 10;

const MAX_OBJECTS_PER_RAYCAST: usize = 3;

/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
#[derive(Debug, Clone)]
pub struct OrganismBehavior {
    previous_velocity: Vector,
    developed_neural_network: DevelopedNeuralNetwork,
    neural_network_developer: Box<dyn NeuralNetworkDeveloper>,
}

/// Distances to objects in FOV from right to left
const VISION_INPUT_COUNT: usize = RAYCAST_COUNT * MAX_OBJECTS_PER_RAYCAST;

/// 1. Average axial acceleration since last step (forward)
/// 2. Average axial acceleration since last step (backward)
/// 3. Average lateral acceleration since last step (left)
/// 4. Average lateral acceleration since last step (right)
const INPUT_NEURON_COUNT: usize = 4 + VISION_INPUT_COUNT;

const FIRST_VISION_INDEX: usize = INPUT_NEURON_COUNT - VISION_INPUT_COUNT + 1;

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
            &neuron_handle_mapping.input,
            |key, value| {
                inputs.insert(key, value);
            },
        );

        let objects_in_fov = objects_in_fov(&own_object.description, world_interactor);
        let vision_neuron_inputs =
            objects_in_fov_to_neuron_inputs(&own_object.description, &objects_in_fov);

        neuron_handle_mapping
            .input
            .vision
            .iter()
            .zip(vision_neuron_inputs.iter())
            .filter_map(|(handle, &input)| Some((handle, input?)))
            .for_each(|(handle, input)| {
                inputs.insert(*handle, input);
            });

        let neural_network = &mut self.developed_neural_network.neural_network;
        neural_network.step(
            world_interactor.elapsed_time_in_update().as_millis() as Milliseconds,
            &inputs,
        );

        convert_neural_network_output_to_action(
            neuron_handle_mapping,
            neural_network.as_ref(),
            &own_object,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn convert_neural_network_output_to_action(
    neuron_handle_mapping: NeuronHandleMapping,
    neural_network: &dyn NeuralNetwork,
    own_object: &Object<'_>,
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

    let torque = get_combined_potential(
        neuron_handle_mapping.output.torque.counterclockwise,
        neuron_handle_mapping.output.torque.clockwise,
        neural_network,
    );

    if !(axial_force == 0.0 && lateral_force == 0.0 && torque == 0.0) {
        let relative_linear_force = Vector {
            x: axial_force,
            y: lateral_force,
        };
        let global_linear_force = relative_linear_force.rotate(own_object.description.rotation);
        let scaled_linear_force = global_linear_force * MAX_ACCELERATION_FORCE;
        Some(Action::ApplyForce(Force {
            linear: scaled_linear_force,
            torque: Torque(torque),
        }))
    } else {
        None
    }
}

fn objects_in_fov<'a, 'b>(
    own_description: &'a ObjectDescription,
    world_interactor: &'b dyn WorldInteractor,
) -> Vec<Vec<Object<'b>>> {
    /// The angle in degrees describing the field of view. [Wikipedia](https://en.wikipedia.org/wiki/Human_eye#Field_of_view).
    const FOV_ANGLE: usize = 200;
    const ANGLE_PER_RAYCAST: f64 = FOV_ANGLE as f64 / RAYCAST_COUNT as f64;

    let unit_vector = Vector { x: 1.0, y: 0.0 };
    let own_direction = unit_vector.rotate(own_description.rotation);

    let half_of_fov_angle = degrees_to_radians(FOV_ANGLE as f64 / 2.0).unwrap();
    let rightmost_angle = own_direction.rotate_clockwise(half_of_fov_angle);
    (0..FOV_ANGLE)
        .map(|angle_step| {
            let angle_in_degrees = angle_step as f64 * ANGLE_PER_RAYCAST;
            let angle_in_radians = degrees_to_radians(angle_in_degrees).unwrap();
            let fov_direction = rightmost_angle.rotate(angle_in_radians);
            world_interactor
                .find_objects_in_ray(own_description.location, fov_direction)
                .into_iter()
                .take(MAX_OBJECTS_PER_RAYCAST)
                .collect()
        })
        .collect()
}

fn objects_in_fov_to_neuron_inputs(
    own_description: &ObjectDescription,
    objects: &[Vec<Object<'_>>],
) -> Vec<Option<f64>> {
    objects
        .iter()
        .map(|objects_in_ray| {
            let mut distances = objects_in_ray
                .iter()
                .map(|object| object.description.location - own_description.location)
                .map(Vector::from)
                .map(Vector::magnitude)
                .map(Some)
                .collect::<Vec<_>>();
            distances.sort_by(|a, b| a.partial_cmp(&b).unwrap());

            // Todo: Maybe move the whole Option stuff to another fn
            let not_visible_object_count = MAX_OBJECTS_PER_RAYCAST - distances.len();
            let mut not_visible_objects = vec![None; not_visible_object_count];
            distances.append(&mut not_visible_objects);

            distances
        })
        .flatten()
        .collect()
}

// To do: Move this to radians
fn degrees_to_radians(degrees: f64) -> Result<Radians, RadiansError> {
    Radians::try_new(degrees / 180.0 * PI)
}

fn velocity(object_description: &ObjectDescription) -> Vector {
    match object_description.mobility {
        Mobility::Immovable => Vector::default(),
        Mobility::Movable(velocity) => velocity,
    }
}

fn add_acceleration_inputs(
    acceleration: Vector,
    input_neuron_handle_mapping: &InputNeuronHandleMapping,
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct NeuronHandleMapping {
    input: InputNeuronHandleMapping,
    output: OutputNeuronHandleMapping,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct InputNeuronHandleMapping {
    axial_acceleration: AxialAccelerationHandleMapping,
    lateral_acceleration: LateralAccelerationHandleMapping,
    vision: Vec<Handle>,
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
            vision: (FIRST_VISION_INDEX..INPUT_NEURON_COUNT)
                .map(|index| get_neuron_handle(input_neurons, index))
                .collect(),
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
    use nearly_eq::assert_nearly_eq;

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
            vision: (FIRST_VISION_INDEX..INPUT_NEURON_COUNT)
                .map(Handle)
                .collect(),
        };

        let mut values = HashMap::with_capacity(2);
        add_acceleration_inputs(
            configuration.input_acceleration,
            &mapping,
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

}
