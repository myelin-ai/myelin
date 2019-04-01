use crate::orchestrator_impl::NeuralNetworkConfigurator;
use crate::DevelopedNeuralNetwork;
use myelin_neural_network::{Connection, Handle};

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[derive(Debug)]
pub struct NeuralNetworkConfiguratorImpl<'a> {
    developed_neural_network: &'a mut DevelopedNeuralNetwork,
}

impl<'a> NeuralNetworkConfiguratorImpl<'a> {
    /// Creates a new [`NeuralNetworkBuilder`] for a [`DevelopedNeuralNetwork`]
    pub fn new(developed_neural_network: &'a mut DevelopedNeuralNetwork) -> Self {
        Self {
            developed_neural_network,
        }
    }
}

impl NeuralNetworkConfigurator for NeuralNetworkConfiguratorImpl<'_> {
    fn push_neuron(&mut self) -> Handle {
        self.developed_neural_network.neural_network.push_neuron()
    }

    fn push_input_neuron(&mut self) -> Handle {
        let handle = self.push_neuron();
        self.developed_neural_network
            .input_neuron_handles
            .push(handle);
        handle
    }

    fn push_output_neuron(&mut self) -> Handle {
        let handle = self.push_neuron();
        self.developed_neural_network
            .output_neuron_handles
            .push(handle);
        handle
    }

    fn add_connection(&mut self, connection: Connection) -> Result<(), ()> {
        self.developed_neural_network
            .neural_network
            .add_connection(connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::{
        ClusterGene, ClusterGeneIndex, Connection as GenomeConnection, Genome, HoxGene,
        HoxPlacement, Neuron, NeuronClusterLocalIndex,
    };
    use mockiato::partial_eq;
    use myelin_neural_network::{NeuralNetwork, NeuralNetworkMock};

    fn create_developed_network(neural_network: Box<dyn NeuralNetwork>) -> DevelopedNeuralNetwork {
        DevelopedNeuralNetwork {
            neural_network,
            genome: Genome::default(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        }
    }

    #[test]
    fn new_does_not_mutate_neural_network() {
        let network = NeuralNetworkMock::new();

        let mut developed_network = create_developed_network(box network);

        NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        assert!(developed_network.input_neuron_handles.is_empty());
        assert!(developed_network.output_neuron_handles.is_empty());
    }

    #[test]
    fn adds_neuron_to_network() {
        let expected_handle = Handle(42);

        let mut network = NeuralNetworkMock::new();
        network.expect_push_neuron().returns(expected_handle);

        let mut developed_network = create_developed_network(box network);

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let handle = configurator.push_neuron();

        assert_eq!(expected_handle, handle);

        assert!(developed_network.input_neuron_handles.is_empty());
        assert!(developed_network.output_neuron_handles.is_empty());
    }

    #[test]
    fn adds_connection_to_network() {
        let connection = Connection {
            from: Handle(42),
            to: Handle(404),
            weight: 0.4,
        };

        let mut network = NeuralNetworkMock::new();
        network
            .expect_add_connection(partial_eq(connection.clone()))
            .returns(Ok(()));

        let mut developed_network = create_developed_network(box network);

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let result = configurator.add_connection(connection);

        result.unwrap();

        assert!(developed_network.input_neuron_handles.is_empty());
        assert!(developed_network.output_neuron_handles.is_empty());
    }

    #[test]
    fn add_connection_propagates_error() {
        let connection = Connection {
            from: Handle(42),
            to: Handle(404),
            weight: 0.4,
        };

        let mut network = NeuralNetworkMock::new();
        network
            .expect_add_connection(partial_eq(connection.clone()))
            .returns(Err(()));

        let mut developed_network = create_developed_network(box network);

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let result = configurator.add_connection(connection);

        result.unwrap_err();

        assert!(developed_network.input_neuron_handles.is_empty());
        assert!(developed_network.output_neuron_handles.is_empty());
    }

    #[test]
    fn adds_input_neuron() {
        let expected_handle = Handle(42);

        let mut network = NeuralNetworkMock::new();
        network.expect_push_neuron().returns(expected_handle);

        let mut developed_network = create_developed_network(box network);

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let input_neuron = configurator.push_input_neuron();

        assert_eq!(1, developed_network.input_neuron_handles.len());
        assert!(developed_network.output_neuron_handles.is_empty());

        assert_eq!(
            &expected_handle,
            developed_network.input_neuron_handles.get(0).unwrap()
        );
        assert_eq!(expected_handle, input_neuron);
    }

    #[test]
    fn adds_output_neuron() {
        let expected_handle = Handle(42);

        let mut network = NeuralNetworkMock::new();
        network.expect_push_neuron().returns(expected_handle);

        let mut developed_network = create_developed_network(box network);

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let output_neuron = configurator.push_output_neuron();

        assert!(developed_network.input_neuron_handles.is_empty());
        assert_eq!(1, developed_network.output_neuron_handles.len());

        assert_eq!(
            &expected_handle,
            developed_network.output_neuron_handles.get(0).unwrap()
        );
        assert_eq!(expected_handle, output_neuron);
    }

    #[test]
    fn genome_is_never_modified() {
        let mut neural_network = NeuralNetworkMock::new();
        neural_network
            .expect_push_neuron()
            .returns(Handle(42))
            .times(3);

        let genome = Genome {
            cluster_genes: vec![ClusterGene {
                placement_neuron: NeuronClusterLocalIndex(0),
                neurons: vec![Neuron {}, Neuron {}],
                connections: vec![GenomeConnection {
                    from: NeuronClusterLocalIndex(0),
                    to: NeuronClusterLocalIndex(1),
                    weight: 1.0,
                }],
            }],
            hox_genes: vec![HoxGene {
                placement: HoxPlacement::Standalone,
                cluster_index: ClusterGeneIndex(1),
                disabled_connections: Vec::new(),
            }],
        };

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box neural_network,
            genome: genome.clone(),
            input_neuron_handles: vec![Handle(2), Handle(4)],
            output_neuron_handles: vec![Handle(7), Handle(9)],
        };

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        configurator.push_neuron();
        configurator.push_input_neuron();
        configurator.push_output_neuron();

        assert_eq!(genome, developed_network.genome);
        assert_eq!(
            vec![Handle(2), Handle(4), Handle(42)],
            developed_network.input_neuron_handles
        );
        assert_eq!(
            vec![Handle(7), Handle(9), Handle(42)],
            developed_network.output_neuron_handles
        );
    }
}
