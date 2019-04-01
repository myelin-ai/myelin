use crate::orchestrator_impl::NeuralNetworkConfigurator;
use crate::orchestrator_impl::{InputNeuronHandles, OutputNeuronHandles};
use myelin_neural_network::{Connection, Handle, NeuralNetwork};

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[derive(Debug)]
pub struct NeuralNetworkConfiguratorImpl<'a> {
    neural_network: &'a mut dyn NeuralNetwork,
    input_neuron_handles: &'a mut InputNeuronHandles,
    output_neuron_handles: &'a mut OutputNeuronHandles,
}

impl<'a> NeuralNetworkConfiguratorImpl<'a> {
    /// Creates a new [`NeuralNetworkBuilder`] for a [`DevelopedNeuralNetwork`]
    pub fn new(
        neural_network: &'a mut dyn NeuralNetwork,
        input_neuron_handles: &'a mut InputNeuronHandles,
        output_neuron_handles: &'a mut OutputNeuronHandles,
    ) -> Self {
        Self {
            neural_network,
            input_neuron_handles,
            output_neuron_handles,
        }
    }
}

impl NeuralNetworkConfigurator for NeuralNetworkConfiguratorImpl<'_> {
    fn push_neuron(&mut self) -> Handle {
        self.neural_network.push_neuron()
    }

    fn push_input_neuron(&mut self) -> Handle {
        let handle = self.push_neuron();
        self.input_neuron_handles.push(handle);
        handle
    }

    fn push_output_neuron(&mut self) -> Handle {
        let handle = self.push_neuron();
        self.output_neuron_handles.push(handle);
        handle
    }

    fn add_connection(&mut self, connection: Connection) -> Result<(), ()> {
        self.neural_network.add_connection(connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use myelin_neural_network::{NeuralNetwork, NeuralNetworkMock};

    #[test]
    fn new_does_not_mutate_neural_network() {
        let mut network: Box<dyn NeuralNetwork> = box NeuralNetworkMock::new();
        let mut input_neuron_handles = Vec::new();
        let mut output_neuron_handles = Vec::new();

        {
            let _ = NeuralNetworkConfiguratorImpl::new(
                &mut *network,
                &mut input_neuron_handles,
                &mut output_neuron_handles,
            );
        }

        assert!(input_neuron_handles.is_empty());
        assert!(output_neuron_handles.is_empty());
    }

    #[test]
    fn adds_neuron_to_network() {
        let expected_handle = Handle(42);

        let mut network: Box<dyn NeuralNetwork> = {
            let mut network = NeuralNetworkMock::new();
            network.expect_push_neuron().returns(expected_handle);
            box network
        };

        let mut input_neuron_handles = Vec::default();
        let mut output_neuron_handles = Vec::default();

        let mut configurator = NeuralNetworkConfiguratorImpl::new(
            &mut *network,
            &mut input_neuron_handles,
            &mut output_neuron_handles,
        );

        let handle = configurator.push_neuron();

        assert_eq!(expected_handle, handle);

        assert!(input_neuron_handles.is_empty());
        assert!(output_neuron_handles.is_empty());
    }

    #[test]
    fn adds_connection_to_network() {
        let connection = Connection {
            from: Handle(42),
            to: Handle(404),
            weight: 0.4,
        };

        let mut network: Box<dyn NeuralNetwork> = {
            let mut network = NeuralNetworkMock::new();
            network
                .expect_add_connection(partial_eq(connection.clone()))
                .returns(Ok(()));
            box network
        };

        let mut input_neuron_handles = Vec::default();
        let mut output_neuron_handles = Vec::default();

        let mut configurator = NeuralNetworkConfiguratorImpl::new(
            &mut *network,
            &mut input_neuron_handles,
            &mut output_neuron_handles,
        );

        let result = configurator.add_connection(connection);

        result.unwrap();

        assert!(input_neuron_handles.is_empty());
        assert!(output_neuron_handles.is_empty());
    }

    #[test]
    fn add_connection_propagates_error() {
        let connection = Connection {
            from: Handle(42),
            to: Handle(404),
            weight: 0.4,
        };

        let mut network: Box<dyn NeuralNetwork> = {
            let mut network = NeuralNetworkMock::new();
            network
                .expect_add_connection(partial_eq(connection.clone()))
                .returns(Err(()));
            box network
        };

        let mut input_neuron_handles = Vec::default();
        let mut output_neuron_handles = Vec::default();

        let mut configurator = NeuralNetworkConfiguratorImpl::new(
            &mut *network,
            &mut input_neuron_handles,
            &mut output_neuron_handles,
        );

        let result = configurator.add_connection(connection);

        result.unwrap_err();

        assert!(input_neuron_handles.is_empty());
        assert!(output_neuron_handles.is_empty());
    }

    #[test]
    fn adds_input_neuron() {
        let expected_handle = Handle(42);

        let mut network: Box<dyn NeuralNetwork> = {
            let mut network = NeuralNetworkMock::new();
            network.expect_push_neuron().returns(expected_handle);
            box network
        };

        let mut input_neuron_handles = Vec::default();
        let mut output_neuron_handles = Vec::default();

        let mut configurator = NeuralNetworkConfiguratorImpl::new(
            &mut *network,
            &mut input_neuron_handles,
            &mut output_neuron_handles,
        );

        let input_neuron = configurator.push_input_neuron();

        assert_eq!(1, input_neuron_handles.len());
        assert!(output_neuron_handles.is_empty());

        assert_eq!(vec![expected_handle], input_neuron_handles);
        assert_eq!(expected_handle, input_neuron);
    }

    #[test]
    fn adds_output_neuron() {
        let expected_handle = Handle(42);

        let mut network: Box<dyn NeuralNetwork> = {
            let mut network = NeuralNetworkMock::new();
            network.expect_push_neuron().returns(expected_handle);
            box network
        };

        let mut input_neuron_handles = Vec::default();
        let mut output_neuron_handles = Vec::default();

        let mut configurator = NeuralNetworkConfiguratorImpl::new(
            &mut *network,
            &mut input_neuron_handles,
            &mut output_neuron_handles,
        );

        let output_neuron = configurator.push_output_neuron();

        assert!(input_neuron_handles.is_empty());
        assert_eq!(1, output_neuron_handles.len());

        assert_eq!(vec![expected_handle], output_neuron_handles);
        assert_eq!(expected_handle, output_neuron);
    }
}
