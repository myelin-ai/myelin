use super::GenomeMutator;
use crate::genome::*;
use std::marker::PhantomData;

/// A [`GenomeMutator`] that does nothing and just returns the passed [`Genome`].
#[derive(Debug, Clone, Default)]
pub struct GenomeMutatorStub(PhantomData<()>);

impl GenomeMutatorStub {
    /// Creates a new [`GenomeMutatorStub`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl GenomeMutator for GenomeMutatorStub {
    fn mutate_genome(&self, genome: Genome) -> Genome {
        genome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutate_genome_does_not_mutate_genome() {
        let genome = create_genome();
        let mutator = GenomeMutatorStub::new();

        let mutated_genome = mutator.mutate_genome(genome.clone());

        assert_eq!(genome, mutated_genome);
    }

    fn create_genome() -> Genome {
        Genome {
            cluster_genes: vec![ClusterGene {
                placement_neuron: NeuronClusterLocalIndex(0),
                neurons: vec![Neuron {}, Neuron {}],
                connections: vec![Connection {
                    from: NeuronClusterLocalIndex(0),
                    to: NeuronClusterLocalIndex(1),
                    weight: 1.0,
                }],
            }],
            hox_genes: vec![HoxGene {
                placement: HoxPlacement::Standalone,
                cluster_index: ClusterGeneIndex(1),
                disabled_connections: Vec::new(),
            }],
        }
    }
}
