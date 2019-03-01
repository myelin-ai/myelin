use crate::*;

/// A developer for a flat neural network with no hidden layers.
/// Uses no actual genetics and sets all connection weights at 1.0
#[derive(Default, Debug, Clone)]
pub struct FlatNeuralNetworkDeveloper;

impl NeuralNetworkDeveloper for FlatNeuralNetworkDeveloper {
    fn develop_neural_network(
        &self,
        neural_network_development_configuration: NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork {
        let mut neural_network = box SpikingNeuralNetwork::default();

        let input_neuron_handles: Vec<Handle> = (0..neural_network_development_configuration
            .input_neuron_count)
            .map(|_| neural_network.push_neuron())
            .collect();

        let output_neuron_handles: Vec<Handle> = (0..neural_network_development_configuration
            .output_neuron_count)
            .map(|_| neural_network.push_neuron())
            .collect();

        for &input_neuron in input_neuron_handles.iter() {
            for &output_neuron in output_neuron_handles.iter() {
                neural_network
                    .add_connection(Connection {
                        from: input_neuron,
                        to: output_neuron,
                        weight: 1.0,
                    })
                    .expect("Internal error: Stored neuron handle was invalid");
            }
        }

        DevelopedNeuralNetwork {
            neural_network,
            genome: Genome {},
            input_neuron_handles,
            output_neuron_handles,
        }
    }
}
