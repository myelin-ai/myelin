//! Trait and implementations for mutating [`Genome`]s.

use crate::genome::Genome;
pub use self::variants::*;

mod variants;

/// Generates a random [`MutationVariants`].
pub trait MutationGenerator {
    /// Picks a variant of [`MutationVariants`] at random and generates it.
    fn generate_mutation(&self, genome: &Genome) -> MutationVariants;
}

/// Applies mutations to a [`Genome`].
pub trait MutationApplier {
    /// Applies the given [`MutationVariant`] to a [`Genome`].
    fn apply_mutation(&self, genome: &mut Genome, mutation: MutationVariants);
}
