//! Run neural-network/scripts/run_plot.sh in order to visualize the neurons' output

use maplit::hashmap;
use myelin_neural_network::spiking_neural_network::SpikingNeuralNetwork;
use myelin_neural_network::{
    Connection, Handle, MembranePotential, Milliseconds, NeuralNetwork, Weight,
};
use std::collections::HashMap;

fn main() {
    const TIME_TO_SIMULATE: Milliseconds = Milliseconds(10.0);
    const TIMESTEP: Milliseconds = Milliseconds(0.001);
    const CONSTANT_INPUT: bool = true;

    let mut neural_network = SpikingNeuralNetwork::default();
    let sensor_handle = neural_network.push_neuron();
    let neuron_handle = neural_network.push_neuron();
    let connection = Connection {
        from: Handle(sensor_handle.0),
        to: Handle(neuron_handle.0),
        weight: Weight(1.0),
    };

    neural_network.add_connection(connection).unwrap();
    let threshold_inputs = hashmap! {
        sensor_handle => MembranePotential(-55.0)
    };
    neural_network.step(TIMESTEP, &threshold_inputs);

    let steps = f64::ceil(TIME_TO_SIMULATE.0 / TIMESTEP.0) as u32;
    let points: Vec<_> = (0..steps)
        .map(|i| {
            let inputs = if CONSTANT_INPUT {
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
                f64::from(i) * TIMESTEP.0,
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
                .map(|potential| (i, potential.0))
                .unwrap_or((i, -55.0))
        })
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();

    let neuron_points: Vec<_> = points
        .iter()
        .cloned()
        .map(|(i, _, potential)| {
            potential
                .map(|potential| (i, potential.0))
                .unwrap_or((i, -55.0))
        })
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();

    println!("Sensor\t{}", sensor_points.join("\t"));
    println!("Neuron\t{}", neuron_points.join("\t"));
}
