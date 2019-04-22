use super::*;
use crate::genome::ConnectionFilter;

#[test]
fn does_not_create_connections_disabled_on_hox_gene() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_initial_hox_gene_with_disabled_connection()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 4);
    expect_first_cluster_connections_without_disabled_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn ignores_invalid_disabled_connections() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_initial_hox_gene_with_valid_and_invalid_disabled_connection()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 4);
    expect_first_cluster_connections_without_disabled_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn expect_first_cluster_connections_without_disabled_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_connections(
        configurator,
        first_cluster_connections().into_iter().skip(1).collect(),
        ExpectConnectionsParameters {
            cluster_offset: 0,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 0,
                handle: 0,
            }),
        },
    );
}

impl GenomeStubBuilder {
    fn add_initial_hox_gene_with_disabled_connection(&mut self) -> &mut Self {
        self.genome.hox_genes.push(HoxGene {
            placement: HoxPlacement::Standalone,
            cluster_index: ClusterGeneIndex(0),
            disabled_connections: vec![ConnectionFilter {
                from: NeuronClusterLocalIndex(0),
                to: NeuronClusterLocalIndex(1),
            }],
        });
        self
    }

    fn add_initial_hox_gene_with_valid_and_invalid_disabled_connection(&mut self) -> &mut Self {
        self.genome.hox_genes.push(HoxGene {
            placement: HoxPlacement::Standalone,
            cluster_index: ClusterGeneIndex(0),
            disabled_connections: vec![
                ConnectionFilter {
                    from: NeuronClusterLocalIndex(100),
                    to: NeuronClusterLocalIndex(50),
                },
                ConnectionFilter {
                    from: NeuronClusterLocalIndex(0),
                    to: NeuronClusterLocalIndex(1),
                },
            ],
        });
        self
    }
}
