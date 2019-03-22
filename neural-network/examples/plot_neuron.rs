//! Run neural-network/scripts/plot.py in order to visualize the neurons' output

#![feature(box_syntax)]

use maplit::hashmap;
use myelin_neural_network::spiking_neural_network::DefaultSpikingNeuralNetwork;
use myelin_neural_network::{Connection, Handle, MembranePotential, Milliseconds, NeuralNetwork};
use std::collections::HashMap;

fn main() {
    const TIME_TO_SIMULATE: Milliseconds = 10.0;
    const TIMESTEP: Milliseconds = 0.001;
    const IS_INPUT_CONSTANT: bool = true;

    let mut neural_network = box DefaultSpikingNeuralNetwork::default();
    let sensor_handle = neural_network.push_neuron();
    let neuron_handle = neural_network.push_neuron();
    let connection = Connection {
        from: Handle(sensor_handle.0),
        to: Handle(neuron_handle.0),
        weight: 1.0,
    };

    neural_network.add_connection(connection).unwrap();

    /// constant duplicated because the one used internally is pub(crate)
    pub(crate) const THRESHOLD_POTENTIAL: MembranePotential = -55.0;

    let threshold_inputs = hashmap! {
        sensor_handle => THRESHOLD_POTENTIAL
    };
    neural_network.step(TIMESTEP, &threshold_inputs);

    let steps = f64::ceil(TIME_TO_SIMULATE / TIMESTEP) as u32;
    let points: Vec<_> = (0..steps)
        .map(|i| {
            let inputs = if IS_INPUT_CONSTANT {
                threshold_inputs.clone()
            } else {
                HashMap::new()
            };
            neural_network.step(TIMESTEP, &inputs);

            let sensor_membrane_potential = neural_network
                .membrane_potential_of_neuron(sensor_handle)
                .unwrap();
            let neuron_membrane_potential = neural_network
                .membrane_potential_of_neuron(neuron_handle)
                .unwrap();

            (
                f64::from(i) * TIMESTEP,
                sensor_membrane_potential,
                neuron_membrane_potential,
            )
        })
        .collect();

    let sensor_points: Vec<_> = points
        .iter()
        .cloned()
        .map(|(i, potential, _)| {
            potential
                .map(|potential| (i, potential))
                .unwrap_or((i, -55.0))
        })
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();

    let neuron_points: Vec<_> = points
        .iter()
        .cloned()
        .map(|(i, _, potential)| {
            potential
                .map(|potential| (i, potential))
                .unwrap_or((i, -55.0))
        })
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();

    const TAB_ESCAPE_SEQUENCE: &str = "\t";
    println!(
        "Sensor{}{}",
        TAB_ESCAPE_SEQUENCE,
        sensor_points.join(TAB_ESCAPE_SEQUENCE)
    );
    println!(
        "Neuron{}{}",
        TAB_ESCAPE_SEQUENCE,
        neuron_points.join(TAB_ESCAPE_SEQUENCE)
    );
}
