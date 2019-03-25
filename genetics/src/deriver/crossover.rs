use super::GenomeDeriver;
use crate::genome::Genome;
use std::marker::PhantomData;

/// Implementation of chromosomal crossover
#[derive(Default, Debug)]
pub struct ChromosomalCrossover(PhantomData<()>);

impl ChromosomalCrossover {
    /// Creates a new instance of [`ChromosomalCrossover`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl GenomeDeriver for ChromosomalCrossover {
    fn derive_genome_from_parents(&mut self, _parent_genomes: (Genome, Genome)) -> Genome {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
