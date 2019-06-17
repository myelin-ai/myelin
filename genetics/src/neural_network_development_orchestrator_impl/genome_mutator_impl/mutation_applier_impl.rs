use super::{Mutation, MutationApplier};
use crate::genome::*;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

/// Default implementation of [`MutationApplier`].
#[derive(Debug, Default)]
pub struct MutationApplierImpl(PhantomData<()>);

impl MutationApplierImpl {
    /// Creates a new [`MutationApplierImpl`].
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl MutationApplier for MutationApplierImpl {
    fn apply_mutation(
        &self,
        genome: &mut Genome,
        mutation: Mutation,
    ) -> Result<(), Box<dyn Error>> {
        match mutation {
            Mutation::AddNeuron {
                cluster_gene,
                connection,
                new_connection_weight,
            } => genome.add_neuron(cluster_gene, connection, new_connection_weight),

            Mutation::AddConnection {
                cluster_gene,
                connection,
            } => genome.add_connection(cluster_gene, connection),

            Mutation::DisableConnection {
                hox_gene,
                connection,
            } => genome.disable_connection(hox_gene, connection),

            Mutation::NudgeWeight {
                cluster_gene,
                connection,
                weight_delta,
            } => genome.nudge_weight(cluster_gene, connection, weight_delta),

            Mutation::ChangePlacementNeuron {
                cluster_gene,
                new_placement_neuron,
            } => genome.change_placement_neuron(cluster_gene, new_placement_neuron),

            Mutation::AddNewCluster {
                cluster_gene,
                hox_gene,
            } => genome.add_new_cluster(cluster_gene, hox_gene),

            Mutation::CopyCluster {
                source_cluster_gene,
                new_hox_gene,
            } => genome.copy_cluster(source_cluster_gene, new_hox_gene),

            Mutation::DesyncCluster { hox_gene } => genome.desync_cluster(hox_gene),

            Mutation::Bridge {
                target_hox_gene,
                bridge_cluster_gene,
            } => genome.bridge(target_hox_gene, bridge_cluster_gene),

            Mutation::AddHoxWithExistingCluster { new_hox_gene } => {
                genome.add_hox_with_existing_cluster(new_hox_gene)
            }

            Mutation::ChangeTargetNeuron {
                hox_gene,
                new_target_neuron,
            } => genome.change_target_neuron(hox_gene, new_target_neuron),

            Mutation::DuplicateHox { hox_gene } => genome.duplicate_hox(hox_gene),
        }
    }
}

#[derive(Debug)]
enum MutationApplierError {
    IndexOutOfBounds,
    InvalidTarget,
}

impl Error for MutationApplierError {}

impl fmt::Display for MutationApplierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutationApplierError::IndexOutOfBounds => write!(f, "The given index does not exist"),
            MutationApplierError::InvalidTarget => write!(f, "The given target is invalid"),
        }
    }
}

impl Genome {
    fn add_neuron(
        &mut self,
        cluster_gene_index: ClusterGeneIndex,
        connection_index: ClusterConnectionIndex,
        new_connection_weight: Weight,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = self.get_cluster_gene_mut(cluster_gene_index)?;

        let existing_connection = get_connection(cluster_gene, connection_index)?;

        let to_neuron = existing_connection.to;

        let new_neuron_index = ClusterNeuronIndex(cluster_gene.neurons.len());
        let new_neuron = Neuron::new();

        cluster_gene.neurons.push(new_neuron);
        cluster_gene.connections.push(Connection {
            from: new_neuron_index,
            to: to_neuron,
            weight: new_connection_weight,
        });

        let existing_connection = get_connection_mut(cluster_gene, connection_index).unwrap();
        existing_connection.to = new_neuron_index;

        Ok(())
    }

    fn add_connection(
        &mut self,
        cluster_gene_index: ClusterGeneIndex,
        connection: Connection,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = self.get_cluster_gene_mut(cluster_gene_index)?;

        cluster_gene.connections.push(connection);

        Ok(())
    }

    fn disable_connection(
        &mut self,
        hox_gene_index: HoxGeneIndex,
        connection_index: ClusterConnectionIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = self.get_hox_gene_mut(hox_gene_index)?;

        if gene
            .disabled_connections
            .iter()
            .all(|disabled_connection| *disabled_connection != connection_index)
        {
            gene.disabled_connections.push(connection_index);
        }

        Ok(())
    }

    fn nudge_weight(
        &mut self,
        cluster_gene_index: ClusterGeneIndex,
        connection_index: ClusterConnectionIndex,
        weight_delta: Weight,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = self.get_cluster_gene_mut(cluster_gene_index)?;
        let connection = get_connection_mut(cluster_gene, connection_index)?;

        connection.weight += weight_delta;

        Ok(())
    }

    fn change_placement_neuron(
        &mut self,
        cluster_gene_index: ClusterGeneIndex,
        new_placement_neuron_index: ClusterNeuronIndex,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = self.get_cluster_gene_mut(cluster_gene_index)?;

        cluster_gene.placement_neuron = new_placement_neuron_index;

        Ok(())
    }

    fn add_new_cluster(
        &mut self,
        cluster_gene: ClusterGene,
        hox_gene: HoxGene,
    ) -> Result<(), Box<dyn Error>> {
        self.cluster_genes.push(cluster_gene);
        self.hox_genes.push(hox_gene);

        Ok(())
    }

    fn copy_cluster(
        &mut self,
        cluster_gene_index: ClusterGeneIndex,
        hox_gene: HoxGene,
    ) -> Result<(), Box<dyn Error>> {
        let new_cluster_gene = self.get_cluster_gene(cluster_gene_index)?.clone();

        self.cluster_genes.push(new_cluster_gene);
        self.hox_genes.push(hox_gene);

        Ok(())
    }

    fn desync_cluster(&mut self, hox_gene_index: HoxGeneIndex) -> Result<(), Box<dyn Error>> {
        let hox_gene = self.get_hox_gene(hox_gene_index)?;
        let cluster_gene_index = hox_gene.cluster_gene;

        let new_cluster_gene = self.get_cluster_gene(cluster_gene_index)?.clone();

        let new_cluster_gene_index = ClusterGeneIndex(self.cluster_genes.len());
        self.cluster_genes.push(new_cluster_gene);

        let hox_gene = self.get_hox_gene_mut(hox_gene_index).unwrap();
        hox_gene.cluster_gene = new_cluster_gene_index;

        Ok(())
    }

    fn bridge(
        &mut self,
        hox_gene_index: HoxGeneIndex,
        bridge_cluster_gene: ClusterGene,
    ) -> Result<(), Box<dyn Error>> {
        let _ = self.get_hox_gene(hox_gene_index)?;

        let bridge_cluster_gene_index = ClusterGeneIndex(self.cluster_genes.len());
        self.cluster_genes.push(bridge_cluster_gene);

        let existing_hox_gene = self.get_hox_gene(hox_gene_index).unwrap();

        let bridge_hox_gene = HoxGene {
            placement_target: existing_hox_gene.placement_target.clone(),
            cluster_gene: bridge_cluster_gene_index,
            disabled_connections: vec![],
        };

        let bridge_hox_gene_index = HoxGeneIndex(self.hox_genes.len());
        self.hox_genes.push(bridge_hox_gene);

        let existing_hox_gene = self.get_hox_gene_mut(hox_gene_index).unwrap();

        existing_hox_gene.placement_target = HoxPlacement::HoxGene {
            hox_gene: bridge_hox_gene_index,
            target_neuron: ClusterNeuronIndex(1),
        };

        Ok(())
    }

    fn add_hox_with_existing_cluster(&mut self, hox_gene: HoxGene) -> Result<(), Box<dyn Error>> {
        self.hox_genes.push(hox_gene);

        Ok(())
    }

    fn change_target_neuron(
        &mut self,
        hox_gene_index: HoxGeneIndex,
        new_target_neuron_index: ClusterNeuronIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = self.get_hox_gene_mut(hox_gene_index)?;

        gene.placement_target = match gene.placement_target {
            HoxPlacement::ClusterGene { cluster_gene, .. } => HoxPlacement::ClusterGene {
                cluster_gene,
                target_neuron: new_target_neuron_index,
            },
            HoxPlacement::HoxGene { hox_gene, .. } => HoxPlacement::HoxGene {
                hox_gene,
                target_neuron: new_target_neuron_index,
            },
            HoxPlacement::Standalone => Err(MutationApplierError::InvalidTarget)?,
        };

        Ok(())
    }

    fn duplicate_hox(&mut self, hox_gene_index: HoxGeneIndex) -> Result<(), Box<dyn Error>> {
        let gene = self.get_hox_gene(hox_gene_index)?.clone();

        self.hox_genes.push(gene);

        Ok(())
    }

    fn get_hox_gene(&self, index: HoxGeneIndex) -> Result<&HoxGene, Box<dyn Error>> {
        self.hox_genes
            .get(index.0)
            .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
    }

    fn get_hox_gene_mut(&mut self, index: HoxGeneIndex) -> Result<&mut HoxGene, Box<dyn Error>> {
        self.hox_genes
            .get_mut(index.0)
            .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
    }

    fn get_cluster_gene(&self, index: ClusterGeneIndex) -> Result<&ClusterGene, Box<dyn Error>> {
        self.cluster_genes
            .get(index.0)
            .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
    }

    fn get_cluster_gene_mut(
        &mut self,
        index: ClusterGeneIndex,
    ) -> Result<&mut ClusterGene, Box<dyn Error>> {
        self.cluster_genes
            .get_mut(index.0)
            .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
    }
}

fn get_connection(
    cluster_gene: &ClusterGene,
    index: ClusterConnectionIndex,
) -> Result<&Connection, Box<dyn Error>> {
    cluster_gene
        .connections
        .get(index.0)
        .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
}

fn get_connection_mut(
    cluster_gene: &mut ClusterGene,
    index: ClusterConnectionIndex,
) -> Result<&mut Connection, Box<dyn Error>> {
    cluster_gene
        .connections
        .get_mut(index.0)
        .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
}
