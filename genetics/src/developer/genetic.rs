use crate::genome::{self, *};
use crate::orchestrator_impl::{NeuralNetworkConfigurator, NeuralNetworkDeveloper};
use crate::NeuralNetworkDevelopmentConfiguration;
use myelin_neural_network::{Connection, Handle};
use std::collections::HashMap;

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
        let mut placed_cluster_cache = PlacedClusterCache::default();

        for (hox_index, (hox_gene, cluster_gene)) in hox_genes_to_cluster_genes.enumerate() {
            match hox_gene.placement_target {
                HoxPlacement::Standalone => {
                    let placed_cluster =
                        push_standalone_cluster_neurons(cluster_gene, configurator);
                    push_cluster_connections(
                        cluster_gene,
                        &hox_gene,
                        &placed_cluster,
                        configurator,
                    );
                    placed_cluster_cache.add_placed_cluster_to_cache(
                        &hox_gene,
                        HoxGeneIndex(hox_index),
                        placed_cluster,
                    );
                }
                HoxPlacement::ClusterGene {
                    cluster_gene: target_cluster_gene_index,
                    target_neuron: target_neuron_index,
                } => {
                    let placed_clusters: Vec<_> = placed_cluster_cache
                        .cluster_gene_to_placed_clusters
                        .get(&target_cluster_gene_index)
                        .iter()
                        .flat_map(|placed_clusters| placed_clusters.iter())
                        .map(|placed_cluster| {
                            push_targeted_cluster_neurons(
                                cluster_gene,
                                placed_cluster,
                                target_neuron_index,
                                configurator,
                            )
                        })
                        .collect();

                     for placed_cluster in placed_clusters.into_iter() {
                        push_cluster_connections(
                            cluster_gene,
                            &hox_gene,
                            &placed_cluster,
                            configurator,
                        );
                        placed_cluster_cache.add_placed_cluster_to_cache(
                            &hox_gene,
                            HoxGeneIndex(hox_index),
                            placed_cluster,
                        );
                    }
                    
                },
                HoxPlacement::HoxGene {
                    hox_gene: target_hox_gene_index,
                    target_neuron: target_neuron_index,
                } => {
                    let placed_clusters: Vec<_> = placed_cluster_cache
                        .hox_gene_to_placed_clusters
                        .get(&target_hox_gene_index)
                        .iter()
                        .flat_map(|placed_clusters| placed_clusters.iter())
                        .map(|placed_cluster| {
                            push_targeted_cluster_neurons(
                                cluster_gene,
                                placed_cluster,
                                target_neuron_index,
                                configurator,
                            )
                        })
                        .collect();

                    for placed_cluster in placed_clusters.into_iter() {
                        push_cluster_connections(
                            cluster_gene,
                            &hox_gene,
                            &placed_cluster,
                            configurator,
                        );
                        placed_cluster_cache.add_placed_cluster_to_cache(
                            &hox_gene,
                            HoxGeneIndex(hox_index),
                            placed_cluster,
                        );
                    }
                }
            }
        }
    }
}

type PlacedCluster = Vec<Handle>;

fn push_standalone_cluster_neurons(
    cluster_gene: &ClusterGene,
    configurator: &mut dyn NeuralNetworkConfigurator,
) -> Vec<Handle> {
    cluster_gene
        .neurons
        .iter()
        .map(|_| configurator.push_neuron())
        .collect()
}

fn push_targeted_cluster_neurons(
    cluster_gene: &ClusterGene,
    placed_cluster: &[Handle],
    target_neuron_index: NeuronClusterLocalIndex,
    configurator: &mut dyn NeuralNetworkConfigurator,
) -> Vec<Handle> {
    let target_neuron_handle = placed_cluster
        .get(target_neuron_index.0)
        .expect("Target neuron index out of bounds");
    cluster_gene
        .neurons
        .iter()
        .enumerate()
        // This preserves the order of neurons
        .map(|(index, _)| {
            if index == cluster_gene.placement_neuron.0 {
                *target_neuron_handle
            } else {
                configurator.push_neuron()
            }
        })
        .collect()
}

#[derive(Debug, Default)]
struct PlacedClusterCache {
    cluster_gene_to_placed_clusters: HashMap<ClusterGeneIndex, Vec<PlacedCluster>>,
    hox_gene_to_placed_clusters: HashMap<HoxGeneIndex, Vec<PlacedCluster>>,
}

impl PlacedClusterCache {
    fn add_placed_cluster_to_cache(
        &mut self,
        hox_gene: &HoxGene,
        hox_index: HoxGeneIndex,
        placed_cluster: Vec<Handle>,
    ) {
        self.cluster_gene_to_placed_clusters
            .entry(hox_gene.cluster_index)
            .or_default()
            .push(placed_cluster.clone());
        self.hox_gene_to_placed_clusters
            .entry(hox_index)
            .or_default()
            .push(placed_cluster);
    }
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
