use super::{Mutation, MutationApplier};
use crate::genome::*;
use std::error::Error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct MutationApplierImpl {}

impl MutationApplier for MutationApplierImpl {
    fn apply_mutation(
        &self,
        genome: &mut Genome,
        mutation: Mutation,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}
