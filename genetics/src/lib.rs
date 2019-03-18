//! Genes, genomes and the mechanisms needed to evolve neural networks from them

#![feature(specialization)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented,
    clippy::default_trait_access
)]

#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use myelin_neural_network::{Handle as NeuronHandle, NeuralNetwork};
use std::fmt::Debug;

/// The set of all genes in an organism
#[derive(Debug, Clone)]
pub struct Genome;

/// Information needed by a [`NeuralNetworkDeveloper`] to build a [`DevelopedNeuralNetwork`]
///
/// [`NeuralNetworkDeveloper`]: ./trait.NeuralNetworkDeveloper.html
/// [`DevelopedNeuralNetwork`]: ./struct.DevelopedNeuralNetwork.html
#[derive(Debug, Clone)]
pub struct NeuralNetworkDevelopmentConfiguration {
    /// The genomes that will be combined to form a new genome for this neural network.
    /// Will result in [`DevelopedNeuralNetwork.genome`].
    ///
    /// [`DevelopedNeuralNetwork.genome`]: ./struct.DevelopedNeuralNetwork.html#structfield.genome
    pub parent_genomes: (Genome, Genome),

    /// The number of neurons that shall receive inputs.
    /// Will result in [`DevelopedNeuralNetwork.input_neuron_handles`].
    ///
    /// [`DevelopedNeuralNetwork.input_neuron_handles`]: ./struct.DevelopedNeuralNetwork.html#structfield.input_neuron_handles
    pub input_neuron_count: u32,

    /// The number of neurons that shall emit outputs
    /// Will result in [`DevelopedNeuralNetwork.output_neuron_handles`].
    ///
    /// [`DevelopedNeuralNetwork.output_neuron_handles`]: ./struct.DevelopedNeuralNetwork.html#structfield.output_neuron_handles
    pub output_neuron_count: u32,
}

/// [`NeuralNetwork`] and auxillary data developed by a [`NeuralNetworkDeveloper`].
///
/// [`NeuralNetworkDeveloper`]: trait.NeuralNetworkDeveloper.html
/// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
#[derive(Debug, Clone)]
pub struct DevelopedNeuralNetwork {
    /// The generated [`NeuralNetwork`]
    ///
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    pub neural_network: Box<dyn NeuralNetwork>,

    /// The generated [`Genome`], originating from the union of
    /// [`NeuralNetworkDevelopmentConfiguration.parent_genomes`].
    ///
    /// [`Genome`]: ./struct.Genome.html
    /// [`NeuralNetworkDevelopmentConfiguration.parent_genomes`]: ./struct.NeuralNetworkDevelopmentConfiguration.html#structfield.parent_genomes
    pub genome: Genome,

    /// The handles to generated neurons that can accept inputs, originating from
    /// [`NeuralNetworkDevelopmentConfiguration.input_neuron_count`].
    ///
    /// [`NeuralNetworkDevelopmentConfiguration.input_neuron_count`]: ./struct.NeuralNetworkDevelopmentConfiguration.html#structfield.input_neuron_count
    pub input_neuron_handles: Vec<NeuronHandle>,

    /// The handles to generated neurons that emit outputs, originating from
    /// [`NeuralNetworkDevelopmentConfiguration.output_neuron_count`].
    ///
    /// [`NeuralNetworkDevelopmentConfiguration.output_neuron_count`]: ./struct.NeuralNetworkDevelopmentConfiguration.html#structfield.output_neuron_count
    pub output_neuron_handles: Vec<NeuronHandle>,
}

/// A factory for producing a [`NeuralNetwork`] out of a [`Genome`]
///
/// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
/// [`Genome`]: ./struct.Genome.html
#[cfg_attr(any(test, feature = "use-mocks"), mockable(static_references))]
pub trait NeuralNetworkDeveloper: Debug + NeuralNetworkDeveloperClone {
    /// Create a [`DevelopedNeuralNetwork`] using the information contained in the provided [`NeuralNetworkDevelopmentConfiguration`]
    ///
    /// [`DevelopedNeuralNetwork`]: ./struct.DevelopedNeuralNetwork.html
    /// [`NeuralNetworkDevelopmentConfiguration`]: ./struct.NeuralNetworkDevelopmentConfiguration.html
    fn develop_neural_network(
        &self,
        neural_network_development_configuration: NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork;
}

/// Supertrait used to make sure that all implementors
/// of [`NeuralNetworkDeveloper`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`NeuralNetworkDeveloper`]: ./trait.NeuralNetworkDeveloper.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait NeuralNetworkDeveloperClone {
    fn clone_box(&self) -> Box<dyn NeuralNetworkDeveloper>;
}

impl<T> NeuralNetworkDeveloperClone for T
where
    T: NeuralNetworkDeveloper + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn NeuralNetworkDeveloper> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn NeuralNetworkDeveloper> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
