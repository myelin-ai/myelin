use crate::orchestrator_impl::{NeuralNetworkConfigurator, NeuralNetworkDeveloper};
use crate::*;

/// A developer for a flat neural network with no hidden layers.
/// Uses no actual genetics and sets all connection weights at 1.0
#[derive(Debug, Clone)]
pub struct FlatNeuralNetworkDeveloper<'a> {
    configuration: &'a NeuralNetworkDevelopmentConfiguration,
}

impl<'a> FlatNeuralNetworkDeveloper<'a> {
    /// Constructs a new [`FlatNeuralNetworkDeveloper`]
    ///
    /// [`FlatNeuralNetworkDeveloper `]: ./struct.FlatNeuralNetworkDeveloper.html
    pub fn new(
        neural_network_development_configuration: &'a NeuralNetworkDevelopmentConfiguration,
    ) -> Self {
        Self {
            configuration: neural_network_development_configuration,
        }
    }
}

impl NeuralNetworkDeveloper for FlatNeuralNetworkDeveloper<'_> {
    fn develop_neural_network(self: Box<Self>, configurator: &mut dyn NeuralNetworkConfigurator) {
        let input_neuron_count = self.configuration.input_neuron_count.get();
        let output_neuron_count = self.configuration.output_neuron_count.get();

        let input_neuron_handles: Vec<Handle> = (0..input_neuron_count)
            .map(|_| configurator.push_input_neuron())
            .collect();

        let output_neuron_handles: Vec<Handle> = (0..output_neuron_count)
            .map(|_| configurator.push_output_neuron())
            .collect();

        for input_neuron in input_neuron_handles {
            for &output_neuron in output_neuron_handles.iter() {
                configurator
                    .add_connection(Connection {
                        from: input_neuron,
                        to: output_neuron,
                        weight: 1.0,
                    })
                    .expect("Internal error: Stored neuron handle was invalid");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestrator_impl::NeuralNetworkConfiguratorMock;
    use mockiato::partial_eq;

    #[test]
    fn develops_correct_number_of_input_and_output_neurons() {
        let configuration = configuration();
        let input_neuron_count = configuration.input_neuron_count.get() as u64;
        let output_neuron_count = configuration.output_neuron_count.get() as u64;
        let connection_count = input_neuron_count * output_neuron_count;

        let mut configurator = {
            let mut configurator = NeuralNetworkConfiguratorMock::new();
            configurator
                .expect_push_input_neuron()
                .times(input_neuron_count)
                .returns(Handle(1));
            configurator
                .expect_push_output_neuron()
                .times(output_neuron_count)
                .returns(Handle(2));
            configurator
                .expect_add_connection(partial_eq(Connection {
                    from: Handle(1),
                    to: Handle(2),
                    weight: 1.0,
                }))
                .times(connection_count)
                .returns(Ok(()));
            box configurator
        };
        let developer = box FlatNeuralNetworkDeveloper::new(&configuration);

        developer.develop_neural_network(&mut *configurator);
    }

    fn configuration() -> NeuralNetworkDevelopmentConfiguration {
        NeuralNetworkDevelopmentConfiguration {
            parent_genomes: (Genome::default(), Genome::default()),
            input_neuron_count: NonZeroUsize::new(3).unwrap(),
            output_neuron_count: NonZeroUsize::new(5).unwrap(),
        }
    }
}
