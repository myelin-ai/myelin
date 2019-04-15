use super::*;

#[test]
fn clusters_are_only_placed_on_specified_targets() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_second_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_first_cluster_on_first_hox()
        .add_hox_gene_placing_second_cluster_on_first_hox()
        .add_hox_gene_placing_second_cluster_on_first_cluster()
        .build();

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
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_second_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_first_cluster_on_first_hox()
        .add_hox_gene_placing_second_cluster_on_first_hox()
        .add_hox_gene_placing_second_cluster_on_first_cluster()
        .add_hox_gene_placing_first_cluster_on_fourth_hox()
        .build();

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

impl GenomeStubBuilder {
    fn add_hox_gene_placing_first_cluster_on_first_hox(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_hox(ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(3),
            cluster_index: ClusterGeneIndex(0),
        });
        self
    }

    fn add_hox_gene_placing_second_cluster_on_first_hox(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_hox(ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(2),
            cluster_index: ClusterGeneIndex(1),
        });
        self
    }

    fn add_hox_gene_placing_first_cluster_on_fourth_hox(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_hox(ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(3),
            target_neuron: NeuronClusterLocalIndex(1),
            cluster_index: ClusterGeneIndex(0),
        });
        self
    }
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
