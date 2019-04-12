use super::*;

#[test]
fn places_cluster() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);
    let genome = add_second_cluster_to_genome(genome);

    let genome = add_initial_hox_gene_to_genome(genome);
    let genome = add_hox_gene_placing_second_cluster_on_first_cluster(genome);

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 7);
    expect_first_cluster_connections(&mut configurator);
    expect_second_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
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

fn add_second_cluster_to_genome(mut genome: Genome) -> Genome {
    genome.cluster_genes.insert(
        1,
        ClusterGene {
            neurons: vec![Neuron {}, Neuron {}, Neuron {}],
            connections: second_cluster_connections(),
            placement_neuron: NeuronClusterLocalIndex(0),
        },
    );

    genome
}

fn add_hox_gene_placing_second_cluster_on_first_cluster(mut genome: Genome) -> Genome {
    genome.hox_genes.insert(
        1,
        HoxGene {
            placement: HoxPlacement::ClusterGene {
                cluster_gene: ClusterGeneIndex(0),
                target_neuron: NeuronClusterLocalIndex(2),
            },
            cluster_index: ClusterGeneIndex(1),
            disabled_connections: Vec::new(),
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

fn expect_second_cluster_placed_on_first_cluster_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    second_cluster_connections()
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

fn second_cluster_connections() -> Vec<ConnectionDefinition> {
    vec![
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(0),
            to: NeuronClusterLocalIndex(2),
            weight: 0.4,
        },
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(1),
            to: NeuronClusterLocalIndex(2),
            weight: 0.6,
        },
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(2),
            to: NeuronClusterLocalIndex(0),
            weight: 0.45,
        },
        ConnectionDefinition {
            from: NeuronClusterLocalIndex(2),
            to: NeuronClusterLocalIndex(1),
            weight: 0.82,
        },
    ]
}
