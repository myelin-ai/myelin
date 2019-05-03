//! Default implementation of [`GenomeGenerator`].

use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
use crate::genome::Genome;

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug, Default, Clone)]
pub struct GenomeGeneratorImpl;

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}
