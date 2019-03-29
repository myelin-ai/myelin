use crate::orchestrator_impl::NeuralNetworkConfigurator;
use crate::DevelopedNeuralNetwork;
use myelin_neural_network::{Connection, Handle};

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[derive(Debug)]
pub struct NeuralNetworkConfiguratorImpl<'a> {
    developed_neural_network: &'a mut DevelopedNeuralNetwork,
}

impl<'a> NeuralNetworkConfiguratorImpl<'a> {
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
    use crate::genome::Genome;
    use mockiato::partial_eq;
    use myelin_neural_network::NeuralNetworkMock;

    #[test]
    fn new_does_nothing() {
        let network = NeuralNetworkMock::new();

        let genome = Genome::default();

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box network.clone(),
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
        assert_eq!(0, developed_network.output_neuron_handles.len());
    }

    #[test]
    fn push_neuron_adds_neuron_to_network() {
        let expected_handle = Handle(42);

        let mut network = NeuralNetworkMock::new();
        network.expect_push_neuron().returns(expected_handle);

        let genome = Genome::default();

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let handle = configurator.push_neuron();

        assert_eq!(expected_handle, handle);

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
        assert_eq!(0, developed_network.output_neuron_handles.len());
    }

    #[test]
    fn add_connection_adds_connection_to_network() {
        let connection = Connection {
            from: Handle(42),
            to: Handle(404),
            weight: 0.4,
        };

        let mut network = NeuralNetworkMock::new();
        network
            .expect_add_connection(partial_eq(connection.clone()))
            .returns(Ok(()));

        let genome = Genome::default();

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let result = configurator.add_connection(connection);

        assert!(result.is_ok());

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
        assert_eq!(0, developed_network.output_neuron_handles.len());
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

        let genome = Genome::default();

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let result = configurator.add_connection(connection);

        assert!(result.is_err());

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
        assert_eq!(0, developed_network.output_neuron_handles.len());
    }

    #[test]
    fn mark_neuron_as_input_adds_handle_to_input_neurons() {
        let expected_handle = Handle(42);

        let mut network = NeuralNetworkMock::new();
        network.expect_push_neuron().returns(expected_handle);

        let genome = Genome::default();

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let input_neuron = configurator.push_input_neuron();

        assert_eq!(1, developed_network.input_neuron_handles.len());
        assert_eq!(
            &expected_handle,
            developed_network.input_neuron_handles.get(0).unwrap()
        );
        assert_eq!(expected_handle, input_neuron);

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.output_neuron_handles.len());
    }

    #[test]
    fn mark_neuron_as_output_adds_handle_to_output_neurons() {
        let expected_handle = Handle(42);

        let mut network = NeuralNetworkMock::new();
        network.expect_push_neuron().returns(expected_handle);

        let genome = Genome::default();

        let mut developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut configurator = NeuralNetworkConfiguratorImpl::new(&mut developed_network);

        let output_neuron = configurator.push_output_neuron();

        assert_eq!(1, developed_network.output_neuron_handles.len());
        assert_eq!(
            &expected_handle,
            developed_network.output_neuron_handles.get(0).unwrap()
        );
        assert_eq!(expected_handle, output_neuron);

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
    }
}
