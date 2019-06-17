//! Trait and implementations for mutating [`Genome`]s.

pub use self::mutation::*;
use crate::genome::Genome;
mod mutation;
mod mutation_generator_impl;

/// Generates a random [`Mutation`].
pub trait MutationGenerator {
    /// Picks a variant of [`Mutation`] at random and generates it.
    fn generate_mutation(&self, genome: &Genome) -> Mutation;
}

/// Applies mutations to a [`Genome`].
pub trait MutationApplier {
    /// Applies the given [`Mutation`] to a [`Genome`].
    fn apply_mutation(&self, genome: &mut Genome, mutation: Mutation);
}
