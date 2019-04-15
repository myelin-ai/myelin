use crate::genome::{ClusterGene, ConnectionFilter, Genome, HoxGene, HoxPlacement};
use crate::orchestrator_impl::{NeuralNetworkConfigurator, NeuralNetworkDeveloper};
use crate::NeuralNetworkDevelopmentConfiguration;
use myelin_neural_network::Connection;

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
                    let neuron_handles: Vec<_> = cluster_gene
                        .neurons
                        .iter()
                        .map(|_| configurator.push_neuron())
                        .collect();
                    dbg!(&neuron_handles);
                    dbg!(&cluster_gene.connections);

                    let filtered_connections =
                        cluster_gene.connections.iter().filter_map(|connection| {
                            let connection_filter = ConnectionFilter {
                                from: connection.from,
                                to: connection.to,
                            };
                            dbg!(&hox_gene.disabled_connections);
                            dbg!(&connection_filter);
                            if !hox_gene.disabled_connections.contains(&connection_filter) {
                                let from = *neuron_handles.get(connection.from.0)?;
                                let to = *neuron_handles.get(connection.to.0)?;

                                Some((from, to, connection.weight))
                            } else {
                                None
                            }
                        });

                    for (from, to, weight) in filtered_connections {
                        configurator
                            .add_connection(Connection { from, to, weight })
                            .unwrap();
                    }
                }
                _ => unimplemented!(),
            }
        }
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
