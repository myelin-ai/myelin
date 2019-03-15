use crate::*;
use nameof::name_of_type;
use std::fmt::{self, Debug};
use std::rc::Rc;

/// A developer for a flat neural network with no hidden layers.
/// Uses no actual genetics and sets all connection weights at 1.0
#[derive(Clone)]
pub struct FlatNeuralNetworkDeveloper {
    neural_network_factory: Rc<NeuralNetworkFactory>,
}

/// A factory for building a [`NeuralNetwork`]
///
/// [`NeuralNetwork`]: ../../../myelin-neural-network/trait.NeuralNetwork.html
pub type NeuralNetworkFactory = dyn Fn() -> Box<dyn NeuralNetwork>;

impl FlatNeuralNetworkDeveloper {
    /// Constructs a new `FlatNeuralNetworkDeveloper`
    pub fn new(neural_network_factory: Rc<NeuralNetworkFactory>) -> Self {
        Self {
            neural_network_factory,
        }
    }
}

impl Debug for FlatNeuralNetworkDeveloper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(FlatNeuralNetworkDeveloper))
            .finish()
    }
}

impl NeuralNetworkDeveloper for FlatNeuralNetworkDeveloper {
    fn develop_neural_network(
        &self,
        neural_network_development_configuration: &NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork {
        let mut neural_network = (self.neural_network_factory)();

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
            genome: Genome::default(),
            input_neuron_handles,
            output_neuron_handles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_neural_network::spiking_neural_network::{SpikingNeuralNetwork, SpikingNeuronImpl};

    #[test]
    fn develops_correct_number_of_input_neurons() {
        let configuration = configuration();
        let neural_network_factory = neural_network_factory();
        let developer = FlatNeuralNetworkDeveloper::new(neural_network_factory);
        let neural_network = developer.develop_neural_network(&configuration);

        assert_eq!(
            configuration.input_neuron_count,
            neural_network.input_neuron_handles.len()
        );
    }

    #[test]
    fn develops_correct_number_of_output_neurons() {
        let configuration = configuration();
        let neural_network_factory = neural_network_factory();
        let developer = FlatNeuralNetworkDeveloper::new(neural_network_factory);
        let developed_neural_network = developer.develop_neural_network(&configuration);

        assert_eq!(
            configuration.output_neuron_count,
            developed_neural_network.output_neuron_handles.len()
        );
    }

    fn configuration() -> NeuralNetworkDevelopmentConfiguration {
        NeuralNetworkDevelopmentConfiguration {
            parent_genomes: (Genome, Genome),
            input_neuron_count: 3,
            output_neuron_count: 5,
        }
    }

    fn neural_network_factory() -> Rc<NeuralNetworkFactory> {
        Rc::new(|| box SpikingNeuralNetwork::<SpikingNeuronImpl>::default())
    }
}
