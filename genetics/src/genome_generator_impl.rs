//! Default implementation of [`GenomeGenerator`].

pub use self::io_cluster_gene_generator_impl::*;
use crate::genome::*;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;

mod io_cluster_gene_generator_impl;

#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
/// Generates new input and output clusters
pub trait IoClusterGeneGenerator {
    /// Generates a new [`ClusterGene`] with the specialization [`ClusterGeneSpecialization::Input`].
    fn generate_input_cluster_gene(&self) -> ClusterGene;
    /// Generates a new [`ClusterGene`] with the specialization [`ClusterGeneSpecialization::Output`].
    fn generate_output_cluster_gene(&self) -> ClusterGene;
}

/// Generates the central cluster gene that connects input
/// with output clusters (aka "Corpus callosum").
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait CorpusCallosumClusterGeneGenerator {
    /// Generates a new corpus callossum [`ClusterGene`]
    fn generate_cluster_gene(&self) -> ClusterGene;
}

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug, Default, Clone)]
pub struct GenomeGeneratorImpl;

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}
