//! Default implementation of [`GenomeGenerator`].

use crate::genome::Genome;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug, Default, Clone)]
pub struct GenomeGeneratorImpl;

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}
