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
    fn generate_genome(
        &self,
        &GenomeGeneratorConfiguration {
            input_neuron_count,
            output_neuron_count,
        }: &GenomeGeneratorConfiguration,
    ) -> Genome {
        let corpus_callosum_config = CorpusCallosumConfiguration {
            input_neuron_count,
            output_neuron_count,
        };
        let corpus_callosum = self
            .corpus_callosum_cluster_gene_generator
            .generate(&corpus_callosum_config);

        const IO_CLUSTER_GENE_OFFSET: ClusterGeneIndex = ClusterGeneIndex(1);

        let input_hox_genes_config = IoHoxGeneGenerationConfiguration {
            neuron_count: input_neuron_count,
            cluster_gene_offset: IO_CLUSTER_GENE_OFFSET,
            corpus_callosum_cluster_neurons: corpus_callosum.input_cluster_neurons,
        };
        let (input_hox_genes, input_cluster_genes) = self
            .generate_io_hox_genes_with_clusters(input_hox_genes_config, || {
                self.io_cluster_gene_generator.generate_input_cluster_gene()
            });

        let output_cluster_gene_offset =
            ClusterGeneIndex(IO_CLUSTER_GENE_OFFSET.0 + input_cluster_genes.len());
        let output_hox_genes_config = IoHoxGeneGenerationConfiguration {
            neuron_count: output_neuron_count,
            cluster_gene_offset: output_cluster_gene_offset,
            corpus_callosum_cluster_neurons: corpus_callosum.output_cluster_neurons,
        };
        let (output_hox_genes, output_cluster_genes) =
            self.generate_io_hox_genes_with_clusters(output_hox_genes_config, || {
                self.io_cluster_gene_generator
                    .generate_output_cluster_gene()
            });

        let hox_genes = iter::once(corpus_callosum_hox_gene())
            .chain(input_hox_genes)
            .chain(output_hox_genes)
            .collect();
        let cluster_genes = iter::once(corpus_callosum.cluster_gene)
            .chain(input_cluster_genes)
            .chain(output_cluster_genes)
            .collect();
        Genome {
            hox_genes,
            cluster_genes,
        }
    }
}

fn corpus_callosum_hox_gene() -> HoxGene {
    const CORPUS_CALLOSUM_CLUSTER_GENE: ClusterGeneIndex = ClusterGeneIndex(0);
    HoxGene {
        placement_target: HoxPlacement::Standalone,
        cluster_gene: CORPUS_CALLOSUM_CLUSTER_GENE,
        disabled_connections: Vec::new(),
    }
}

struct IoHoxGeneGenerationConfiguration {
    neuron_count: NonZeroUsize,
    cluster_gene_offset: ClusterGeneIndex,
    corpus_callosum_cluster_neurons: Vec<ClusterNeuronIndex>,
}

impl GenomeGeneratorImpl {
    fn generate_io_hox_genes_with_clusters(
        &self,
        configuration: IoHoxGeneGenerationConfiguration,
        generate_cluster_gene_fn: impl Fn() -> ClusterGene,
    ) -> (Vec<HoxGene>, Vec<ClusterGene>) {
        let neuron_count = configuration.neuron_count.get();
        let selections: Vec<_> = self
            .generate_cluster_gene_selections(neuron_count)
            .collect();
        let cluster_genes: Vec<_> = self
            .generate_cluster_genes_from_selections(&selections, generate_cluster_gene_fn)
            .collect();
        let hox_genes: Vec<_> = self
            .generate_hox_genes_from_selections(&selections, &cluster_genes, &configuration)
            .collect();
        (hox_genes, cluster_genes)
    }

    fn generate_cluster_gene_selections<'a>(
        &'a self,
        neuron_count: usize,
    ) -> impl Iterator<Item = EnumeratedClusterGeneSelection> + 'a {
        const FIRST_CLUSTER_GENE_SELECTION: ClusterGeneSelection = ClusterGeneSelection::New;
        let variable_cluster_gene_selections = (0..neuron_count)
            .skip(1)
            .map(move |_| self.select_cluster_gene());
        iter::once(FIRST_CLUSTER_GENE_SELECTION)
            .chain(variable_cluster_gene_selections)
            .scan(0, |current_new_index, selection| {
                Some(enumerate_cluster_gene_selection(
                    current_new_index,
                    selection,
                ))
            })
    }

    fn generate_cluster_genes_from_selections<'a>(
        &'a self,
        selections: &'a [EnumeratedClusterGeneSelection],
        generate_cluster_gene_fn: impl Fn() -> ClusterGene + 'a,
    ) -> impl Iterator<Item = ClusterGene> + 'a {
        selections
            .iter()
            .filter(|selection| selection.is_new())
            .map(move |_| generate_cluster_gene_fn())
    }

    fn generate_hox_genes_from_selections<'a>(
        &'a self,
        selections: &'a [EnumeratedClusterGeneSelection],
        cluster_genes: &'a [ClusterGene],
        IoHoxGeneGenerationConfiguration {
            cluster_gene_offset,
            corpus_callosum_cluster_neurons,
            ..
        }: &'a IoHoxGeneGenerationConfiguration,
    ) -> impl Iterator<Item = HoxGene> + 'a {
        selections
            .iter()
            .enumerate()
            .map(move |(index, &selection)| {
                let relative_cluster_gene_index =
                    self.pick_cluster_gene_from_selection(cluster_genes.len(), selection);
                let cluster_gene =
                    ClusterGeneIndex(cluster_gene_offset.0 + relative_cluster_gene_index);
                let target_neuron = *corpus_callosum_cluster_neurons
                    .get(index)
                    .expect("Corpus callosum does not contain enough neurons");
                HoxGene {
                    cluster_gene,
                    disabled_connections: Vec::new(),
                    placement_target: HoxPlacement::HoxGene {
                        hox_gene: PLACEMENT_TARGET_HOX_GENE,
                        target_neuron,
                    },
                }
            })
    }

    fn pick_cluster_gene_from_selection(
        &self,
        cluster_gene_count: usize,
        selection: EnumeratedClusterGeneSelection,
    ) -> usize {
        match selection {
            EnumeratedClusterGeneSelection::Existing => {
                self.random.usize_in_range(0, cluster_gene_count)
            }
            EnumeratedClusterGeneSelection::New(index) => index,
        }
    }

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

fn enumerate_cluster_gene_selection(
    current_new_index: &mut usize,
    selection: ClusterGeneSelection,
) -> EnumeratedClusterGeneSelection {
    match selection {
        ClusterGeneSelection::Existing => EnumeratedClusterGeneSelection::Existing,
        ClusterGeneSelection::New => {
            let index = *current_new_index;
            *current_new_index += 1;
            EnumeratedClusterGeneSelection::New(index)
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
