use crate::genome::Genome;
use crate::orchestrator_impl::{NeuralNetworkConfigurator, NeuralNetworkDeveloper};
use crate::NeuralNetworkDevelopmentConfiguration;

/// Bootstraps the neural network based on the genome
#[derive(Debug, Clone)]
pub struct GeneticNeuralNetworkDeveloper {
    development_configuration: NeuralNetworkDevelopmentConfiguration,
    genome: Genome,
}

impl GeneticNeuralNetworkDeveloper {
    /// Creates a new [`GeneticNeuralNetworkDeveloper`].
    ///
    /// [`GeneticNeuralNetworkDeveloper`]: ./struct.GeneticNeuralNetworkDeveloper.html
    pub fn new(
        development_configuration: NeuralNetworkDevelopmentConfiguration,
        genome: Genome,
    ) -> Self {
        Self {
            development_configuration,
            genome,
        }
    }
}

impl NeuralNetworkDeveloper for GeneticNeuralNetworkDeveloper {
    fn develop_neural_network(self: Box<Self>, _configurator: &mut dyn NeuralNetworkConfigurator) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn creates_standalone_cluster() {
        let genome = genome_stub();
        let genome = add_first_cluster_to_genome(genome);
        let genome = add_initial_hox_gene_to_genome(genome);
        let config = config_stub();

        let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
        let mut configurator = NeuralNetworkConfiguratorMock::new();

        expect_push_amount_of_neurons(&mut configurator, 4);
        expect_first_cluster_connections(&mut configurator);

        developer.develop_neural_network(&mut configurator);
    }

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

    #[test]
    fn places_hox() {
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

    fn connection_definition_to_connection(
        connection_definition: ConnectionDefinition,
    ) -> Connection {
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
}
