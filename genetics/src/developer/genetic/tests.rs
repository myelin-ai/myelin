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

mod creates_standalone_cluster;
mod places_cluster;
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
        offset,
        placement_neuron_index,
    } = connection_translation_parameters;

    let translate_index_to_handle = move |index: NeuronClusterLocalIndex| {
        let index = index.0;
        let translated_index = match index.cmp(&placement_neuron_index) {
            Ordering::Equal => index,
            Ordering::Less => offset + index,
            Ordering::Greater => offset + index - 1,
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
    offset: usize,
    placement_neuron_index: usize,
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
