use crate::genome::*;
use crate::neural_network_development_orchestrator_impl::GenomeMutator;
use std::marker::PhantomData;
use wonderbox::autoresolvable;

/// A [`GenomeMutator`] that does nothing and just returns the passed [`Genome`].
#[derive(Debug, Default, Clone)]
pub struct GenomeMutatorStub(PhantomData<()>);

#[autoresolvable]
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
                placement_neuron: ClusterNeuronIndex(0),
                neurons: vec![Neuron::new(); 2],
                connections: vec![Connection {
                    from: ClusterNeuronIndex(0),
                    to: ClusterNeuronIndex(1),
                    weight: 1.0,
                }],
                specialization: ClusterGeneSpecialization::default(),
            }],
            hox_genes: vec![HoxGene {
                placement_target: HoxPlacement::Standalone,
                cluster_gene: ClusterGeneIndex(1),
                disabled_connections: Vec::new(),
            }],
        }
    }
}
