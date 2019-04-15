use crate::genome::{self, ClusterGene, ConnectionFilter, Genome, HoxGene, HoxPlacement};
use crate::orchestrator_impl::{NeuralNetworkConfigurator, NeuralNetworkDeveloper};
use crate::NeuralNetworkDevelopmentConfiguration;
use myelin_neural_network::{Connection, Handle};

#[cfg(test)]
mod tests;

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
    fn develop_neural_network(self: Box<Self>, configurator: &mut dyn NeuralNetworkConfigurator) {
        let Self {
            genome: Genome {
                hox_genes,
                cluster_genes,
            },
            ..
        } = *self;

        let hox_genes_to_cluster_genes = map_hox_genes_to_cluster_genes(hox_genes, &cluster_genes);

        for (hox_gene, cluster_gene) in hox_genes_to_cluster_genes {
            match hox_gene.placement {
                HoxPlacement::Standalone => {
                    let neuron_handles = push_cluster_neurons(cluster_gene, configurator);
                    push_cluster_connections(
                        cluster_gene,
                        &hox_gene,
                        &neuron_handles,
                        configurator,
                    );
                }
                _ => unimplemented!(),
            }
        }
    }
}

fn push_cluster_neurons(
    cluster_gene: &ClusterGene,
    configurator: &mut dyn NeuralNetworkConfigurator,
) -> Vec<Handle> {
    cluster_gene
        .neurons
        .iter()
        .map(|_| configurator.push_neuron())
        .collect()
}

fn push_cluster_connections(
    cluster_gene: &ClusterGene,
    hox_gene: &HoxGene,
    neuron_handles: &[Handle],
    configurator: &mut dyn NeuralNetworkConfigurator,
) {
    cluster_gene
        .connections
        .iter()
        .filter_map(|connection| {
            filter_map_enabled_connection(connection, neuron_handles, hox_gene)
        })
        .for_each(|connection| {
            configurator.add_connection(connection).unwrap();
        });
}

fn filter_map_enabled_connection(
    connection: &genome::Connection,
    neuron_handles: &[Handle],
    hox_gene: &HoxGene,
) -> Option<Connection> {
    let connection_filter = ConnectionFilter {
        from: connection.from,
        to: connection.to,
    };
    if !hox_gene.disabled_connections.contains(&connection_filter) {
        let from = *neuron_handles.get(connection.from.0)?;
        let to = *neuron_handles.get(connection.to.0)?;
        let weight = connection.weight;
        Some(Connection { from, to, weight })
    } else {
        None
    }
}

fn map_hox_genes_to_cluster_genes<'a>(
    hox_genes: Vec<HoxGene>,
    cluster_genes: &'a [ClusterGene],
) -> impl Iterator<Item = (HoxGene, &'a ClusterGene)> {
    hox_genes.into_iter().filter_map(move |hox_gene| {
        let cluster_gene = cluster_genes.get(hox_gene.cluster_index.0)?;
        Some((hox_gene, cluster_gene))
    })
}
