use super::*;

#[test]
fn places_two_hoxes_after_each_other() {
    let mut genome = genome_stub();
    add_first_cluster_to_genome(&mut genome);

    add_initial_hox_gene_to_genome(&mut genome);
    add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(&mut genome, HoxGeneIndex(0));
    add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(&mut genome, HoxGeneIndex(1));

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_first_cluster_placed_on_second_hox_by_third_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn places_two_hoxes_with_the_same_target() {
    let mut genome = genome_stub();
    add_first_cluster_to_genome(&mut genome);

    add_initial_hox_gene_to_genome(&mut genome);
    add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(&mut genome, HoxGeneIndex(0));
    add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(&mut genome, HoxGeneIndex(0));

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_first_cluster_placed_on_first_hox_by_third_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn add_hox_gene_placing_first_cluster_on_clusters_placed_by_hox(
    genome: &mut Genome,
    hox_index: HoxGeneIndex,
) {
    add_hox_gene_placing_cluster_on_hox(
        genome,
        ClusterOnHoxTestParameters {
            hox_gene: hox_index,
            target_neuron: NeuronClusterLocalIndex(3),
            cluster_index: ClusterGeneIndex(0),
        },
    )
}

fn expect_first_cluster_placed_on_first_hox_by_second_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 4,
            placement_neuron_index: 1,
            placement_neuron_handle: 3,
        },
    )
}

fn expect_first_cluster_placed_on_second_hox_by_third_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 7,
            placement_neuron_index: 1,
            placement_neuron_handle: 6,
        },
    )
}

fn expect_first_cluster_placed_on_first_hox_by_third_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 7,
            placement_neuron_index: 1,
            placement_neuron_handle: 3,
        },
    )
}
