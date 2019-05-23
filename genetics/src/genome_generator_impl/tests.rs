use super::*;
use mockiato::{partial_eq, partial_eq_owned};
use myelin_random::RandomMock;
use pretty_assertions::assert_eq;

#[test]
fn generates_correct_genome() {
    test_genome_is_generated_correctly(GenerateGenomeTestConfiguration::default());
}

struct GenerateGenomeTestConfiguration {
    input_cluster_neurons: Vec<ClusterNeuronIndex>,
    output_cluster_neurons: Vec<ClusterNeuronIndex>,
    input_cluster_gene_selections: Vec<DetailedClusterGeneSelection>,
    output_cluster_gene_selections: Vec<DetailedClusterGeneSelection>,
    expected_genome: Genome,
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
        let mut create_input_hox_gene = create_io_hox_gene_fn(ClusterNeuronIndex(0));
        let mut create_output_hox_gene = create_io_hox_gene_fn(ClusterNeuronIndex(5));
        let expected_genome = Genome {
            hox_genes: vec![
                HoxGene {
                    placement_target: HoxPlacement::Standalone,
                    cluster_gene: ClusterGeneIndex(0),
                    disabled_connections: Vec::new(),
                },
                create_input_hox_gene(ClusterGeneIndex(1)),
                create_input_hox_gene(ClusterGeneIndex(2)),
                create_input_hox_gene(ClusterGeneIndex(2)),
                create_output_hox_gene(ClusterGeneIndex(3)),
                create_output_hox_gene(ClusterGeneIndex(3)),
            ],
            cluster_genes: vec![cluster_gene_stub(); 4],
        };

        Self {
            input_cluster_neurons: neuron_indices_in_range(0, 3),
            output_cluster_neurons: neuron_indices_in_range(5, 7),
            input_cluster_gene_selections: vec![
                DetailedClusterGeneSelection::New,
                DetailedClusterGeneSelection::Existing(1),
            ],
            output_cluster_gene_selections: vec![DetailedClusterGeneSelection::Existing(0)],
            expected_genome,
        }
    }
}

fn neuron_indices_in_range(start: usize, end: usize) -> Vec<ClusterNeuronIndex> {
    (start..end).map(ClusterNeuronIndex).collect()
}

fn create_io_hox_gene_fn(
    target_neuron_offset: ClusterNeuronIndex,
) -> impl FnMut(ClusterGeneIndex) -> HoxGene {
    let mut target_neuron_counter = 0;

    move |cluster_gene| {
        let target_neuron = ClusterNeuronIndex(target_neuron_offset.0 + target_neuron_counter);

        target_neuron_counter += 1;

        HoxGene {
            cluster_gene,
            disabled_connections: Vec::new(),
            placement_target: HoxPlacement::HoxGene {
                hox_gene: HoxGeneIndex(0),
                target_neuron,
            },
        }
    }
}

fn test_genome_is_generated_correctly(
    GenerateGenomeTestConfiguration {
        input_cluster_neurons,
        output_cluster_neurons,
        input_cluster_gene_selections,
        output_cluster_gene_selections,
        expected_genome,
    }: GenerateGenomeTestConfiguration,
) {
    let input_neuron_count = input_cluster_neurons.len();
    let output_neuron_count = output_cluster_neurons.len();
    let corpus_callosum_configuration = CorpusCallosumConfiguration {
        input_neuron_count: NonZeroUsize::new(input_neuron_count).unwrap(),
        output_neuron_count: NonZeroUsize::new(output_neuron_count).unwrap(),
    };
    let corpus_callosum_cluster_gene_generator = mock_corpus_callosum_cluster_gene_generator(
        corpus_callosum_configuration,
        input_cluster_neurons,
        output_cluster_neurons,
    );

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
    let genome = genome_generator.generate_genome(&config);

    assert_eq!(expected_genome, genome);
}

fn mock_corpus_callosum_cluster_gene_generator(
    configuration: CorpusCallosumConfiguration,
    input_cluster_neurons: Vec<ClusterNeuronIndex>,
    output_cluster_neurons: Vec<ClusterNeuronIndex>,
) -> Box<dyn CorpusCallosumClusterGeneGenerator> {
    let mut corpus_callosum_cluster_gene_generator = CorpusCallosumClusterGeneGeneratorMock::new();

    corpus_callosum_cluster_gene_generator
        .expect_generate(partial_eq_owned(configuration))
        .returns(corpus_callosum_stub(
            input_cluster_neurons,
            output_cluster_neurons,
        ));

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

    let output_cluster_gene_count = count_generated_cluster_genes(output_cluster_gene_selections);
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
    random.expect_usize_in_range_calls_in_order();

    let generated_cluster_gene_count = count_generated_cluster_genes(cluster_gene_selections);

    for &selection in cluster_gene_selections {
        let coin_toss_result = selection == DetailedClusterGeneSelection::New;
        random
            .expect_flip_coin_with_probability(partial_eq(PROBABILITY_FOR_NEW_CLUSTER_GENE))
            .returns(coin_toss_result);

        if let DetailedClusterGeneSelection::Existing(cluster_gene_index) = selection {
            random
                .expect_usize_in_range(partial_eq(0), partial_eq(generated_cluster_gene_count))
                .returns(cluster_gene_index);
        }
    }
}

fn corpus_callosum_stub(
    input_cluster_neurons: Vec<ClusterNeuronIndex>,
    output_cluster_neurons: Vec<ClusterNeuronIndex>,
) -> CorpusCallosum {
    CorpusCallosum {
        cluster_gene: cluster_gene_stub(),
        input_cluster_neurons,
        output_cluster_neurons,
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
