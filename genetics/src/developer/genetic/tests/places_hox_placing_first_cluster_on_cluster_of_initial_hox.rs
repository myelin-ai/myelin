use super::*;

#[test]
fn places_hox_placing_first_cluster_on_cluster_of_initial_hox() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_first_cluster_on_first_hox_clusters()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 7);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

impl GenomeStubBuilder {
    fn add_hox_gene_placing_first_cluster_on_first_hox_clusters(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_hox(ClusterOnHoxTestParameters {
            hox_gene: HoxGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(3),
            cluster_index: ClusterGeneIndex(0),
        });
        self
    }
}

fn expect_first_cluster_placed_on_first_cluster_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_connections(
        configurator,
        first_cluster_connections(),
        ExpectConnectionsParameters {
            cluster_offset: 4,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 1,
                handle: 3,
            }),
        },
    )
}
