//! Default implementation of [`GenomeGenerator`].

pub use self::corpus_callosum_cluster_gene_generator_impl::*;
pub use self::io_cluster_gene_generator_impl::*;
use crate::genome::*;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use myelin_random::Random;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use wonderbox::autoresolvable;

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
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ClusterGeneSelection {
    Existing,
    New,
}

const PROBABILITY_FOR_NEW_CLUSTER_GENE: f64 = 3.0 / 4.0;

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::{partial_eq, partial_eq_owned};
    use myelin_random::RandomMock;

    #[test]
    fn generates_correct_genome() {
        test_genome_is_generated_correctly(GenerateGenomeTestConfiguration::default());
    }

    struct GenerateGenomeTestConfiguration {
        input_neuron_count: usize,
        output_neuron_count: usize,
        input_cluster_gene_selections: Vec<DetailedClusterGeneSelection>,
        output_cluster_gene_selections: Vec<DetailedClusterGeneSelection>,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum DetailedClusterGeneSelection {
        Existing(usize),
        New,
    }

    impl From<DetailedClusterGeneSelection> for ClusterGeneSelection {
        fn from(selection: DetailedClusterGeneSelection) -> Self {
            match selection {
                DetailedClusterGeneSelection::Existing(_) => ClusterGeneSelection::Existing,
                DetailedClusterGeneSelection::New => ClusterGeneSelection::New,
            }
        }
    }

    impl Default for GenerateGenomeTestConfiguration {
        fn default() -> Self {
            Self {
                input_neuron_count: 3,
                output_neuron_count: 2,
                input_cluster_gene_selections: vec![
                    DetailedClusterGeneSelection::New,
                    DetailedClusterGeneSelection::Existing(1),
                ],
                output_cluster_gene_selections: vec![DetailedClusterGeneSelection::Existing(0)],
            }
        }
    }

    fn test_genome_is_generated_correctly(
        GenerateGenomeTestConfiguration {
            input_neuron_count,
            output_neuron_count,
            input_cluster_gene_selections,
            output_cluster_gene_selections,
        }: GenerateGenomeTestConfiguration,
    ) {
        let corpus_callosum_configuration = CorpusCallosumConfiguration {
            input_neuron_count: NonZeroUsize::new(input_neuron_count).unwrap(),
            output_neuron_count: NonZeroUsize::new(input_neuron_count).unwrap(),
        };
        let corpus_callosum_cluster_gene_generator =
            mock_corpus_callosum_cluster_gene_generator(corpus_callosum_configuration);

        let mut random = RandomMock::new();
        register_cluster_gene_selection_expectations(&mut random, &input_cluster_gene_selections);
        register_cluster_gene_selection_expectations(&mut random, &output_cluster_gene_selections);

        let io_cluster_gene_generator = mock_io_cluster_gene_generator(
            &input_cluster_gene_selections,
            &output_cluster_gene_selections,
        );

        let genome_generator = GenomeGeneratorImpl::new(
            io_cluster_gene_generator,
            corpus_callosum_cluster_gene_generator,
            box random,
        );

        let config = genome_generator_configuration(input_neuron_count, output_neuron_count);
        let _genome = genome_generator.generate_genome(&config);
    }

    fn mock_corpus_callosum_cluster_gene_generator(
        configuration: CorpusCallosumConfiguration,
    ) -> Box<dyn CorpusCallosumClusterGeneGenerator> {
        let mut corpus_callosum_cluster_gene_generator =
            CorpusCallosumClusterGeneGeneratorMock::new();

        corpus_callosum_cluster_gene_generator
            .expect_generate_cluster_gene(partial_eq_owned(configuration))
            .returns(cluster_gene_stub());

        box corpus_callosum_cluster_gene_generator
    }

    fn mock_io_cluster_gene_generator(
        input_cluster_gene_selections: &[DetailedClusterGeneSelection],
        output_cluster_gene_selections: &[DetailedClusterGeneSelection],
    ) -> Box<dyn IoClusterGeneGenerator> {
        let mut io_cluster_gene_generator = IoClusterGeneGeneratorMock::new();

        let input_cluster_gene_count = count_generated_cluster_genes(input_cluster_gene_selections);
        io_cluster_gene_generator
            .expect_generate_input_cluster_gene()
            .times(input_cluster_gene_count as u64)
            .returns(cluster_gene_stub());

        let output_cluster_gene_count =
            count_generated_cluster_genes(output_cluster_gene_selections);
        io_cluster_gene_generator
            .expect_generate_output_cluster_gene()
            .times(output_cluster_gene_count as u64)
            .returns(cluster_gene_stub());

        box io_cluster_gene_generator
    }

    fn register_cluster_gene_selection_expectations(
        random: &mut RandomMock<'_>,
        cluster_gene_selections: &[DetailedClusterGeneSelection],
    ) {
        random.expect_flip_coin_with_probability_calls_in_order();
        random.expect_random_usize_in_range_calls_in_order();

        let generated_cluster_gene_count = count_generated_cluster_genes(cluster_gene_selections);

        for &selection in cluster_gene_selections {
            let coin_toss_result = selection == DetailedClusterGeneSelection::New;
            random
                .expect_flip_coin_with_probability(partial_eq(PROBABILITY_FOR_NEW_CLUSTER_GENE))
                .returns(coin_toss_result);

            if let DetailedClusterGeneSelection::Existing(cluster_gene_index) = selection {
                random
                    .expect_random_usize_in_range(
                        partial_eq(0),
                        partial_eq(generated_cluster_gene_count),
                    )
                    .returns(cluster_gene_index);
            }
        }
    }

    fn cluster_gene_stub() -> ClusterGene {
        ClusterGene {
            neurons: Vec::new(),
            connections: Vec::new(),
            placement_neuron: ClusterNeuronIndex(0),
            specialization: ClusterGeneSpecialization::None,
        }
    }

    fn count_generated_cluster_genes(
        cluster_gene_selections: &[DetailedClusterGeneSelection],
    ) -> usize {
        const ALWAYS_GENERATED_CLUSTER_GENES: usize = 1;
        let new_cluster_genes = cluster_gene_selections
            .iter()
            .filter(|&&selection| selection == DetailedClusterGeneSelection::New)
            .count();
        ALWAYS_GENERATED_CLUSTER_GENES + new_cluster_genes
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
