use super::*;

#[test]
fn places_two_hoxes_after_each_other() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);

    let genome = add_initial_hox_gene_to_genome(genome);
    let genome =
        add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(genome, HoxGeneIndex(0));
    let genome =
        add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(genome, HoxGeneIndex(1));

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_connections(&mut configurator);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_first_cluster_placed_on_second_hox_by_third_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn places_two_hoxes_with_the_same_target() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);

    let genome = add_initial_hox_gene_to_genome(genome);
    let genome =
        add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(genome, HoxGeneIndex(0));
    let genome =
        add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(genome, HoxGeneIndex(0));

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_connections(&mut configurator);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_first_cluster_placed_on_first_hox_by_third_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(
    mut genome: Genome,
    hox_index: HoxGeneIndex,
) -> Genome {
    genome.hox_genes.push(HoxGene {
        placement: HoxPlacement::HoxGene {
            hox_gene: hox_index,
            target_neuron: NeuronClusterLocalIndex(3),
        },
        cluster_index: ClusterGeneIndex(0),
        disabled_connections: Vec::new(),
    });
    genome
}

fn expect_first_cluster_connections(configurator: &mut NeuralNetworkConfiguratorMock<'_>) {
    first_cluster_connections()
        .into_iter()
        .map(connection_definition_to_connection)
        .for_each(|connection| {
            configurator
                .expect_add_connection(partial_eq(connection))
                .returns(Ok(()));
        });
}

fn expect_first_cluster_placed_on_first_hox_by_second_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox(configurator, 4, 1, 3)
}

fn expect_first_cluster_placed_on_second_hox_by_third_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox(configurator, 7, 1, 6)
}

fn expect_first_cluster_placed_on_first_hox_by_third_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox(configurator, 7, 1, 3)
}

fn expect_first_cluster_placed_on_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
    cluster_offset: usize,
    placement_neuron_index: usize,
    placement_neuron_handle: usize,
) {
    first_cluster_connections()
        .into_iter()
        .map(|connection_definition| {
            connection_definition_to_placed_connection(ConnectionTranslationParameters {
                connection: connection_definition,
                cluster_offset,
                placement_neuron_index,
                placement_neuron_handle,
            })
        })
        .for_each(|connection| {
            configurator
                .expect_add_connection(partial_eq(connection))
                .returns(Ok(()));
        });
}
