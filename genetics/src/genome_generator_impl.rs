//! Default implementation of [`GenomeGenerator`].

pub use self::corpus_callosum_cluster_gene_generator_impl::*;
pub use self::io_cluster_gene_generator_impl::*;
use crate::genome::*;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
use matches::matches;
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use myelin_random::Random;
use std::fmt::Debug;
use std::iter;
use std::num::NonZeroUsize;
use wonderbox::autoresolvable;

mod corpus_callosum_cluster_gene_generator_impl;
mod io_cluster_gene_generator_impl;
#[cfg(test)]
mod tests;

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
    /// Generates a new corpus callosum [`ClusterGene`].
    fn generate(&self, configuration: &CorpusCallosumConfiguration) -> CorpusCallosum;
}

/// Configuration for generating a new corpus callosum [`ClusterGene`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CorpusCallosumConfiguration {
    /// The amount of input neurons.
    pub input_neuron_count: NonZeroUsize,
    /// The amount of output neurons.
    pub output_neuron_count: NonZeroUsize,
}

/// A generated [Corpus callosum].
///
/// [Corpus callosum]: https://en.wikipedia.org/wiki/Corpus_callosum
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusCallosum {
    /// The generated cluster gene.
    pub cluster_gene: ClusterGene,
    /// Indices of the neurons in [`CorpusCallosum::cluster_gene`] that connect with input clusters.
    pub input_cluster_neurons: Vec<ClusterNeuronIndex>,
    /// Indices of the neurons in [`CorpusCallosum::cluster_gene`] that connect with output clusters.
    pub output_cluster_neurons: Vec<ClusterNeuronIndex>,
}

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug)]
pub struct GenomeGeneratorImpl {
    io_cluster_gene_generator: Box<dyn IoClusterGeneGenerator>,
    corpus_callosum_cluster_gene_generator: Box<dyn CorpusCallosumClusterGeneGenerator>,
    random: Box<dyn Random>,
}

#[autoresolvable]
impl GenomeGeneratorImpl {
    /// Creates a new [`GenomeGeneratorImpl`].
    pub fn new(
        io_cluster_gene_generator: Box<dyn IoClusterGeneGenerator>,
        corpus_callosum_cluster_gene_generator: Box<dyn CorpusCallosumClusterGeneGenerator>,
        random: Box<dyn Random>,
    ) -> Self {
        GenomeGeneratorImpl {
            io_cluster_gene_generator,
            corpus_callosum_cluster_gene_generator,
            random,
        }
    }
}

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, configuration: &GenomeGeneratorConfiguration) -> Genome {
        let input_cluster_selections = iter::once(ClusterGeneSelection::New)
            .chain(
                (0..(configuration.input_neuron_count.get().saturating_sub(1)))
                    .map(|_| self.select_cluster_gene()),
            )
            .scan(0, |current_new_index, selection| {
                Some(match selection {
                    ClusterGeneSelection::Existing => EnumeratedClusterGeneSelection::Existing,
                    ClusterGeneSelection::New => {
                        let index = *current_new_index;
                        *current_new_index += 1;
                        EnumeratedClusterGeneSelection::New(index)
                    }
                })
            });

        let input_clusters: Vec<_> = input_cluster_selections
            .clone()
            .filter(|selection| selection.is_new())
            .map(|_| self.io_cluster_gene_generator.generate_input_cluster_gene())
            .collect();

        let input_hox_genes = input_cluster_selections
            .enumerate()
            .map(|(index, selection)| {
                let cluster_index = match selection {
                    EnumeratedClusterGeneSelection::Existing => {
                        self.random.usize_in_range(0, input_clusters.len())
                    }
                    EnumeratedClusterGeneSelection::New(index) => index,
                };

                HoxGene {
                    cluster_index: ClusterGeneIndex(cluster_index),
                    disabled_connections: Vec::new(),
                    placement_target: HoxPlacement::HoxGene {
                        hox_gene: PLACEMENT_TARGET_HOX_GENE,
                        target_neuron: ClusterNeuronIndex(index),
                    },
                }
            });

        unimplemented!()
    }
}

impl GenomeGeneratorImpl {
    fn select_cluster_gene(&self) -> ClusterGeneSelection {
        if self
            .random
            .flip_coin_with_probability(PROBABILITY_FOR_NEW_CLUSTER_GENE)
        {
            ClusterGeneSelection::New
        } else {
            ClusterGeneSelection::Existing
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ClusterGeneSelection {
    Existing,
    New,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum EnumeratedClusterGeneSelection {
    Existing,
    New(usize),
}

impl EnumeratedClusterGeneSelection {
    fn is_new(self) -> bool {
        matches!(self, EnumeratedClusterGeneSelection::New(_))
    }
}

const PROBABILITY_FOR_NEW_CLUSTER_GENE: f64 = 3.0 / 4.0;
const PLACEMENT_TARGET_HOX_GENE: HoxGeneIndex = HoxGeneIndex(0);
