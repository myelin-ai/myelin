use super::*;

#[test]
fn clusters_are_only_placed_on_specified_targets() {
    let mut genome = genome_stub();
    add_first_cluster_to_genome(&mut genome);
    add_second_cluster_to_genome(&mut genome);

    add_initial_hox_gene_to_genome(&mut genome);
    add_hox_gene_placing_first_cluster_on_first_hox(&mut genome);
    add_hox_gene_placing_second_cluster_on_first_hox(&mut genome);
    add_hox_gene_placing_second_cluster_on_first_cluster(&mut genome);

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_second_cluster_placed_on_first_hox_by_third_hox(&mut configurator);
    expect_second_cluster_placed_on_first_placed_cluster_by_fourth_hox(&mut configurator);
    expect_second_cluster_placed_on_second_placed_cluster_by_fourth_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn clusters_are_only_placed_on_specified_targets_when_target_is_hox_that_targeted_cluster_type() {
    let mut genome = genome_stub();
    add_first_cluster_to_genome(&mut genome);
    add_second_cluster_to_genome(&mut genome);

    add_initial_hox_gene_to_genome(&mut genome);
    add_hox_gene_placing_first_cluster_on_first_hox(&mut genome);
    add_hox_gene_placing_second_cluster_on_first_hox(&mut genome);
    add_hox_gene_placing_second_cluster_on_first_cluster(&mut genome);
    add_hox_gene_placing_first_cluster_on_fourth_hox(&mut genome);
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_second_cluster_placed_on_first_hox_by_third_hox(&mut configurator);

    expect_second_cluster_placed_on_first_placed_cluster_by_fourth_hox(&mut configurator);
    expect_second_cluster_placed_on_second_placed_cluster_by_fourth_hox(&mut configurator);

    expect_first_cluster_placed_on_fourth_placed_cluster_by_fifth_hox(&mut configurator);
    expect_first_cluster_placed_on_fifth_placed_cluster_by_fifth_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn add_hox_gene_placing_first_cluster_on_first_hox(genome: &mut Genome) {
    add_hox_gene_placing_cluster_on_hox(
        genome,
        ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(3),
            cluster_index: ClusterGeneIndex(0),
        },
    )
}

fn add_hox_gene_placing_second_cluster_on_first_hox(genome: &mut Genome) {
    add_hox_gene_placing_cluster_on_hox(
        genome,
        ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(2),
            cluster_index: ClusterGeneIndex(1),
        },
    )
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

fn add_hox_gene_placing_first_cluster_on_fourth_hox(genome: &mut Genome) {
    add_hox_gene_placing_cluster_on_hox(
        genome,
        ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(3),
            target_neuron: NeuronClusterLocalIndex(1),
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

fn expect_second_cluster_placed_on_first_hox_by_third_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 7,
            placement_neuron_index: 0,
            placement_neuron_handle: 2,
        },
    )
}

fn expect_second_cluster_placed_on_first_placed_cluster_by_fourth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 9,
            placement_neuron_index: 0,
            placement_neuron_handle: 2,
        },
    )
}

fn expect_second_cluster_placed_on_second_placed_cluster_by_fourth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 11,
            placement_neuron_index: 0,
            placement_neuron_handle: 5,
        },
    )
}

fn expect_first_cluster_placed_on_fourth_placed_cluster_by_fifth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 13,
            placement_neuron_index: 1,
            placement_neuron_handle: 9,
        },
    )
}

fn expect_first_cluster_placed_on_fifth_placed_cluster_by_fifth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 16,
            placement_neuron_index: 1,
            placement_neuron_handle: 11,
        },
    )
}
