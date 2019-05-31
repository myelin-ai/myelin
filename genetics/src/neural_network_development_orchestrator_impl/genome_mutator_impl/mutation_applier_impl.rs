use crate::genome::*;
use super::{MutationApplier, Mutation};

#[derive(Debug)]
pub struct MutationApplierImpl {}

impl MutationApplier for MutationApplierImpl {
    fn apply_mutation(&self, genome: &mut Genome, mutation: Mutation) {
        unimplemented!()
    }
}
