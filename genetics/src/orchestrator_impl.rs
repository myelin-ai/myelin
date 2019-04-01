//! Default implementation of [`NeuralNetworkDevelopmentOrchestrator`].

pub use self::NeuralNetworkConfigurator;
use crate::deriver::GenomeDeriver;
use crate::mutator::GenomeMutator;
use crate::*;
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use nameof::{name_of, name_of_type};
use std::fmt::{self, Debug};
use std::rc::Rc;

mod neural_network_builder;

/// Provides a function that can be used to develop a neural network
pub trait NeuralNetworkDeveloper: Debug {
    /// Develops a neural network and writes it into a [`NeuralNetworkConfigurator`].
    fn develop_neural_network(self: Box<Self>, configurator: &mut dyn NeuralNetworkConfigurator);
}

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait NeuralNetworkConfigurator: Debug {
    /// Adds a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle;

    /// Adds a new unconnected neuron to the network and marks is as an input
    fn push_input_neuron(&mut self) -> Handle;

    /// Adds a new unconnected neuron to the network and marks is as an input
    fn push_output_neuron(&mut self) -> Handle;

    /// Adds a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<(), ()>;
}

/// A factory for building a [`NeuralNetwork`]
///
/// [`NeuralNetwork`]: ../../../myelin-neural-network/trait.NeuralNetwork.html
pub type NeuralNetworkFactory = dyn Fn() -> Box<dyn NeuralNetwork>;

/// Creates a new [`NeuralNetworkDeveloper`]
pub type NeuralNetworkDeveloperFactory = dyn for<'a> Fn(
    &'a NeuralNetworkDevelopmentConfiguration,
    Genome,
) -> Box<dyn NeuralNetworkDeveloper + 'a>;

/// Creates a new [`NeuralNetworkConfigurator`]
pub type NeuralNetworkConfiguratorFactory =
    dyn for<'a> Fn(&'a mut DevelopedNeuralNetwork) -> Box<dyn NeuralNetworkConfigurator + 'a>;

/// Default implementation of a [`NeuralNetworkDevelopmentOrchestrator`]
#[derive(Clone)]
pub struct NeuralNetworkDevelopmentOrchestratorImpl {
    neural_network_factory: Rc<NeuralNetworkFactory>,
    neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
    neural_network_configurator_factory: Rc<NeuralNetworkConfiguratorFactory>,
    genome_deriver: Box<dyn GenomeDeriver>,
    genome_mutator: Box<dyn GenomeMutator>,
}

impl NeuralNetworkDevelopmentOrchestratorImpl {
    /// Constructs a new [`NeuralNetworkDevelopmentOrchestratorImpl`]
    pub fn new(
        neural_network_factory: Rc<NeuralNetworkFactory>,
        neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
        neural_network_configurator_factory: Rc<NeuralNetworkConfiguratorFactory>,
        genome_deriver: Box<dyn GenomeDeriver>,
        genome_mutator: Box<dyn GenomeMutator>,
    ) -> Self {
        Self {
            neural_network_factory,
            neural_network_developer_factory,
            neural_network_configurator_factory,
            genome_deriver,
            genome_mutator,
        }
    }
}

impl Debug for NeuralNetworkDevelopmentOrchestratorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(NeuralNetworkDevelopmentOrchestratorImpl))
            .field(name_of!(genome_deriver in Self), &self.genome_deriver)
            .field(name_of!(genome_mutator in Self), &self.genome_mutator)
            .finish()
    }
}

impl NeuralNetworkDevelopmentOrchestrator for NeuralNetworkDevelopmentOrchestratorImpl {
    fn develop_neural_network(
        &self,
        _neural_network_development_configuration: &NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork {
        unimplemented!();
    }
}
