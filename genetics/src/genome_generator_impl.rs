//! Default implementation of [`GenomeGenerator`].

pub use self::io_cluster_gene_generator_impl::*;
use crate::genome::*;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use std::fmt::Debug;
pub use self::corpus_callossum_cluster_gene_generator_impl::*;

mod io_cluster_gene_generator_impl;
mod corpus_callossum_cluster_gene_generator_impl;

/// Generates new input and output clusters
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait IoClusterGeneGenerator: Debug {
    /// Generates a new [`ClusterGene`] with the specialization [`ClusterGeneSpecialization::Input`].
    fn generate_input_cluster_gene(&self) -> ClusterGene;
    /// Generates a new [`ClusterGene`] with the specialization [`ClusterGeneSpecialization::Output`].
    fn generate_output_cluster_gene(&self) -> ClusterGene;
}

/// Generates the central cluster gene that connects input
/// with output clusters (aka "Corpus callosum").
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait CorpusCallosumClusterGeneGenerator: Debug {
    /// Generates a new corpus callossum [`ClusterGene`]
    fn generate_cluster_gene(&self, config: &CorpusCallosumConfig) -> ClusterGene;
}

/// Configuration for generating a new corpus callosum [`ClusterGene`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusCallosumConfig {
    /// The amount of input neurons
    pub input_neuron_count: usize,
    /// The amount of output neurons
    pub output_neuron_count: usize,
}

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug, Default, Clone)]
pub struct GenomeGeneratorImpl;

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}
