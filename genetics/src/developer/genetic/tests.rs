use super::*;
use crate::genome::{
    ClusterGene, ClusterGeneIndex, Connection as ConnectionDefinition, HoxGene, HoxGeneIndex,
    HoxPlacement, Neuron, NeuronClusterLocalIndex,
};
use crate::orchestrator_impl::NeuralNetworkConfiguratorMock;
use mockiato::partial_eq;
use myelin_neural_network::Connection;
use myelin_neural_network::Handle;
use std::cmp::Ordering;
use std::num::NonZeroUsize;

mod clusters_are_only_placed_on_specified_targets;
mod creates_standalone_cluster;
mod places_cluster;
mod places_cluster_on_hox_target;
mod places_hox_placing_first_cluster_on_cluster_of_initial_hox;
mod places_nothing_when_genome_is_empty;
mod places_nothing_when_genome_only_contains_clusters_genes;
mod places_nothing_when_hox_gene_points_to_non_existent_cluster_gene;
mod places_two_hox_genes_placing_first_cluster_on_cluster_of_initial_hox;

fn connection_definition_to_connection(connection_definition: ConnectionDefinition) -> Connection {
    Connection {
        from: Handle(connection_definition.from.0),
        to: Handle(connection_definition.to.0),
        weight: connection_definition.weight,
    }
}

fn connection_definition_to_placed_connection(
    connection_translation_parameters: ConnectionTranslationParameters,
) -> Connection {
    let ConnectionTranslationParameters {
        connection,
        cluster_offset,
        placement_neuron_index,
        placement_neuron_handle,
    } = connection_translation_parameters;

    let translate_index_to_handle = move |index: NeuronClusterLocalIndex| {
        let index = index.0;
        let translated_index = match index.cmp(&placement_neuron_index) {
            Ordering::Equal => placement_neuron_handle,
            Ordering::Less => cluster_offset + index,
            Ordering::Greater => cluster_offset + index - 1,
        };
        Handle(translated_index)
    };

    Connection {
        from: translate_index_to_handle(connection.from),
        to: translate_index_to_handle(connection.to),
        weight: connection.weight,
    }
}

struct ConnectionTranslationParameters {
    connection: ConnectionDefinition,
    cluster_offset: usize,
    placement_neuron_index: usize,
    placement_neuron_handle: usize,
}

fn expect_push_amount_of_neurons(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
    neuron_count: usize,
) {
    for handle in 0..neuron_count {
        configurator.expect_push_neuron().returns(Handle(handle));
    }
    configurator.expect_push_neuron_calls_in_order();
}

fn config_stub() -> NeuralNetworkDevelopmentConfiguration {
    NeuralNetworkDevelopmentConfiguration {
        parent_genomes: (Genome::default(), Genome::default()),
        input_neuron_count: NonZeroUsize::new(1).unwrap(),
        output_neuron_count: NonZeroUsize::new(1).unwrap(),
    }
}

fn genome_stub() -> Genome {
    Genome::default()
}

fn add_initial_hox_gene_to_genome(mut genome: Genome) -> Genome {
    genome.hox_genes.insert(
        0,
        HoxGene {
            placement: HoxPlacement::Standalone,
            cluster_index: ClusterGeneIndex(0),
            disabled_connections: vec![],
        },
    );

    genome
}

fn add_first_cluster_to_genome(mut genome: Genome) -> Genome {
    genome.cluster_genes.insert(
        0,
        ClusterGene {
            neurons: vec![Neuron::new(); 4],
            connections: first_cluster_connections(),
            placement_neuron: NeuronClusterLocalIndex(1),
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

fn add_hox_gene_placing_cluster_on_cluster(
    genome: &mut Genome,
    parameters: ClusterOnClusterTestParameters,
) {
    genome.hox_genes.push(HoxGene {
        placement: HoxPlacement::ClusterGene {
            cluster_gene: parameters.cluster_gene,
            target_neuron: parameters.target_neuron,
        },
        cluster_index: parameters.cluster_index,
        disabled_connections: Vec::new(),
    })
}

struct ClusterOnClusterTestParameters {
    cluster_gene: ClusterGeneIndex,
    target_neuron: NeuronClusterLocalIndex,
    cluster_index: ClusterGeneIndex,
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

fn expect_first_cluster_placed_on_hox(
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, usize, usize, usize) {
    expect_cluster_placed_on_hox(first_cluster_connections())
}

fn expect_second_cluster_placed_on_hox(
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, usize, usize, usize) {
    expect_cluster_placed_on_hox(second_cluster_connections())
}

fn expect_cluster_placed_on_hox(
    connections: Vec<ConnectionDefinition>,
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, usize, usize, usize) {
    move |configurator: &mut NeuralNetworkConfiguratorMock<'_>,
          cluster_offset: usize,
          placement_neuron_index: usize,
          placement_neuron_handle: usize| {
        connections
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
            })
    }
}

fn add_second_cluster_to_genome(mut genome: Genome) -> Genome {
    genome.cluster_genes.insert(
        1,
        ClusterGene {
            neurons: vec![Neuron::new(); 3],
            connections: second_cluster_connections(),
            placement_neuron: NeuronClusterLocalIndex(0),
        },
    );

    genome
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
