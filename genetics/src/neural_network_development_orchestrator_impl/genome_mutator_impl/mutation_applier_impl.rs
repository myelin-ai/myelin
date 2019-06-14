use super::{Mutation, MutationApplier};
use crate::genome::*;
use std::error::Error;
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
        _genome: &mut Genome,
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
            Mutation::DuplicateHox { .. } => unimplemented!(),
        }
    }
}
