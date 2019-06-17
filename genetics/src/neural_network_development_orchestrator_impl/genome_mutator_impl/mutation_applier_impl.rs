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

            Mutation::AddConnection {
                cluster_gene,
                connection,
            } => self.add_connection(genome, cluster_gene, connection),

            Mutation::DisableConnection {
                hox_gene,
                connection,
            } => self.disable_connection(genome, hox_gene, connection),

            Mutation::NudgeWeight {
                cluster_gene,
                connection,
                weight_delta,
            } => self.nudge_weight(genome, cluster_gene, connection, weight_delta),

            Mutation::ChangePlacementNeuron {
                cluster_gene,
                new_placement_neuron,
            } => self.change_placement_neuron(genome, cluster_gene, new_placement_neuron),

            Mutation::AddNewCluster {
                cluster_gene,
                hox_gene,
            } => self.add_new_cluster(genome, cluster_gene, hox_gene),

            Mutation::CopyCluster {
                source_cluster_gene,
                new_hox_gene,
            } => self.copy_cluster(genome, source_cluster_gene, new_hox_gene),

            Mutation::DesyncCluster { hox_gene } => self.desync_cluster(genome, hox_gene),

            Mutation::Bridge {
                target_hox_gene,
                bridge_cluster_gene,
            } => self.bridge(genome, target_hox_gene, bridge_cluster_gene),

            Mutation::AddHoxWithExistingCluster { new_hox_gene } => {
                self.add_hox_with_existing_cluster(genome, new_hox_gene)
            }

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

        let existing_connection = get_connection_mut(cluster_gene, connection_index).unwrap();
        existing_connection.to = new_neuron_index;

        Ok(())
    }

    fn add_connection(
        &self,
        genome: &mut Genome,
        cluster_gene_index: ClusterGeneIndex,
        connection: Connection,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = get_cluster_gene_mut(genome, cluster_gene_index)?;

        cluster_gene.connections.push(connection);

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

    fn nudge_weight(
        &self,
        genome: &mut Genome,
        cluster_gene_index: ClusterGeneIndex,
        connection_index: ClusterConnectionIndex,
        weight_delta: Weight,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = get_cluster_gene_mut(genome, cluster_gene_index)?;
        let connection = get_connection_mut(cluster_gene, connection_index)?;

        connection.weight += weight_delta;

        Ok(())
    }

    fn change_placement_neuron(
        &self,
        genome: &mut Genome,
        cluster_gene_index: ClusterGeneIndex,
        new_placement_neuron_index: ClusterNeuronIndex,
    ) -> Result<(), Box<dyn Error>> {
        let cluster_gene = get_cluster_gene_mut(genome, cluster_gene_index)?;

        cluster_gene.placement_neuron = new_placement_neuron_index;

        Ok(())
    }

    fn add_new_cluster(
        &self,
        genome: &mut Genome,
        cluster_gene: ClusterGene,
        hox_gene: HoxGene,
    ) -> Result<(), Box<dyn Error>> {
        genome.cluster_genes.push(cluster_gene);
        genome.hox_genes.push(hox_gene);

        Ok(())
    }

    fn copy_cluster(
        &self,
        genome: &mut Genome,
        cluster_gene_index: ClusterGeneIndex,
        hox_gene: HoxGene,
    ) -> Result<(), Box<dyn Error>> {
        let new_cluster_gene = get_cluster_gene(genome, cluster_gene_index)?.clone();

        genome.cluster_genes.push(new_cluster_gene);
        genome.hox_genes.push(hox_gene);

        Ok(())
    }

    fn desync_cluster(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
    ) -> Result<(), Box<dyn Error>> {
        let hox_gene = get_hox_gene(genome, hox_gene_index)?;
        let cluster_gene_index = hox_gene.cluster_gene;

        let new_cluster_gene = get_cluster_gene(genome, cluster_gene_index)?.clone();

        let new_cluster_gene_index = ClusterGeneIndex(genome.cluster_genes.len());
        genome.cluster_genes.push(new_cluster_gene);

        let hox_gene = get_hox_gene_mut(genome, hox_gene_index).unwrap();
        hox_gene.cluster_gene = new_cluster_gene_index;

        Ok(())
    }

    fn bridge(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
        bridge_cluster_gene: ClusterGene,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    fn add_hox_with_existing_cluster(
        &self,
        genome: &mut Genome,
        hox_gene: HoxGene,
    ) -> Result<(), Box<dyn Error>> {
        genome.hox_genes.push(hox_gene);

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

    fn duplicate_hox(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = get_hox_gene(genome, hox_gene_index)?.clone();

        genome.hox_genes.push(gene);

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
