//! Trait and implementations for mutating [`Genome`]s.

use crate::genome::Genome;
#[cfg(test)]
use mockiato::mockable;
use myelin_clone_box::clone_box;
use std::fmt::Debug;

/// Trait for mutating a [`Genome`].
#[cfg_attr(test, mockable)]
pub trait GenomeMutator: Debug + GenomeMutatorClone {
    /// Might apply mutations to any part of the genome.
    fn mutate_genome(&self, genome: Genome) -> Genome;
}

clone_box!(GenomeMutator, GenomeMutatorClone);
