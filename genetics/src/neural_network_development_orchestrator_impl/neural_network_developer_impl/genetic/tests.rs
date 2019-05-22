use super::*;
use crate::genome::{
    ClusterGene, ClusterGeneIndex, ClusterGeneSpecialization, ClusterNeuronIndex,
    Connection as ConnectionDefinition, HoxGene, HoxGeneIndex, HoxPlacement, Neuron,
};
use crate::neural_network_development_orchestrator_impl::NeuralNetworkConfiguratorMock;
use mockiato::partial_eq;
use myelin_neural_network::Connection;
use myelin_neural_network::Handle;
use std::cmp::Ordering;
use std::num::NonZeroUsize;

mod clusters_are_only_placed_on_specified_targets;
mod creates_standalone_cluster;
mod does_not_create_connections_disabled_on_hox_gene;
mod places_cluster;
mod places_cluster_on_hox_target;
mod places_hox_placing_first_cluster_on_cluster_of_initial_hox;
mod places_nothing_when_genome_is_empty;
mod places_nothing_when_genome_only_contains_clusters_genes;
mod places_nothing_when_hox_gene_points_to_non_existent_cluster_gene;
mod places_two_hox_genes_placing_first_cluster_on_cluster_of_initial_hox;
mod places_two_standalone_clusters;
mod respects_cluster_specializations;

#[derive(Debug, Default)]
struct GenomeStubBuilder {
    genome: Genome,
}

impl GenomeStubBuilder {
    fn new() -> Self {
        GenomeStubBuilder::default()
    }

    fn add_first_cluster(&mut self) -> &mut Self {
        self.add_first_cluster_with_specialization(ClusterGeneSpecialization::default())
    }

    fn add_first_cluster_with_specialization(
        &mut self,
        specialization: ClusterGeneSpecialization,
    ) -> &mut Self {
        self.genome.cluster_genes.push(ClusterGene {
            neurons: vec![Neuron::new(); 4],
            connections: first_cluster_connections(),
            placement_neuron: ClusterNeuronIndex(1),
            specialization,
        });
        self
    }

    fn add_second_cluster(&mut self) -> &mut Self {
        self.add_second_cluster_with_specialization(ClusterGeneSpecialization::default())
    }

    fn add_second_cluster_with_specialization(
        &mut self,
        specialization: ClusterGeneSpecialization,
    ) -> &mut Self {
        self.genome.cluster_genes.push(ClusterGene {
            neurons: vec![Neuron::new(); 3],
            connections: second_cluster_connections(),
            placement_neuron: ClusterNeuronIndex(0),
            specialization,
        });
        self
    }

    fn add_third_cluster_with_specialization(
        &mut self,
        specialization: ClusterGeneSpecialization,
    ) -> &mut Self {
        self.genome.cluster_genes.push(ClusterGene {
            neurons: vec![Neuron::new(); 4],
            connections: third_cluster_connections(),
            placement_neuron: ClusterNeuronIndex(3),
            specialization,
        });
        self
    }

    fn add_initial_hox_gene(&mut self) -> &mut Self {
        self.genome.hox_genes.push(HoxGene {
            placement_target: HoxPlacement::Standalone,
            cluster_gene: ClusterGeneIndex(0),
            disabled_connections: vec![],
        });
        self
    }

    fn add_hox_gene_placing_second_cluster_on_first_cluster(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_cluster(ClusterOnClusterTestParameters {
            target_cluster_gene: ClusterGeneIndex(0),
            target_neuron: ClusterNeuronIndex(2),
            cluster_gene: ClusterGeneIndex(1),
        });
        self
    }

    fn add_hox_gene_placing_cluster_on_cluster(
        &mut self,
        ClusterOnClusterTestParameters {
            target_cluster_gene,
            target_neuron,
            cluster_gene,
        }: ClusterOnClusterTestParameters,
    ) -> &mut Self {
        self.genome.hox_genes.push(HoxGene {
            placement_target: HoxPlacement::ClusterGene {
                cluster_gene: target_cluster_gene,
                target_neuron,
            },
            cluster_gene,
            disabled_connections: Vec::new(),
        });
        self
    }

    fn add_hox_gene_placing_cluster_on_hox(
        &mut self,
        parameters: ClusterOnHoxTestParameters,
    ) -> &mut Self {
        self.genome.hox_genes.push(HoxGene {
            placement_target: HoxPlacement::HoxGene {
                hox_gene: parameters.hox_gene,
                target_neuron: parameters.target_neuron,
            },
            cluster_gene: parameters.cluster_gene,
            disabled_connections: Vec::new(),
        });
        self
    }

    fn build(&self) -> Genome {
        self.genome.clone()
    }
}

fn connection_definition_to_placed_connection(
    connection_translation_parameters: ConnectionTranslationParameters,
) -> Connection {
    let ConnectionTranslationParameters {
        connection,
        cluster_offset,
        placement_neuron,
    } = connection_translation_parameters;

    let translate_index_to_handle = move |index: ClusterNeuronIndex| {
        let index = index.0;
        Handle(match placement_neuron {
            Some(PlacementNeuronTranslation {
                index: placement_neuron_index,
                handle: placement_neuron_handle,
            }) => {
                match index.cmp(&placement_neuron_index) {
                    // Use the global handle passed to the function
                    Ordering::Equal => placement_neuron_handle,
                    // Calculate the global handle by adding the offset
                    Ordering::Less => cluster_offset + index,
                    // Because we handled the `Equal` case already, we are off by one
                    Ordering::Greater => cluster_offset + index - 1,
                }
            }
            None => cluster_offset + index,
        })
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
    placement_neuron: Option<PlacementNeuronTranslation>,
}

fn expect_push_amount_of_neurons(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
    neuron_count: usize,
) {
    let neuron_handles = NeuronHandles {
        regular: (0..neuron_count).collect(),
        ..NeuronHandles::default()
    };
    expect_push_different_kinds_of_neurons(configurator, neuron_handles)
}

#[derive(Debug, Default)]
struct NeuronHandles {
    regular: Vec<usize>,
    input: Vec<usize>,
    output: Vec<usize>,
}

fn expect_push_different_kinds_of_neurons(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
    neuron_handles: NeuronHandles,
) {
    let NeuronHandles {
        regular,
        input,
        output,
    } = neuron_handles;

    for handle in regular {
        configurator.expect_push_neuron().returns(Handle(handle));
    }

    for handle in input {
        configurator
            .expect_push_input_neuron()
            .returns(Handle(handle));
    }

    for handle in output {
        configurator
            .expect_push_output_neuron()
            .returns(Handle(handle));
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

struct ClusterOnClusterTestParameters {
    target_cluster_gene: ClusterGeneIndex,
    target_neuron: ClusterNeuronIndex,
    cluster_gene: ClusterGeneIndex,
}

struct ClusterOnHoxTestParameters {
    hox_gene: HoxGeneIndex,
    target_neuron: ClusterNeuronIndex,
    cluster_gene: ClusterGeneIndex,
}

fn first_cluster_connections() -> Vec<ConnectionDefinition> {
    vec![
        ConnectionDefinition {
            from: ClusterNeuronIndex(0),
            to: ClusterNeuronIndex(1),
            weight: 0.5,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(0),
            to: ClusterNeuronIndex(2),
            weight: 0.7,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(0),
            to: ClusterNeuronIndex(3),
            weight: 0.2,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(3),
            to: ClusterNeuronIndex(1),
            weight: 0.3,
        },
    ]
}

fn expect_first_cluster_placed_on_hox(
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, ExpectConnectionsParameters) {
    expect_cluster_placed_on_hox(first_cluster_connections())
}

fn expect_second_cluster_placed_on_hox(
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, ExpectConnectionsParameters) {
    expect_cluster_placed_on_hox(second_cluster_connections())
}

fn expect_third_cluster_placed_on_hox(
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, ExpectConnectionsParameters) {
    expect_cluster_placed_on_hox(third_cluster_connections())
}

fn expect_cluster_placed_on_hox(
    connections: Vec<ConnectionDefinition>,
) -> impl FnOnce(&mut NeuralNetworkConfiguratorMock<'_>, ExpectConnectionsParameters) {
    move |configurator: &mut NeuralNetworkConfiguratorMock<'_>,
          parameters: ExpectConnectionsParameters| {
        expect_connections(configurator, connections, parameters);
    }
}

fn expect_first_cluster_placed_standalone(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
    cluster_offset: usize,
) {
    expect_connections(
        configurator,
        first_cluster_connections(),
        ExpectConnectionsParameters {
            cluster_offset,
            placement_neuron: None,
        },
    )
}

fn expect_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
    connections: Vec<ConnectionDefinition>,
    ExpectConnectionsParameters {
        cluster_offset,
        placement_neuron,
    }: ExpectConnectionsParameters,
) {
    connections
        .into_iter()
        .map(|connection| {
            connection_definition_to_placed_connection(ConnectionTranslationParameters {
                connection,
                cluster_offset,
                placement_neuron: placement_neuron.clone(),
            })
        })
        .for_each(|connection| {
            configurator
                .expect_add_connection(partial_eq(connection))
                .returns(Ok(()));
        })
}

struct ExpectConnectionsParameters {
    cluster_offset: usize,
    placement_neuron: Option<PlacementNeuronTranslation>,
}

#[derive(Debug, Clone)]
struct PlacementNeuronTranslation {
    index: usize,
    handle: usize,
}

fn second_cluster_connections() -> Vec<ConnectionDefinition> {
    vec![
        ConnectionDefinition {
            from: ClusterNeuronIndex(0),
            to: ClusterNeuronIndex(2),
            weight: 0.4,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(1),
            to: ClusterNeuronIndex(2),
            weight: 0.6,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(2),
            to: ClusterNeuronIndex(0),
            weight: 0.45,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(2),
            to: ClusterNeuronIndex(1),
            weight: 0.82,
        },
    ]
}

fn third_cluster_connections() -> Vec<ConnectionDefinition> {
    vec![
        ConnectionDefinition {
            from: ClusterNeuronIndex(0),
            to: ClusterNeuronIndex(2),
            weight: 1.0,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(3),
            to: ClusterNeuronIndex(2),
            weight: 0.2,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(0),
            to: ClusterNeuronIndex(3),
            weight: 0.99,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(3),
            to: ClusterNeuronIndex(1),
            weight: 0.1,
        },
        ConnectionDefinition {
            from: ClusterNeuronIndex(1),
            to: ClusterNeuronIndex(0),
            weight: 0.5,
        },
    ]
}
