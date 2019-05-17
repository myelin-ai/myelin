//! Default implementation of [`GenomeGenerator`].

pub use self::corpus_callosum_cluster_gene_generator_impl::*;
pub use self::io_cluster_gene_generator_impl::*;
use crate::genome::*;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use std::fmt::Debug;
use std::num::NonZeroUsize;

mod corpus_callosum_cluster_gene_generator_impl;
mod io_cluster_gene_generator_impl;

/// Generates new input and output clusters
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait IoClusterGeneGenerator: Debug {
    /// Generates a new [`ClusterGene`] with the specialization [`ClusterGeneSpecialization::Input`].
    fn generate_input_cluster_gene(&self) -> ClusterGene;
    /// Generates a new [`ClusterGene`] with the specialization [`ClusterGeneSpecialization::Output`].
    fn generate_output_cluster_gene(&self) -> ClusterGene;
}

/// Generates the central cluster gene that connects input
/// with output clusters (aka "[Corpus callosum]").
///
/// [Corpus callosum]: https://en.wikipedia.org/wiki/Corpus_callosum
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait CorpusCallosumClusterGeneGenerator: Debug {
    /// Generates a new corpus callosum [`ClusterGene`]
    fn generate_cluster_gene(&self, configuration: &CorpusCallosumConfiguration) -> ClusterGene;
}

/// Configuration for generating a new corpus callosum [`ClusterGene`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CorpusCallosumConfiguration {
    /// The amount of input neurons
    pub input_neuron_count: NonZeroUsize,
    /// The amount of output neurons
    pub output_neuron_count: NonZeroUsize,
}

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug)]
pub struct GenomeGeneratorImpl {
    io_cluster_gene_generator: Box<dyn IoClusterGeneGenerator>,
    corpus_callosum_cluster_gene_generator: Box<dyn CorpusCallosumClusterGeneGenerator>,
}

impl GenomeGeneratorImpl {
    /// Creates a new [`GenomeGeneratorImpl`].
    pub fn new(
        io_cluster_gene_generator: Box<dyn IoClusterGeneGenerator>,
        corpus_callosum_cluster_gene_generator: Box<dyn CorpusCallosumClusterGeneGenerator>,
    ) -> Self {
        GenomeGeneratorImpl {
            io_cluster_gene_generator,
            corpus_callosum_cluster_gene_generator,
        }
    }
}

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_genome() {
        test_genome_is_generated_correctly(GenerateGenomeTestConfiguration {
            input_neuron_count: 7,
            output_neuron_count: 4,
        });
    }

    struct GenerateGenomeTestConfiguration {
        input_neuron_count: usize,
        output_neuron_count: usize,
    }

    fn test_genome_is_generated_correctly(
        GenerateGenomeTestConfiguration {
            input_neuron_count,
            output_neuron_count,
        }: GenerateGenomeTestConfiguration,
    ) {
        let config = genome_generator_configuration(input_neuron_count, output_neuron_count);
        let mut io_cluster_gene_generator = IoClusterGeneGeneratorMock::new();
        let mut corpus_callosum_cluster_gene_generator = CorpusCallosumClusterGeneGeneratorMock::new();
        let genome_generator = GenomeGeneratorImpl::new(
            box io_cluster_gene_generator,
            box corpus_callosum_cluster_gene_generator,
        );
        let _genome = genome_generator.generate_genome(&config);
    }

    fn genome_generator_configuration(
        input_neuron_count: usize,
        output_neuron_count: usize,
    ) -> GenomeGeneratorConfiguration {
        GenomeGeneratorConfiguration {
            input_neuron_count: NonZeroUsize::new(input_neuron_count).unwrap(),
            output_neuron_count: NonZeroUsize::new(output_neuron_count).unwrap(),
        }
    }
}
