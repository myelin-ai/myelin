use super::*;

#[test]
fn places_two_hox_genes_placing_first_cluster_gene_on_cluster_of_initial_hox() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_clusters_on_clusters_of_first_cluster_gene()
        .add_hox_gene_placing_clusters_on_clusters_of_first_cluster_gene()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 13);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_second_hox_places_first_cluster_on_initially_placed_cluster(&mut configurator);
    expect_third_hox_places_first_cluster_on_initially_placed_cluster(&mut configurator);
    expect_third_hox_places_first_cluster_on_second_placed_cluster(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

impl GenomeStubBuilder {
    fn add_hox_gene_placing_clusters_on_clusters_of_first_cluster_gene(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_cluster(ClusterOnClusterTestParameters {
            cluster_gene: ClusterGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(3),
            cluster_index: ClusterGeneIndex(0),
        });
        self
    }
}

fn expect_second_hox_places_first_cluster_on_initially_placed_cluster(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 4,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 1,
                handle: 3,
            }),
        },
    )
}

fn expect_third_hox_places_first_cluster_on_initially_placed_cluster(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 7,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 1,
                handle: 3,
            }),
        },
    )
}

fn expect_third_hox_places_first_cluster_on_second_placed_cluster(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 10,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 1,
                handle: 6,
            }),
        },
    )
}
