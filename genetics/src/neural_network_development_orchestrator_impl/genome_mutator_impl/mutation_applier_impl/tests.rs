use super::super::Mutation;
use super::*;

mod add_connection;
mod add_neuron;

struct MutationApplicationTestParameters<F> {
    genome: Genome,
    expected_genome: Genome,
    mutation: Mutation,
    result_test_fn: F,
}

fn test_mutation_application<F>(
    MutationApplicationTestParameters {
        mut genome,
        expected_genome,
        mutation,
        result_test_fn,
    }: MutationApplicationTestParameters<F>,
) where
    F: for<'a> FnOnce(&'a Result<(), Box<dyn Error>>) -> bool,
{
    let mutation_applier = MutationApplierImpl::new();
    let result = mutation_applier.apply_mutation(&mut genome, mutation);
    assert!(result_test_fn(&result));
    assert_eq!(genome, expected_genome);
}

fn empty_genome() -> Genome {
    Genome {
        hox_genes: Vec::new(),
        cluster_genes: Vec::new(),
    }
}

fn emtpy_cluster_gene() -> ClusterGene {
    ClusterGene {
        placement_neuron: ClusterNeuronIndex(0),
        specialization: ClusterGeneSpecialization::None,
        neurons: Vec::new(),
        connections: Vec::new(),
    }
}

impl Genome {
    fn add_cluster_gene(mut self, cluster_gene: ClusterGene) -> Self {
        self.cluster_genes.push(cluster_gene);
        self
    }

    fn add_first_cluster_gene(self) -> Self {
        self.add_cluster_gene(first_cluster_gene())
    }
}

fn first_cluster_gene() -> ClusterGene {
    ClusterGene {
        neurons: vec![Neuron {}; 2],
        placement_neuron: ClusterNeuronIndex(0),
        specialization: ClusterGeneSpecialization::None,
        connections: vec![
            Connection {
                from: ClusterNeuronIndex(0),
                to: ClusterNeuronIndex(1),
                weight: 1.0,
            },
            Connection {
                from: ClusterNeuronIndex(1),
                to: ClusterNeuronIndex(0),
                weight: 1.0,
            },
        ],
    }
}
