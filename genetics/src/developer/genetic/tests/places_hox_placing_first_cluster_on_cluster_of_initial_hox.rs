use super::*;

#[test]
fn places_hox_placing_first_cluster_on_cluster_of_initial_hox() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);

    let genome = add_initial_hox_gene_to_genome(genome);
    let genome = add_hox_gene_placing_first_cluster_on_first_hox_clusters(genome);

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 7);
    expect_first_cluster_connections(&mut configurator);
    expect_first_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn add_hox_gene_placing_first_cluster_on_first_hox_clusters(mut genome: Genome) -> Genome {
    genome.hox_genes.insert(
        1,
        HoxGene {
            placement: HoxPlacement::HoxGene {
                hox_gene: HoxGeneIndex(0),
                target_neuron: NeuronClusterLocalIndex(3),
            },
            cluster_index: ClusterGeneIndex(0),
            disabled_connections: Vec::new(),
        },
    );
    genome
}

fn add_first_cluster_to_genome(mut genome: Genome) -> Genome {
    genome.cluster_genes.insert(
        0,
        ClusterGene {
            neurons: vec![Neuron {}, Neuron {}, Neuron {}, Neuron {}],
            connections: first_cluster_connections(),
            placement_neuron: NeuronClusterLocalIndex(1),
        },
    );

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

fn expect_first_cluster_placed_on_first_cluster_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    first_cluster_connections()
        .into_iter()
        .map(|connection_definition| {
            connection_definition_to_placed_connection(ConnectionTranslationParameters {
                connection: connection_definition,
                offset: 4,
                placement_neuron_index: 1,
            })
        })
        .for_each(|connection| {
            configurator
                .expect_add_connection(partial_eq(connection))
                .returns(Ok(()));
        });
}

fn first_cluster_connections() -> Vec<ConnectionDefinition> {
    vec![
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(0),
            to: NeuronClusterLocalIndex(1),
            weight: 0.5,
        },
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(0),
            to: NeuronClusterLocalIndex(2),
            weight: 0.7,
        },
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(0),
            to: NeuronClusterLocalIndex(3),
            weight: 0.2,
        },
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(3),
            to: NeuronClusterLocalIndex(1),
            weight: 0.3,
        },
    ]
}
