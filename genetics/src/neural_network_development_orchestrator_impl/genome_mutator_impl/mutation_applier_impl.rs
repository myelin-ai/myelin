use super::{Mutation, MutationApplier};
use crate::genome::*;
use std::error::Error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct MutationApplierImpl {}

impl MutationApplierImpl {
    pub fn new() -> MutationApplierImpl {
        MutationApplierImpl {}
    }
}

impl MutationApplier for MutationApplierImpl {
    fn apply_mutation(
        &self,
        _genome: &mut Genome,
        _mutation: Mutation,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}
