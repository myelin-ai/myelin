//! Run neural-network/scripts/run_plot.sh in order to visualize the neurons' output

use maplit::hashmap;
use myelin_neural_network::spiking_neural_network::SpikingNeuralNetwork;
use myelin_neural_network::{
    Connection, Handle, MembranePotential, Milliseconds, NeuralNetwork, Weight,
};

fn main() {
    let mut neural_network = SpikingNeuralNetwork::default();
    let sensor_handle = neural_network.push_neuron();
    let neuron_handle = neural_network.push_neuron();
    let connection = Connection {
        from: Handle(sensor_handle.0),
        to: Handle(neuron_handle.0),
        weight: Weight(1.0),
    };
    neural_network.add_connection(connection).unwrap();

    let points: Vec<_> = (0..100)
        .map(|i| {
            let elapsed_time = Milliseconds(1.0);
            let inputs = hashmap! {
                sensor_handle => MembranePotential(-10.0)
            };
            neural_network.step(elapsed_time, &inputs);

            let sensor_membrane_potential = neural_network
                .membrane_potential_of_neuron(sensor_handle)
                .unwrap();
            let neuron_membrane_potential = neural_network
                .membrane_potential_of_neuron(neuron_handle)
                .unwrap();

            (i, sensor_membrane_potential, neuron_membrane_potential)
        })
        .collect();

    let sensor_points: Vec<_> = points
        .iter()
        .cloned()
        .filter_map(|(i, potential, _)| potential.map(|potential| (f64::from(i), potential.0)))
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();

    let neuron_points: Vec<_> = points
        .iter()
        .cloned()
        .filter_map(|(i, _, potential)| potential.map(|potential| (f64::from(i), potential.0)))
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();

    println!("Sensor\t{}", sensor_points.join("\t"));
    println!("Neuron\t{}", neuron_points.join("\t"));
}
