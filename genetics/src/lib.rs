//! Genes, genomes and the mechanisms needed to evolve neural networks from them

#![feature(specialization)]
#![feature(box_syntax)]
#![warn(missing_docs, clippy::dbg_macro)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

use crate::genome::Genome;
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use myelin_clone_box::clone_box;
use myelin_neural_network::{Connection, Handle, NeuralNetwork};
use std::fmt::Debug;
use std::num::NonZeroUsize;

pub mod genome;
pub mod neural_network_development_orchestrator_impl;
pub mod genome_generator_impl;

mod constant;

/// Information needed by a [`NeuralNetworkDeveloper`] to build a [`DevelopedNeuralNetwork`]
///
/// [`NeuralNetworkDeveloper`]: ./trait.NeuralNetworkDeveloper.html
/// [`DevelopedNeuralNetwork`]: ./struct.DevelopedNeuralNetwork.html
#[derive(Debug, Clone, PartialEq)]
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
    pub input_neuron_count: NonZeroUsize,

    /// The number of neurons that shall emit outputs
    /// Will result in [`DevelopedNeuralNetwork.output_neuron_handles`].
    ///
    /// [`DevelopedNeuralNetwork.output_neuron_handles`]: ./struct.DevelopedNeuralNetwork.html#structfield.output_neuron_handles
    pub output_neuron_count: NonZeroUsize,
}

/// [`NeuralNetwork`] and auxiliary data developed by a [`NeuralNetworkDeveloper`].
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
    pub input_neuron_handles: Vec<Handle>,

    /// The handles to generated neurons that emit outputs, originating from
    /// [`NeuralNetworkDevelopmentConfiguration.output_neuron_count`].
    ///
    /// [`NeuralNetworkDevelopmentConfiguration.output_neuron_count`]: ./struct.NeuralNetworkDevelopmentConfiguration.html#structfield.output_neuron_count
    pub output_neuron_handles: Vec<Handle>,
}

/// A factory for producing a [`NeuralNetwork`] out of a [`Genome`]
///
/// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
/// [`Genome`]: ./struct.Genome.html
#[cfg_attr(any(test, feature = "use-mocks"), mockable(static_references))]
pub trait NeuralNetworkDevelopmentOrchestrator:
    Debug + NeuralNetworkDevelopmentOrchestratorClone
{
    /// Create a [`DevelopedNeuralNetwork`] using the information contained in the provided [`NeuralNetworkDevelopmentConfiguration`]
    ///
    /// [`DevelopedNeuralNetwork`]: ./struct.DevelopedNeuralNetwork.html
    /// [`NeuralNetworkDevelopmentConfiguration`]: ./struct.NeuralNetworkDevelopmentConfiguration.html
    fn develop_neural_network(
        &self,
        neural_network_development_configuration: &NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork;
}

clone_box!(
    NeuralNetworkDevelopmentOrchestrator,
    NeuralNetworkDevelopmentOrchestratorClone
);

/// Configuration for [`GenomeGenerator::generate_genome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenomeGeneratorConfiguration {
    /// The number of neurons that shall receive inputs.
    pub input_neuron_count: NonZeroUsize,

    /// The number of neurons that shall emit outputs
    pub output_neuron_count: NonZeroUsize,
}

/// A factory for producing a new genome
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait GenomeGenerator {
    /// Generates a new genome from scratch according to the given configuration.
    fn generate_genome(&self, configuration: &GenomeGeneratorConfiguration) -> Genome;
}
