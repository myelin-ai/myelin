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

const MAX_ANGULAR_FORCE: f64 = MAX_ACCELERATION_FORCE;

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

/// 2. axial force (backward)
/// 3. lateral force (left)
/// 4. lateral force (right)
/// 5. torque (counterclockwise)
/// 6. torque (clockwise)
const OUTPUT_NEURON_COUNT: usize = 6;

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

        add_vision_inputs(
            &vision_neuron_inputs,
            &neuron_handle_mapping.input,
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
    (0..RAYCAST_COUNT)
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
    let axial_acceleration_handle = axial_acceleration_handle(
        acceleration.x,
        input_neuron_handle_mapping.axial_acceleration,
    );
    if let Some(axial_acceleration_handle) = axial_acceleration_handle {
        add_input_fn(
            axial_acceleration_handle,
            acceleration.x.abs().min(MAX_ACCELERATION) / MAX_ACCELERATION,
        );
    }

    let lateral_acceleration_handle = lateral_acceleration_handle(
        acceleration.y,
        input_neuron_handle_mapping.lateral_acceleration,
    );
    if let Some(lateral_acceleration_handle) = lateral_acceleration_handle {
        add_input_fn(
            lateral_acceleration_handle,
            acceleration.y.abs().min(MAX_ACCELERATION) / MAX_ACCELERATION,
        );
    }
}

fn add_vision_inputs(
    vision_neuron_inputs: &[Option<f64>],
    input_neuron_handle_mapping: &InputNeuronHandleMapping,
    mut add_input_fn: impl FnMut(Handle, f64),
) {
    input_neuron_handle_mapping
        .vision
        .iter()
        .zip(vision_neuron_inputs.iter())
        .filter_map(|(handle, &input)| Some((handle, input?)))
        .for_each(|(handle, input)| {
            /// A bit more than the size of the simulated world
            const MAXIMAL_VIEWABLE_DISTANCE_IN_METERS: f64 = 1200.0;
            let normalized_input = input / MAXIMAL_VIEWABLE_DISTANCE_IN_METERS;
            add_input_fn(*handle, normalized_input);
        });
}

/// Arbitrary value
const MIN_PERCEIVABLE_ACCELERATION: f64 = 0.0001;

fn axial_acceleration_handle(
    axial_acceleration: f64,
    axial_acceleration_handle_mapping: AxialAccelerationHandleMapping,
) -> Option<Handle> {
    if axial_acceleration >= MIN_PERCEIVABLE_ACCELERATION {
        Some(axial_acceleration_handle_mapping.forward)
    } else if axial_acceleration <= -MIN_PERCEIVABLE_ACCELERATION {
        Some(axial_acceleration_handle_mapping.backward)
    } else {
        None
    }
}

fn lateral_acceleration_handle(
    lateral_acceleration: f64,
    lateral_acceleration_handle_mapping: LateralAccelerationHandleMapping,
) -> Option<Handle> {
    if lateral_acceleration <= -MIN_PERCEIVABLE_ACCELERATION {
        Some(lateral_acceleration_handle_mapping.left)
    } else if lateral_acceleration >= MIN_PERCEIVABLE_ACCELERATION {
        Some(lateral_acceleration_handle_mapping.right)
    } else {
        None
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
        test_expected_handle_is_returned_for_axial_acceleration(-1.0, Handle(1));
    }

    #[test]
    fn axial_acceleration_handle_returns_correct_handle_for_zero() {
        test_expected_handle_is_returned_for_axial_acceleration(0.0, None);
    }

    #[test]
    fn axial_acceleration_handle_returns_correct_handle_for_one() {
        test_expected_handle_is_returned_for_axial_acceleration(1.0, Handle(0));
    }

    fn test_expected_handle_is_returned_for_axial_acceleration(
        axial_acceleration: f64,
        expected_handle: impl Into<Option<Handle>>,
    ) {
        let mapping = AxialAccelerationHandleMapping {
            forward: Handle(0),
            backward: Handle(1),
        };

        let handle = axial_acceleration_handle(axial_acceleration, mapping);

        assert_eq!(expected_handle.into(), handle);
    }

    #[test]
    fn lateral_acceleration_handle_returns_correct_handle_for_minus_one() {
        test_expected_handle_is_returned_for_lateral_acceleration(-1.0, Handle(0));
    }

    #[test]
    fn lateral_acceleration_handle_returns_correct_handle_for_zero() {
        test_expected_handle_is_returned_for_lateral_acceleration(0.0, None);
    }

    #[test]
    fn lateral_acceleration_handle_returns_correct_handle_for_one() {
        test_expected_handle_is_returned_for_lateral_acceleration(1.0, Handle(1));
    }

    fn test_expected_handle_is_returned_for_lateral_acceleration(
        axial_acceleration: f64,
        expected_handle: impl Into<Option<Handle>>,
    ) {
        let mapping = LateralAccelerationHandleMapping {
            left: Handle(0),
            right: Handle(1),
        };

        let handle = lateral_acceleration_handle(axial_acceleration, mapping);

        assert_eq!(expected_handle.into(), handle);
    }

    // Axial expected handles: (0 forward, 1 backward)
    // Lateral expected handles: (2 left, 3 right)
    #[derive(Debug)]
    struct AddAccelerationInputsTestConfiguration {
        input_acceleration: Vector,
        axial_expected_value: Option<(Handle, f64)>,
        lateral_expected_value: Option<(Handle, f64)>,
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

        let mut values = HashMap::new();
        add_acceleration_inputs(
            configuration.input_acceleration,
            &mapping,
            |handle, value| {
                values.insert(handle, value);
            },
        );

        let expected_length = configuration.axial_expected_value.map_or(0, |_| 1)
            + configuration.lateral_expected_value.map_or(0, |_| 1);

        assert_eq!(expected_length, values.len());

        if let Some((handle, axial_expected_value)) = configuration.axial_expected_value {
            assert_nearly_eq!(
                axial_expected_value,
                *values.get(&handle).expect("Axial input was None")
            );
        };

        if let Some((handle, lateral_expected_value)) = configuration.lateral_expected_value {
            assert_nearly_eq!(
                lateral_expected_value,
                *values.get(&handle).expect("Lateral input was None")
            );
        };
    }

    #[test]
    fn add_acceleration_inputs_with_no_acceleration() {
        let configuration = AddAccelerationInputsTestConfiguration {
            input_acceleration: Vector { x: 0.0, y: 0.0 },
            axial_expected_value: None,
            lateral_expected_value: None,
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
            axial_expected_value: Some((Handle(0), 0.2)),
            lateral_expected_value: None,
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
            axial_expected_value: Some((Handle(1), 0.2)),
            lateral_expected_value: None,
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
            axial_expected_value: None,
            lateral_expected_value: Some((Handle(2), 0.2)),
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
            axial_expected_value: None,
            lateral_expected_value: Some((Handle(3), 0.2)),
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
            axial_expected_value: Some((Handle(0), 1.0)),
            lateral_expected_value: None,
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
            axial_expected_value: Some((Handle(1), 1.0)),
            lateral_expected_value: None,
        };

        add_acceleration_inputs_test(configuration);
    }

    #[test]
    fn neural_network_output_is_mapped_to_action() {
        let developed_neural_network = mock_developed_neural_network();
        let mapping = map_handles(&developed_neural_network);

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

        let object_description = object_description().build().unwrap();

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

    fn object_description() -> ObjectBuilder {
        let mut builder = ObjectBuilder::default();
        builder
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
            .rotation(Radians::try_new(PI).unwrap());
        builder
    }

    fn mock_developed_neural_network() -> DevelopedNeuralNetwork {
        DevelopedNeuralNetwork {
            input_neuron_handles: (0..INPUT_NEURON_COUNT).map(Handle).collect(),
            output_neuron_handles: (0..OUTPUT_NEURON_COUNT).map(Handle).collect(),
            neural_network: Box::new(NeuralNetworkMock::new()),
            genome: Genome {},
        }
    }

    #[test]
    fn objects_in_fov_is_empty_with_no_surrounding_objects() {
        test_objects_in_fov_are_as_expected(ExpectedFovObjects {
            first_objects_in_ray: Vec::new(),
            second_objects_in_ray: Vec::new(),
            third_objects_in_ray: Vec::new(),
            fourth_objects_in_ray: Vec::new(),
            fifth_objects_in_ray: Vec::new(),
            sixth_objects_in_ray: Vec::new(),
            seventh_objects_in_ray: Vec::new(),
            eight_objects_in_ray: Vec::new(),
            ninth_objects_in_ray: Vec::new(),
            tenth_objects_in_ray: Vec::new(),
            expected_objects: vec![Vec::new(); 10],
        })
    }

    #[test]
    fn objects_in_fov_is_sorted_and_filtered_correctly() {
        let mock_behavior = ObjectBehaviorMock::new();
        let mut counter = 0;
        let mut genenerate_objects = |amount| -> Vec<Object<'_>> {
            (0..amount)
                .map(|_| {
                    let object = Object {
                        id: counter,
                        behavior: &mock_behavior,
                        description: object_description()
                            .location(1.0 + counter as f64, 1.0 + counter as f64)
                            .build()
                            .unwrap(),
                    };
                    counter += 1;
                    object
                })
                .collect()
        };
        let fourth_objects = genenerate_objects(6);
        let sixth_objects = genenerate_objects(2);
        let seventh_objects = genenerate_objects(1);

        test_objects_in_fov_are_as_expected(ExpectedFovObjects {
            first_objects_in_ray: Vec::new(),
            second_objects_in_ray: Vec::new(),
            third_objects_in_ray: Vec::new(),
            fourth_objects_in_ray: fourth_objects.clone(),
            fifth_objects_in_ray: Vec::new(),
            sixth_objects_in_ray: sixth_objects.clone(),
            seventh_objects_in_ray: seventh_objects.clone(),
            eight_objects_in_ray: Vec::new(),
            ninth_objects_in_ray: Vec::new(),
            tenth_objects_in_ray: Vec::new(),
            expected_objects: vec![
                Vec::new(),
                Vec::new(),
                Vec::new(),
                fourth_objects
                    .into_iter()
                    .take(MAX_OBJECTS_PER_RAYCAST)
                    .collect(),
                Vec::new(),
                sixth_objects
                    .into_iter()
                    .take(MAX_OBJECTS_PER_RAYCAST)
                    .collect(),
                seventh_objects
                    .into_iter()
                    .take(MAX_OBJECTS_PER_RAYCAST)
                    .collect(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        })
    }

    #[derive(Debug, Default)]
    struct ExpectedFovObjects<'a> {
        first_objects_in_ray: Snapshot<'a>,
        second_objects_in_ray: Snapshot<'a>,
        third_objects_in_ray: Snapshot<'a>,
        fourth_objects_in_ray: Snapshot<'a>,
        fifth_objects_in_ray: Snapshot<'a>,
        sixth_objects_in_ray: Snapshot<'a>,
        seventh_objects_in_ray: Snapshot<'a>,
        eight_objects_in_ray: Snapshot<'a>,
        ninth_objects_in_ray: Snapshot<'a>,
        tenth_objects_in_ray: Snapshot<'a>,
        expected_objects: Vec<Vec<Object<'a>>>,
    }

    fn test_objects_in_fov_are_as_expected(expected_fov_objects: ExpectedFovObjects<'_>) {
        let own_description = object_description().build().unwrap();
        let mut world_interactor = WorldInteractorMock::new();

        let mut connect_ray_to_expectation = |ray, expectation| {
            world_interactor
                .expect_find_objects_in_ray(partial_eq(own_description.location), partial_eq(ray))
                .returns(expectation);
        };

        let first_ray = Vector {
            x: 0.17364817766693041,
            y: 0.984807753012208,
        };
        connect_ray_to_expectation(first_ray, expected_fov_objects.first_objects_in_ray);

        let second_ray = Vector {
            x: -0.17364817766693025,
            y: 0.9848077530122081,
        };
        connect_ray_to_expectation(second_ray, expected_fov_objects.second_objects_in_ray);

        let third_ray = Vector {
            x: -0.4999999999999999,
            y: 0.8660254037844387,
        };
        connect_ray_to_expectation(third_ray, expected_fov_objects.third_objects_in_ray);

        let fourth_ray = Vector {
            x: -0.7660444431189779,
            y: 0.6427876096865395,
        };
        connect_ray_to_expectation(fourth_ray, expected_fov_objects.fourth_objects_in_ray);

        let fifth_ray = Vector {
            x: -0.9396926207859083,
            y: 0.3420201433256688,
        };
        connect_ray_to_expectation(fifth_ray, expected_fov_objects.fifth_objects_in_ray);

        let sixth_ray = Vector {
            x: -0.9999999999999999,
            y: 0.00000000000000008326672684688674,
        };
        connect_ray_to_expectation(sixth_ray, expected_fov_objects.sixth_objects_in_ray);

        let seventh_ray = Vector {
            x: -0.9396926207859085,
            y: -0.34202014332566844,
        };
        connect_ray_to_expectation(seventh_ray, expected_fov_objects.seventh_objects_in_ray);

        let eight_ray = Vector {
            x: -0.7660444431189782,
            y: -0.642787609686539,
        };
        connect_ray_to_expectation(eight_ray, expected_fov_objects.eight_objects_in_ray);

        let ninth_ray = Vector {
            x: -0.5000000000000002,
            y: -0.8660254037844385,
        };
        connect_ray_to_expectation(ninth_ray, expected_fov_objects.ninth_objects_in_ray);

        let tenth_ray = Vector {
            x: -0.17364817766693053,
            y: -0.984807753012208,
        };
        connect_ray_to_expectation(tenth_ray, expected_fov_objects.tenth_objects_in_ray);

        let objects_in_fov = objects_in_fov(&own_description, &world_interactor);
        assert_eq!(
            expected_fov_objects.expected_objects.len(),
            objects_in_fov.len()
        );
        for (expected_objects_in_ray, objects_in_ray) in expected_fov_objects
            .expected_objects
            .iter()
            .zip(objects_in_fov.iter())
        {
            assert_eq!(expected_objects_in_ray.len(), objects_in_ray.len());
            for (expected_object, object) in
                expected_objects_in_ray.iter().zip(objects_in_ray.iter())
            {
                assert_eq!(expected_object.id, object.id);
                assert_eq!(expected_object.description, object.description);
            }
        }
    }

    #[test]
    fn no_objects_in_fov_are_mapped_to_no_neural_inputs() {
        let own_description = object_description().build().unwrap();
        let objects_in_fov = Vec::new();
        let inputs = objects_in_fov_to_neuron_inputs(&own_description, &objects_in_fov);
        assert!(inputs.is_empty());
    }

    #[test]
    fn objects_in_fov_are_mapped_to_neural_inputs() {
        // Todo: Move into generator
        let mock_behavior = ObjectBehaviorMock::new();
        let mut counter = 0;
        let mut genenerate_objects = |amount| -> Vec<Object<'_>> {
            (0..amount)
                .map(|_| {
                    let object = Object {
                        id: counter,
                        behavior: &mock_behavior,
                        description: object_description()
                            .location(1.0 + counter as f64, 1.0 + counter as f64)
                            .build()
                            .unwrap(),
                    };
                    counter += 1;
                    object
                })
                .collect()
        };

        let own_description = object_description().build().unwrap();
        let objects_in_fov = vec![
            Vec::new(),
            Vec::new(),
            genenerate_objects(3),
            Vec::new(),
            genenerate_objects(2),
            Vec::new(),
            genenerate_objects(1),
            genenerate_objects(3),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let inputs = objects_in_fov_to_neuron_inputs(&own_description, &objects_in_fov);

        assert_eq!(9, inputs.len());
    }
}
