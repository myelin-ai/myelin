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
            Mutation::AddNeuron { .. } => unimplemented!(),
            Mutation::AddConnection { .. } => unimplemented!(),
            Mutation::DisableConnection { .. } => unimplemented!(),
            Mutation::NudgeWeight { .. } => unimplemented!(),
            Mutation::ChangePlacementNeuron { .. } => unimplemented!(),
            Mutation::AddNewCluster { .. } => unimplemented!(),
            Mutation::CopyCluster { .. } => unimplemented!(),
            Mutation::DesyncCluster { .. } => unimplemented!(),
            Mutation::Bridge { .. } => unimplemented!(),
            Mutation::AddHoxWithExistingCluster { .. } => unimplemented!(),
            Mutation::ChangeTargetNeuron { .. } => unimplemented!(),
            Mutation::DuplicateHox { hox_gene } => self.duplicate_hox(genome, hox_gene),
        }
    }
}

#[derive(Debug)]
enum MutationApplierResult {
    IndexOutOfBounds,
}

impl Error for MutationApplierResult {}

impl fmt::Display for MutationApplierResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutationApplierResult::IndexOutOfBounds => write!(f, "The given index does not exist"),
        }
    }
}

impl MutationApplierImpl {
    fn duplicate_hox(
        &self,
        genome: &mut Genome,
        hox_gene_index: HoxGeneIndex,
    ) -> Result<(), Box<dyn Error>> {
        let gene = genome
            .hox_genes
            .get(hox_gene_index.0)
            .ok_or(MutationApplierResult::IndexOutOfBounds)?
            .clone();

        genome.hox_genes.push(gene);

        Ok(())
    }
}
