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
            } => self.add_neuron(genome, cluster_gene, connection, new_connection_weight),
            Mutation::AddConnection { .. } => unimplemented!(),
            Mutation::DisableConnection {
                hox_gene,
                connection,
            } => self.disable_connection(genome, hox_gene, connection),
            Mutation::NudgeWeight { .. } => unimplemented!(),
            Mutation::ChangePlacementNeuron { .. } => unimplemented!(),
            Mutation::AddNewCluster { .. } => unimplemented!(),
            Mutation::CopyCluster { .. } => unimplemented!(),
            Mutation::DesyncCluster { .. } => unimplemented!(),
            Mutation::Bridge { .. } => unimplemented!(),
            Mutation::AddHoxWithExistingCluster { .. } => unimplemented!(),
            Mutation::ChangeTargetNeuron {
                hox_gene,
                new_target_neuron,
            } => self.change_target_neuron(genome, hox_gene, new_target_neuron),
            Mutation::DuplicateHox { hox_gene } => self.duplicate_hox(genome, hox_gene),
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

impl MutationApplierImpl {
    fn add_neuron(
        &self,
        genome: &mut Genome,
        cluster_gene_index: ClusterGeneIndex,
        connection_index: ClusterConnectionIndex,
        new_connection_weight: Weight,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = get_cluster_gene_mut(genome, cluster_gene_index)?;

        let existing_connection = get_connection(cluster_gene, connection_index)?;

        let to_neuron = existing_connection.to.clone();

        let new_neuron_index = ClusterNeuronIndex(cluster_gene.neurons.len());
        let new_neuron = Neuron::new();

        cluster_gene.neurons.push(new_neuron);
        cluster_gene.connections.push(Connection {
            from: new_neuron_index,
            to: to_neuron,
            weight: new_connection_weight,
        });

        let existing_connection = get_connection_mut(cluster_gene, connection_index)?;
        existing_connection.to = new_neuron_index;

        Ok(())
    }

    fn disable_connection(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
        connection_index: ClusterConnectionIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = get_hox_gene_mut(genome, hox_gene_index)?;

        if !gene
            .disabled_connections
            .iter()
            .any(|disabled_connection| *disabled_connection == connection_index)
        {
            gene.disabled_connections.push(connection_index);
        }

        Ok(())
    }

    fn duplicate_hox(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = get_hox_gene(genome, hox_gene_index)?.clone();

        genome.hox_genes.push(gene);

        Ok(())
    }

    fn change_target_neuron(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
        new_target_neuron_index: ClusterNeuronIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = get_hox_gene_mut(genome, hox_gene_index)?;

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
}

fn get_hox_gene(genome: &Genome, index: HoxGeneIndex) -> Result<&HoxGene, Box<dyn Error>> {
    genome
        .hox_genes
        .get(index.0)
        .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
}

fn get_hox_gene_mut(
    genome: &mut Genome,
    index: HoxGeneIndex,
) -> Result<&mut HoxGene, Box<dyn Error>> {
    genome
        .hox_genes
        .get_mut(index.0)
        .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
}

fn get_cluster_gene(
    genome: &Genome,
    index: ClusterGeneIndex,
) -> Result<&ClusterGene, Box<dyn Error>> {
    genome
        .cluster_genes
        .get(index.0)
        .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
}

fn get_cluster_gene_mut(
    genome: &mut Genome,
    index: ClusterGeneIndex,
) -> Result<&mut ClusterGene, Box<dyn Error>> {
    genome
        .cluster_genes
        .get_mut(index.0)
        .ok_or_else(|| box MutationApplierError::IndexOutOfBounds as Box<dyn Error>)
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
