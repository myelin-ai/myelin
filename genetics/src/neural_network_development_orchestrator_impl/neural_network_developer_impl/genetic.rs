use crate::genome::{self, *};
use crate::neural_network_development_orchestrator_impl::{
    NeuralNetworkConfigurator, NeuralNetworkDeveloper,
};
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
            match resolve_hox_placement(&hox_gene.placement_target, &placed_cluster_cache) {
                ResolvedPlacement::Standalone => {
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
                ResolvedPlacement::OnPlacedClusters(placed_clusters, target_neuron_index) => {
                    let placed_clusters: Vec<_> = placed_clusters
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

                    for placed_cluster in placed_clusters {
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

fn resolve_hox_placement<'a>(
    placement: &HoxPlacement,
    placed_cluster_cache: &'a PlacedClusterCache,
) -> ResolvedPlacement<'a> {
    match placement {
        HoxPlacement::Standalone => ResolvedPlacement::Standalone,
        HoxPlacement::ClusterGene {
            cluster_gene,
            target_neuron,
        } => {
            let placed_clusters = placed_cluster_cache
                .cluster_gene_to_placed_clusters
                .get(cluster_gene);

            ResolvedPlacement::OnPlacedClusters(placed_clusters, *target_neuron)
        }
        HoxPlacement::HoxGene {
            hox_gene,
            target_neuron,
        } => {
            let placed_clusters = placed_cluster_cache
                .hox_gene_to_placed_clusters
                .get(hox_gene);

            ResolvedPlacement::OnPlacedClusters(placed_clusters, *target_neuron)
        }
    }
}

enum ResolvedPlacement<'a> {
    Standalone,
    OnPlacedClusters(Option<&'a Vec<PlacedCluster>>, ClusterNeuronIndex),
}

type PlacedCluster = Vec<Handle>;

fn push_standalone_cluster_neurons(
    cluster_gene: &ClusterGene,
    configurator: &mut dyn NeuralNetworkConfigurator,
) -> Vec<Handle> {
    cluster_gene
        .neurons
        .iter()
        .enumerate()
        .map(|(neuron_index, _)| ClusterNeuronIndex(neuron_index))
        .map(|neuron_index| push_neuron(configurator, neuron_index, &cluster_gene.specialization))
        .collect()
}

fn push_targeted_cluster_neurons(
    cluster_gene: &ClusterGene,
    placed_cluster: &[Handle],
    target_neuron_index: ClusterNeuronIndex,
    configurator: &mut dyn NeuralNetworkConfigurator,
) -> Vec<Handle> {
    let target_neuron_handle = placed_cluster
        .get(target_neuron_index.0)
        .expect("Target neuron index out of bounds");
    cluster_gene
        .neurons
        .iter()
        .enumerate()
        .map(|(neuron_index, _)| ClusterNeuronIndex(neuron_index))
        // This preserves the order of neurons
        .map(|neuron_index| {
            if neuron_index == cluster_gene.placement_neuron {
                *target_neuron_handle
            } else {
                push_neuron(configurator, neuron_index, &cluster_gene.specialization)
            }
        })
        .collect()
}

fn push_neuron(
    configurator: &mut dyn NeuralNetworkConfigurator,
    neuron_index: ClusterNeuronIndex,
    cluster_specialization: &ClusterGeneSpecialization,
) -> Handle {
    match cluster_specialization {
        ClusterGeneSpecialization::Input(input_neuron_index)
            if neuron_index == *input_neuron_index =>
        {
            configurator.push_input_neuron()
        }
        ClusterGeneSpecialization::Output(output_neuron_index)
            if neuron_index == *output_neuron_index =>
        {
            configurator.push_output_neuron()
        }
        _ => configurator.push_neuron(),
    }
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
            .entry(hox_gene.cluster_gene)
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
        .enumerate()
        .filter_map(|(index, connection)| {
            filter_map_enabled_connection(
                ClusterConnectionIndex(index),
                connection,
                neuron_handles,
                hox_gene,
            )
        })
        .for_each(|connection| {
            configurator.add_connection(connection).unwrap();
        });
}

fn filter_map_enabled_connection(
    index: ClusterConnectionIndex,
    connection: &genome::Connection,
    neuron_handles: &[Handle],
    hox_gene: &HoxGene,
) -> Option<Connection> {
    if !hox_gene.disabled_connections.contains(&index) {
        let from = *neuron_handles.get(connection.from.0)?;
        let to = *neuron_handles.get(connection.to.0)?;
        let weight = connection.weight;
        Some(Connection { from, to, weight })
    } else {
        None
    }
}

fn map_hox_genes_to_cluster_genes(
    hox_genes: Vec<HoxGene>,
    cluster_genes: &[ClusterGene],
) -> impl Iterator<Item = (HoxGene, &ClusterGene)> {
    hox_genes.into_iter().filter_map(move |hox_gene| {
        let cluster_gene = cluster_genes.get(hox_gene.cluster_gene.0)?;
        Some((hox_gene, cluster_gene))
    })
}
