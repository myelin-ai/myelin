//! Implementations for deriving a [`Genome`] from two parent [`Genome`]s.

pub use self::crossover::*;
use crate::genome::Genome;
#[cfg(test)]
use mockiato::mockable;
use myelin_clone_box::clone_box;
use std::fmt::Debug;

mod crossover;

/// Trait for deriving a new [`Genome`] from two parent [`Genome`]s.
#[cfg_attr(test, mockable)]
pub trait GenomeDeriver: Debug + GenomeDeriverClone {
    /// Derives a new [`Genome`] from two parent [`Genome`]s.
    fn derive_genome_from_parents(&mut self, parent_genomes: (Genome, Genome)) -> Genome;
}

clone_box!(GenomeDeriver, GenomeDeriverClone);
