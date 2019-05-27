//! Trait and implementations for mutating [`Genome`]s.

use crate::genome::Genome;
pub use self::mutation::*;

mod mutation;

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
