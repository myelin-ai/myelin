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
        _mutation: Mutation,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}
