//! Default implementation of [`NeuralNetworkDevelopmentOrchestrator`].

use crate::deriver::GenomeDeriver;
use crate::mutator::GenomeMutator;
use crate::*;
use nameof::{name_of, name_of_type};
use std::fmt::{self, Debug};
use std::rc::Rc;

/// Provides a function that can be used to develop a neural network
pub trait NeuralNetworkDeveloper: Debug {
    /// Develops a neural network and writes it into a [`NeuralNetworkConfigurator`].
    fn develop_neural_network(self: Box<Self>, builder: &mut NeuralNetworkBuilder);
}

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[derive(Debug)]
pub struct NeuralNetworkBuilder {}

impl NeuralNetworkBuilder {
    /// Creates a new [`NeuralNetworkBuilder`] for a [`DevelopedNeuralNetwork`]
    pub fn new(developed_neural_network: DevelopedNeuralNetwork) -> Self {
        unimplemented!()
    }

    /// Add a new unconnected neuron to the network
    pub fn push_neuron(&mut self) -> Handle {
        unimplemented!();
    }

    /// Add a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    pub fn add_connection(&mut self, connection: Connection) -> Result<(), ()> {
        unimplemented!();
    }

    /// Marks a neuron as an input
    pub fn mark_neuron_as_input(&mut self, handle: Handle) -> Result<(), ()> {
        unimplemented!();
    }

    /// Marks a neuron as an output
    pub fn mark_neuron_as_output(&mut self, handle: Handle) -> Result<(), ()> {
        unimplemented!();
    }

    /// Consumes `self`, returning the built [`DevelopedNeuralNetwork`]
    pub fn build(self: Box<Self>) -> DevelopedNeuralNetwork {
        unimplemented!();
    }
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

/// Default implementation of a [`NeuralNetworkDevelopmentOrchestrator`]
#[derive(Clone)]
pub struct NeuralNetworkDevelopmentOrchestratorImpl {
    neural_network_factory: Rc<NeuralNetworkFactory>,
    neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
    genome_deriver: Box<dyn GenomeDeriver>,
    genome_mutator: Box<dyn GenomeMutator>,
}

impl NeuralNetworkDevelopmentOrchestratorImpl {
    /// Constructs a new [`NeuralNetworkDevelopmentOrchestratorImpl`]
    pub fn new(
        neural_network_factory: Rc<NeuralNetworkFactory>,
        neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
        genome_deriver: Box<dyn GenomeDeriver>,
        genome_mutator: Box<dyn GenomeMutator>,
    ) -> Self {
        Self {
            neural_network_factory,
            neural_network_developer_factory,
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
