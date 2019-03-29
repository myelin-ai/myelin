use crate::DevelopedNeuralNetwork;
use myelin_neural_network::{Connection, Handle};

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[derive(Debug)]
pub struct NeuralNetworkConfiguratorImpl {
    developed_neural_network: DevelopedNeuralNetwork,
}

impl NeuralNetworkConfiguratorImpl {
    /// Creates a new [`NeuralNetworkBuilder`] for a [`DevelopedNeuralNetwork`]
    pub fn new(developed_neural_network: DevelopedNeuralNetwork) -> Self {
        Self {
            developed_neural_network,
        }
    }

    /// Adds a new unconnected neuron to the network
    pub fn push_neuron(&mut self) -> Handle {
        self.developed_neural_network.neural_network.push_neuron()
    }

    /// Adds a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    pub fn add_connection(&mut self, connection: Connection) -> Result<(), ()> {
        self.developed_neural_network
            .neural_network
            .add_connection(connection)
    }

    /// Marks a neuron as an input
    pub fn mark_neuron_as_input(&mut self, handle: Handle) {
        self.developed_neural_network
            .input_neuron_handles
            .push(handle)
    }

    /// Marks a neuron as an output
    pub fn mark_neuron_as_output(&mut self, handle: Handle) {
        self.developed_neural_network
            .output_neuron_handles
            .push(handle);
    }

    /// Consumes `self`, returning the built [`DevelopedNeuralNetwork`]
    pub fn build(self) -> DevelopedNeuralNetwork {
        self.developed_neural_network
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::Genome;
    use mockiato::partial_eq;
    use myelin_neural_network::NeuralNetworkMock;

    #[test]
    fn new_and_build_does_nothing() {
        let network = NeuralNetworkMock::new();

        let genome = Genome::default();

        let developed_network = DevelopedNeuralNetwork {
            neural_network: box network.clone(),
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let builder = NeuralNetworkConfiguratorImpl::new(developed_network);

        let developed_network = builder.build();

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

        let developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut builder = NeuralNetworkConfiguratorImpl::new(developed_network);

        let handle = builder.push_neuron();

        let developed_network = builder.build();

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

        let developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut builder = NeuralNetworkConfiguratorImpl::new(developed_network);

        let result = builder.add_connection(connection);

        let developed_network = builder.build();

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

        let developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut builder = NeuralNetworkConfiguratorImpl::new(developed_network);

        let result = builder.add_connection(connection);

        let developed_network = builder.build();

        assert!(result.is_err());

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
        assert_eq!(0, developed_network.output_neuron_handles.len());
    }

    #[test]
    fn mark_neuron_as_input_adds_handle_to_input_neurons() {
        let expected_handle = Handle(42);

        let network = NeuralNetworkMock::new();

        let genome = Genome::default();

        let developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut builder = NeuralNetworkConfiguratorImpl::new(developed_network);

        builder.mark_neuron_as_input(expected_handle);

        let developed_network = builder.build();

        assert_eq!(1, developed_network.input_neuron_handles.len());
        assert_eq!(
            &expected_handle,
            developed_network.input_neuron_handles.get(0).unwrap()
        );

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.output_neuron_handles.len());
    }

    #[test]
    fn mark_neuron_as_output_adds_handle_to_output_neurons() {
        let expected_handle = Handle(42);

        let network = NeuralNetworkMock::new();

        let genome = Genome::default();

        let developed_network = DevelopedNeuralNetwork {
            neural_network: box network,
            genome: genome.clone(),
            input_neuron_handles: Vec::default(),
            output_neuron_handles: Vec::default(),
        };

        let mut builder = NeuralNetworkConfiguratorImpl::new(developed_network);

        builder.mark_neuron_as_output(expected_handle);

        let developed_network = builder.build();

        assert_eq!(1, developed_network.output_neuron_handles.len());
        assert_eq!(
            &expected_handle,
            developed_network.output_neuron_handles.get(0).unwrap()
        );

        assert_eq!(genome, developed_network.genome);
        assert_eq!(0, developed_network.input_neuron_handles.len());
    }
}
