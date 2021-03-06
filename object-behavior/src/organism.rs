//! Behavior of an organism that can interact with its surroundings

use itertools::Itertools;
use myelin_engine::prelude::*;
use myelin_genetics::{
    DevelopedNeuralNetwork, GenomeGenerator, GenomeGeneratorConfiguration, GenomeOrigin,
    NeuralNetworkDevelopmentConfiguration, NeuralNetworkDevelopmentOrchestrator,
};
use myelin_neural_network::{Handle, Milliseconds, NeuralNetwork};
use myelin_object_data::{AdditionalObjectDescription, Object, ObjectDescription};

use std::collections::HashMap;
use std::num::NonZeroUsize;
/// The hightest relative acceleration an organism can detect.
/// The value was chosen as many sources, [including Wikipedia](https://en.wikipedia.org/wiki/G-LOC#Thresholds) report
/// 5G as a typical threshold for the loss of consciousness in humans.
/// The origins of this number have not been verified.
const MAX_ACCELERATION: f64 = 5.0 * 9.81;

/// The highest possible force emmited by the organism.
/// Calculated as F = μma, where μ = 1, m = 20kg (hardcoded value in engine) and a = 9.8 m/s^2,
/// which is the [maximum acceleration a human can achieve](https://www.wired.com/2012/08/maximum-acceleration-in-the-100-m-dash/)
const MAX_ACCELERATION_FORCE: f64 = 20.0 * 9.8;

/// Our research indicates that these seem to be the same
const MAX_ANGULAR_FORCE: f64 = MAX_ACCELERATION_FORCE;

/// Number of rays sent out by an organism to detect visible objects
const RAYCAST_COUNT: usize = 10;
/// Number of objects that can be detected by a vision ray
const MAX_OBJECTS_PER_RAYCAST: usize = 3;

/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
#[derive(Debug, Clone)]
pub struct OrganismBehavior {
    previous_velocity: Vector,
    developed_neural_network: DevelopedNeuralNetwork,
    neural_network_developer: Box<dyn NeuralNetworkDevelopmentOrchestrator>,
}

/// Number of inputs reserved for visible objects
const VISION_INPUT_COUNT: usize = RAYCAST_COUNT * MAX_OBJECTS_PER_RAYCAST;

/// 1. Average axial acceleration since last step (forward)
/// 2. Average axial acceleration since last step (backward)
/// 3. Average lateral acceleration since last step (left)
/// 4. Average lateral acceleration since last step (right)
/// Rest: Distances to objects in FOV from right to left
fn input_neuron_count() -> NonZeroUsize {
    NonZeroUsize::new(4 + VISION_INPUT_COUNT).unwrap()
}

fn first_vision_index() -> usize {
    input_neuron_count().get() - VISION_INPUT_COUNT + 1
}

/// 2. axial force (backward)
/// 3. lateral force (left)
/// 4. lateral force (right)
/// 5. torque (counterclockwise)
/// 6. torque (clockwise)
fn output_neuron_count() -> NonZeroUsize {
    NonZeroUsize::new(6).unwrap()
}

impl OrganismBehavior {
    /// Create a new `OrganismBehavior` from a pair of parent [`Genome`]s.
    /// The [`NeuralNetworkDeveloper`] is used to create this organism's [`NeuralNetwork`]
    /// and its eventual offspring.
    ///
    /// [`Genome`]: ../myelin-genetics/struct.Genome.html
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    pub fn new(
        genome_origin: GenomeOrigin,
        neural_network_developer: Box<dyn NeuralNetworkDevelopmentOrchestrator>,
    ) -> Self {
        let configuration = NeuralNetworkDevelopmentConfiguration {
            genome_origin,
            input_neuron_count: input_neuron_count(),
            output_neuron_count: output_neuron_count(),
        };

        Self {
            previous_velocity: Vector::default(),
            developed_neural_network: neural_network_developer
                .develop_neural_network(&configuration),
            neural_network_developer,
        }
    }

    /// Create a new `OrganismBehavior` with a newly developed [`Genome`] using a [`GenomeGenerator`].
    /// The [`NeuralNetworkDeveloper`] is used to create this organism's [`NeuralNetwork`]
    /// and its eventual offspring.
    ///
    /// [`Genome`]: ../myelin-genetics/struct.Genome.html
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    pub fn from_genome_generator(
        genome_generator: Box<dyn GenomeGenerator>,
        neural_network_developer: Box<dyn NeuralNetworkDevelopmentOrchestrator>,
    ) -> Self {
        let configuration = GenomeGeneratorConfiguration {
            input_neuron_count: input_neuron_count(),
            output_neuron_count: output_neuron_count(),
        };
        let genome = genome_generator.generate_genome(&configuration);
        Self::new(GenomeOrigin::Genesis(genome), neural_network_developer)
    }
}

impl ObjectBehavior<AdditionalObjectDescription> for OrganismBehavior {
    fn step(
        &mut self,
        world_interactor: Box<dyn WorldInteractor<AdditionalObjectDescription> + '_>,
    ) -> Option<Action<AdditionalObjectDescription>> {
        let elapsed_time = world_interactor.elapsed_time_in_update().as_millis() as Milliseconds;
        let own_object = world_interactor.own_object();

        let neuron_handle_mapping = map_handles(&self.developed_neural_network);

        let current_velocity = velocity(&own_object.description);
        let absolute_acceleration = (current_velocity - self.previous_velocity) / elapsed_time;
        let relative_acceleration =
            absolute_acceleration.rotate_clockwise(own_object.description.rotation);

        self.previous_velocity = current_velocity;

        let mut inputs = HashMap::with_capacity(2);
        let mut insert_input_fn = |key, value| {
            inputs.insert(key, value);
        };

        add_acceleration_inputs(
            relative_acceleration,
            &neuron_handle_mapping.input,
            &mut insert_input_fn,
        );

        let objects_in_fov = objects_in_fov(&own_object.description, &*world_interactor);
        let vision_neuron_inputs =
            objects_in_fov_to_neuron_inputs(&own_object.description, objects_in_fov);

        add_vision_inputs(
            vision_neuron_inputs,
            &neuron_handle_mapping.input,
            &mut insert_input_fn,
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
}

fn convert_neural_network_output_to_action(
    neuron_handle_mapping: NeuronHandleMapping,
    neural_network: &dyn NeuralNetwork,
    object_description: &ObjectDescription,
) -> Option<Action<AdditionalObjectDescription>> {
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

fn objects_in_fov<'a>(
    own_description: &'a ObjectDescription,
    world_interactor: &'a dyn WorldInteractor<AdditionalObjectDescription>,
) -> impl Iterator<Item = (impl Iterator<Item = Object<'a>> + 'a)> + 'a {
    /// The angle in degrees describing the field of view. [Wikipedia](https://en.wikipedia.org/wiki/Human_eye#Field_of_view).
    const FOV_ANGLE: usize = 200;
    const ANGLE_PER_RAYCAST: f64 = FOV_ANGLE as f64 / RAYCAST_COUNT as f64;

    let unit_vector = Vector { x: 1.0, y: 0.0 };
    let own_direction = unit_vector.rotate(own_description.rotation);

    let half_of_fov_angle = Radians::try_from_degrees(FOV_ANGLE as f64 / 2.0).unwrap();
    let rightmost_angle = own_direction.rotate_clockwise(half_of_fov_angle);
    (0..RAYCAST_COUNT).map(move |angle_step| {
        // Todo(#361): The following three lines produce slightly different numbers on macOS
        let angle_in_degrees = angle_step as f64 * ANGLE_PER_RAYCAST;
        let angle_in_radians = Radians::try_from_degrees(angle_in_degrees).unwrap();
        let fov_direction = rightmost_angle.rotate(angle_in_radians);

        world_interactor
            .find_objects_in_ray(own_description.location, fov_direction)
            .into_iter()
    })
}

fn objects_in_fov_to_neuron_inputs<'a, T, U>(
    own_description: &'a ObjectDescription,
    objects: T,
) -> impl Iterator<Item = Option<f64>> + 'a
where
    T: IntoIterator<Item = U> + 'a,
    U: IntoIterator<Item = Object<'a>> + 'a,
{
    let own_associated_data = &own_description.associated_data;

    objects
        .into_iter()
        .map(move |objects_in_ray| {
            let mut distances: Vec<_> = objects_in_ray
                .into_iter()
                .map(|object| {
                    let distance = distance_between_objects(&object.description, own_description);
                    (object.description.associated_data, distance)
                })
                .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .scan(0.0, |running_max, (associated_data, distance)| {
                    filter_visible_object(
                        running_max,
                        associated_data.height,
                        own_associated_data.height,
                        distance,
                    )
                })
                .take(MAX_OBJECTS_PER_RAYCAST)
                .collect();

            distances.resize(MAX_OBJECTS_PER_RAYCAST, None);
            distances
        })
        .flatten()
}

#[allow(clippy::option_option)]
fn filter_visible_object(
    running_max: &mut f64,
    object_height: f64,
    own_height: f64,
    distance: f64,
) -> Option<Option<f64>> {
    let distance = filter_shorter_object(running_max, object_height, distance);

    ensure_that_objects_behind_obstacle_that_is_taller_than_self_are_not_visible(
        running_max,
        own_height,
        object_height,
    );

    Some(distance)
}

fn filter_shorter_object(running_max: &mut f64, object_height: f64, distance: f64) -> Option<f64> {
    if object_height < *running_max {
        None
    } else {
        *running_max = object_height;
        Some(distance)
    }
}

fn ensure_that_objects_behind_obstacle_that_is_taller_than_self_are_not_visible(
    running_max: &mut f64,
    object_height: f64,
    own_height: f64,
) {
    if object_height > own_height {
        *running_max = std::f64::MAX
    }
}

fn distance_between_objects(
    first_object: &ObjectDescription,
    second_object: &ObjectDescription,
) -> f64 {
    Vector::from(first_object.location - second_object.location).magnitude()
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

fn add_vision_inputs<T>(
    distances: T,
    input_neuron_handle_mapping: &InputNeuronHandleMapping,
    mut add_input_fn: impl FnMut(Handle, f64),
) where
    T: IntoIterator<Item = Option<f64>>,
{
    input_neuron_handle_mapping
        .vision
        .iter()
        .zip(distances.into_iter())
        .filter_map(|(handle, distance)| Some((handle, distance?)))
        .for_each(|(handle, distance)| {
            let input_intensity_by_proximity = MAX_DISTINGUISHABLE_DISTANCE_IN_METERS - distance;
            let scaled_input =
                input_intensity_by_proximity / MAX_DISTINGUISHABLE_DISTANCE_IN_METERS;
            let clamped_input = scaled_input.clamp(0.0, 1.0);
            add_input_fn(*handle, clamped_input);
        });
}

/// Arbitrary value
const MAX_DISTINGUISHABLE_DISTANCE_IN_METERS: f64 = 1200.0;

/// Arbitrary value
const MIN_PERCEIVABLE_ACCELERATION: f64 = 0.000_1;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct OutputNeuronHandleMapping {
    axial_acceleration: AxialAccelerationHandleMapping,
    lateral_acceleration: LateralAccelerationHandleMapping,
    torque: TorqueHandleMapping,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct AxialAccelerationHandleMapping {
    forward: Handle,
    backward: Handle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LateralAccelerationHandleMapping {
    left: Handle,
    right: Handle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
            vision: (first_vision_index()..input_neuron_count().get())
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
    use myelin_genetics::genome::Genome;
    use myelin_genetics::{GenomeGeneratorMock, NeuralNetworkDevelopmentOrchestratorMock};
    use myelin_neural_network::NeuralNetworkMock;
    use myelin_object_data::AdditionalObjectDescription;
    use myelin_object_data::Kind;
    use nearly_eq::assert_nearly_eq;
    use std::f64::consts::PI;
    use std::iter;

    #[test]
    fn can_be_constructed_with_genome_generator() {
        let expected_genome = Genome::default();
        let expected_developed_neural_network = DevelopedNeuralNetwork {
            neural_network: box NeuralNetworkMock::new(),
            genome: expected_genome.clone(),
            input_neuron_handles: Vec::new(),
            output_neuron_handles: Vec::new(),
        };

        let mut genome_generator = GenomeGeneratorMock::new();
        genome_generator
            .expect_generate_genome(|arg| arg.any())
            .returns(expected_genome.clone());

        let mut neural_network_developer = NeuralNetworkDevelopmentOrchestratorMock::new();
        neural_network_developer
            .expect_develop_neural_network(|arg| arg.any())
            .returns(expected_developed_neural_network.clone());

        let organism_behaviour = OrganismBehavior::from_genome_generator(
            box genome_generator,
            box neural_network_developer,
        );
        let developed_neural_network = &organism_behaviour.developed_neural_network;
        assert_eq!(
            expected_developed_neural_network.genome,
            developed_neural_network.genome
        );
        assert_eq!(
            expected_developed_neural_network.input_neuron_handles,
            developed_neural_network.input_neuron_handles
        );
        assert_eq!(
            expected_developed_neural_network.output_neuron_handles,
            developed_neural_network.output_neuron_handles
        );
    }

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
            vision: (first_vision_index()..input_neuron_count().get())
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
            .expect_normalized_potential_of_neuron(|arg| {
                arg.partial_eq(mapping.output.axial_acceleration.forward)
            })
            .returns(Ok(Some(0.5)));
        network
            .expect_normalized_potential_of_neuron(|arg| {
                arg.partial_eq(mapping.output.axial_acceleration.backward)
            })
            .returns(Ok(Some(0.0)));
        network
            .expect_normalized_potential_of_neuron(|arg| {
                arg.partial_eq(mapping.output.lateral_acceleration.left)
            })
            .returns(Ok(Some(0.2)));
        network
            .expect_normalized_potential_of_neuron(|arg| {
                arg.partial_eq(mapping.output.lateral_acceleration.right)
            })
            .returns(Ok(Some(0.0)));
        network
            .expect_normalized_potential_of_neuron(|arg| {
                arg.partial_eq(mapping.output.torque.counterclockwise)
            })
            .returns(Ok(Some(0.0)));
        network
            .expect_normalized_potential_of_neuron(|arg| {
                arg.partial_eq(mapping.output.torque.clockwise)
            })
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

    fn object_description() -> ObjectBuilder<AdditionalObjectDescription> {
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
            .rotation(Radians::try_new(PI).unwrap())
            .associated_data(AdditionalObjectDescription {
                name: None,
                kind: Kind::Organism,
                height: 1.0,
            });
        builder
    }

    fn mock_developed_neural_network() -> DevelopedNeuralNetwork {
        DevelopedNeuralNetwork {
            input_neuron_handles: (0..input_neuron_count().get()).map(Handle).collect(),
            output_neuron_handles: (0..output_neuron_count().get()).map(Handle).collect(),
            neural_network: box NeuralNetworkMock::new(),
            genome: Genome::default(),
        }
    }

    #[test]
    #[cfg_attr(target_os = "macos", ignore)]
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
    #[cfg_attr(target_os = "macos", ignore)]
    fn objects_in_fov_are_filtered_correctly() {
        let mock_behavior = ObjectBehaviorMock::new();
        let mut counter = 0;
        let mut genenerate_objects =
            |amount| generate_objects(amount, &mock_behavior, 1.0, &mut counter);

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
                fourth_objects,
                Vec::new(),
                sixth_objects,
                seventh_objects,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        })
    }

    #[derive(Debug, Default)]
    struct ExpectedFovObjects<'a> {
        first_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        second_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        third_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        fourth_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        fifth_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        sixth_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        seventh_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        eight_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        ninth_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        tenth_objects_in_ray: Snapshot<'a, AdditionalObjectDescription>,
        expected_objects: Vec<Vec<Object<'a>>>,
    }

    fn test_objects_in_fov_are_as_expected(expected_fov_objects: ExpectedFovObjects<'_>) {
        let own_description = object_description().build().unwrap();
        let mut world_interactor = WorldInteractorMock::new();

        let mut connect_ray_to_expectation = |ray, expectation| {
            world_interactor
                .expect_find_objects_in_ray(
                    |arg| arg.partial_eq(own_description.location),
                    |arg| arg.partial_eq(ray),
                )
                .returns(expectation);
        };

        let first_ray = Vector {
            x: 0.173_648_177_666_930_41,
            y: 0.984_807_753_012_208,
        };
        connect_ray_to_expectation(first_ray, expected_fov_objects.first_objects_in_ray);

        let second_ray = Vector {
            x: -0.173_648_177_666_930_25,
            y: 0.984_807_753_012_208_1,
        };
        connect_ray_to_expectation(second_ray, expected_fov_objects.second_objects_in_ray);

        let third_ray = Vector {
            x: -0.499_999_999_999_999_9,
            /// On macOS, the actual value is for some reason 0.866_025_403_784_438_7
            y: 0.866_025_403_784_438_6,
        };
        connect_ray_to_expectation(third_ray, expected_fov_objects.third_objects_in_ray);

        let fourth_ray = Vector {
            x: -0.766_044_443_118_977_9,
            y: 0.642_787_609_686_539_5,
        };
        connect_ray_to_expectation(fourth_ray, expected_fov_objects.fourth_objects_in_ray);

        let fifth_ray = Vector {
            x: -0.939_692_620_785_908_3,
            y: 0.342_020_143_325_668_8,
        };
        connect_ray_to_expectation(fifth_ray, expected_fov_objects.fifth_objects_in_ray);

        let sixth_ray = Vector {
            x: -0.999_999_999_999_999_9,
            y: 0.000_000_000_000_000_083_266_726_846_886_74,
        };
        connect_ray_to_expectation(sixth_ray, expected_fov_objects.sixth_objects_in_ray);

        let seventh_ray = Vector {
            /// On macOS, the actual value is for some reason 0.939_692_620_785_908_4
            x: -0.939_692_620_785_908_4,
            y: -0.342_020_143_325_668_44,
        };
        connect_ray_to_expectation(seventh_ray, expected_fov_objects.seventh_objects_in_ray);

        let eight_ray = Vector {
            x: -0.766_044_443_118_978_2,
            y: -0.642_787_609_686_539,
        };
        connect_ray_to_expectation(eight_ray, expected_fov_objects.eight_objects_in_ray);

        let ninth_ray = Vector {
            x: -0.500_000_000_000_000_2,
            y: -0.866_025_403_784_438_5,
        };
        connect_ray_to_expectation(ninth_ray, expected_fov_objects.ninth_objects_in_ray);

        let tenth_ray = Vector {
            x: -0.173_648_177_666_930_53,
            y: -0.984_807_753_012_208,
        };
        connect_ray_to_expectation(tenth_ray, expected_fov_objects.tenth_objects_in_ray);

        let objects_in_fov: Vec<_> = objects_in_fov(&own_description, &world_interactor).collect();
        assert_eq!(
            expected_fov_objects.expected_objects.len(),
            objects_in_fov.len()
        );
        for (expected_objects_in_ray, objects_in_ray) in expected_fov_objects
            .expected_objects
            .iter()
            .zip(objects_in_fov.into_iter())
        {
            let objects_in_ray: Vec<_> = objects_in_ray.collect();
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
        let objects_in_fov: Vec<Vec<_>> = Vec::new();

        let inputs = objects_in_fov_to_neuron_inputs(&own_description, objects_in_fov);
        assert_eq!(0, inputs.count());
    }

    #[test]
    fn objects_in_fov_are_mapped_to_neural_inputs() {
        let mut counter = 0;
        let mock_behavior = ObjectBehaviorMock::new();

        let mut genenerate_objects = |configurations: Vec<(usize, f64)>| -> Vec<Object<'_>> {
            /// This arbitrary shift jumbles the objects up,
            /// later testing if their distances are sorted
            const SHIFT: usize = 2;

            let total_amount = configurations.iter().map(|(amount, _)| amount).sum();
            let objects: Vec<_> = configurations
                .into_iter()
                .map(|(amount, height)| {
                    generate_objects(amount, &mock_behavior, height, &mut counter)
                })
                .flatten()
                .collect();
            objects
                .into_iter()
                .cycle()
                .skip(SHIFT)
                .take(total_amount)
                .collect()
        };

        let own_description = object_description().build().unwrap();
        let objects_in_fov = vec![
            Vec::new(),
            Vec::new(),
            genenerate_objects(vec![(4, 1.0), (1, 2.0), (1, 1.0)]),
            Vec::new(),
            genenerate_objects(vec![(2, 1.0)]),
            Vec::new(),
            genenerate_objects(vec![(1, 1.0)]),
            genenerate_objects(vec![(4, 1.0)]),
            Vec::new(),
            Vec::new(),
        ];
        assert_eq!(RAYCAST_COUNT, objects_in_fov.len());

        let inputs: Vec<_> =
            objects_in_fov_to_neuron_inputs(&own_description, objects_in_fov).collect();

        let no_distances = vec![None; MAX_OBJECTS_PER_RAYCAST];
        let first_distances = no_distances.clone();
        let second_distances = no_distances.clone();
        let points_to_distances = |points: &[f64]| {
            // Return the length of a vector from [0, 0] to [point, point]
            // Fill the returned values with `None` until `MAX_OBJECTS_PER_RAYCAST`
            points
                .iter()
                .map(|&point| 2.0 * f64::powf(point, 2.0))
                .map(f64::sqrt)
                .map(Some)
                .chain(iter::repeat(None))
                .take(MAX_OBJECTS_PER_RAYCAST)
                .collect()
        };
        let third_distances = points_to_distances(&[1.0, 2.0, 3.0]);
        let fourth_distances = no_distances.clone();
        let fifth_distances = points_to_distances(&[7.0, 8.0]);
        let sixth_distances = no_distances.clone();
        let seventh_distances = points_to_distances(&[9.0]);
        let eight_distances = points_to_distances(&[10.0, 11.0, 12.0]);
        let ninth_distances = no_distances.clone();
        let tenth_distances = no_distances;

        let expected_inputs: Vec<Option<f64>> = vec![
            first_distances,
            second_distances,
            third_distances,
            fourth_distances,
            fifth_distances,
            sixth_distances,
            seventh_distances,
            eight_distances,
            ninth_distances,
            tenth_distances,
        ]
        .into_iter()
        .flatten()
        .collect();
        assert_eq!(
            RAYCAST_COUNT * MAX_OBJECTS_PER_RAYCAST,
            expected_inputs.len()
        );

        assert_eq!(expected_inputs, inputs);
    }

    fn generate_objects<'a, 'b>(
        amount: usize,
        object_behavior: &'a dyn ObjectBehavior<AdditionalObjectDescription>,
        height: f64,
        counter: &'b mut usize,
    ) -> Vec<Object<'a>> {
        (0..amount)
            .map(|_| {
                let object = Object {
                    id: *counter,
                    behavior: object_behavior,
                    description: object_description()
                        .location(1.0 + *counter as f64, 1.0 + *counter as f64)
                        .associated_data(AdditionalObjectDescription {
                            name: None,
                            kind: Kind::Organism,
                            height,
                        })
                        .build()
                        .unwrap(),
                };
                *counter += 1;
                object
            })
            .collect()
    }

    #[test]
    fn clamps_max_distance() {
        test_distance_is_converted_to_input(MAX_DISTINGUISHABLE_DISTANCE_IN_METERS, 0.0);
    }

    #[test]
    fn clamps_zero_distance() {
        test_distance_is_converted_to_input(0.0, 1.0);
    }

    #[test]
    fn clamps_negative_distance() {
        test_distance_is_converted_to_input(-100.0, 1.0);
    }

    #[test]
    fn clamps_too_far_distance() {
        test_distance_is_converted_to_input(MAX_DISTINGUISHABLE_DISTANCE_IN_METERS + 0.1, 0.0);
    }

    #[test]
    fn scales_half_of_max_distance() {
        test_distance_is_converted_to_input(MAX_DISTINGUISHABLE_DISTANCE_IN_METERS * 0.5, 0.5);
    }

    #[test]
    fn scales_a_quarter_of_max_distance() {
        test_distance_is_converted_to_input(MAX_DISTINGUISHABLE_DISTANCE_IN_METERS * 0.25, 0.75);
    }

    fn test_distance_is_converted_to_input(distance: f64, expected_input: f64) {
        let distances = vec![Some(distance)];
        let input_neuron_handle_mapping = stub_input_neuron_handle_mapping();
        let mut add_input_fn_was_called = false;
        let mut add_input_fn = |handle, input| {
            add_input_fn_was_called = true;
            assert_eq!(input_neuron_handle_mapping.vision[0], handle);
            assert_nearly_eq!(expected_input, input);
        };
        add_vision_inputs(
            distances.into_iter(),
            &input_neuron_handle_mapping,
            &mut add_input_fn,
        );

        assert!(
            add_input_fn_was_called,
            "add_input_fn was not called, but was expected"
        );
    }

    #[test]
    fn converts_multiple_distances_to_inputs() {
        let distances = vec![
            Some(MAX_DISTINGUISHABLE_DISTANCE_IN_METERS),
            None,
            Some(0.0),
        ];
        let input_neuron_handle_mapping = stub_input_neuron_handle_mapping();
        let mut inputs_were_added = vec![false; 3];
        let expected_inputs = vec![Some(0.0), None, Some(1.0)];

        let mut add_input_fn = |handle, input| {
            let vision_input_index = input_neuron_handle_mapping
                .vision
                .iter()
                .position(|&vision_handle| vision_handle == handle)
                .unwrap_or_else(|| {
                    panic!(
                        "add_input_fn was called with an unexpected vision handle: {:#?}",
                        handle
                    )
                });
            inputs_were_added[vision_input_index] = true;

            let expected_input = expected_inputs[vision_input_index].unwrap_or_else(|| {
                panic!(
                    "add_input_fn was called with a handle that is expected to receive no \
                     input.\nhandle: {:#?}\ninput: {}",
                    handle, input
                )
            });
            assert_nearly_eq!(expected_input, input);
        };
        add_vision_inputs(
            distances.into_iter(),
            &input_neuron_handle_mapping,
            &mut add_input_fn,
        );

        let expected_added_inputs = vec![true, false, true];

        assert_eq!(expected_added_inputs, inputs_were_added);
    }

    fn stub_input_neuron_handle_mapping() -> InputNeuronHandleMapping {
        InputNeuronHandleMapping {
            axial_acceleration: AxialAccelerationHandleMapping {
                forward: Handle(0),
                backward: Handle(1),
            },
            lateral_acceleration: LateralAccelerationHandleMapping {
                left: Handle(2),
                right: Handle(3),
            },
            vision: vec![Handle(4), Handle(5), Handle(6)],
        }
    }
}
