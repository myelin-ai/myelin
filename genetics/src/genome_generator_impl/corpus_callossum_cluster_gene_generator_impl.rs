use myelin_random::Random;
use super::{CorpusCallosumClusterGeneGenerator, CorpusCallosumConfig};
use crate::genome::ClusterGene;

/// Default implementation of [`IoClusterGeneGenerator`]
#[derive(Debug)]
pub struct CorpusCallosumClusterGeneGeneratorImpl {
    random: Box<dyn Random>,
}

impl CorpusCallosumClusterGeneGeneratorImpl {
    /// Creates a new [`CorbusCallossumClusterGeneGeneratorImpl`].
    pub fn new(random: Box<dyn Random>) -> Self {
        Self { random }
    }
}

impl CorpusCallosumClusterGeneGenerator for CorpusCallosumClusterGeneGeneratorImpl {
    fn generate_cluster_gene(&self, _config: &CorpusCallosumConfig) -> ClusterGene {
        unimplemented!()
    }
}
