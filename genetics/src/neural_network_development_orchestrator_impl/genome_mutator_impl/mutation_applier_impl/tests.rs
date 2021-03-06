use super::super::Mutation;
use super::*;

mod add_connection;
mod add_hox_with_existing_cluster;
mod add_neuron;
mod add_new_cluster;
mod bridge;
mod change_placement_neuron;
mod change_target_neuron;
mod copy_cluster;
mod desync_cluster;
mod disable_connection;
mod duplicate_hox;
mod nudge_weight;

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
    assert_eq!(expected_genome, genome);
}

fn empty_genome() -> Genome {
    Genome::default()
}

fn empty_cluster_gene() -> ClusterGene {
    ClusterGene {
        placement_neuron: ClusterNeuronIndex(0),
        specialization: ClusterGeneSpecialization::None,
        neurons: Vec::new(),
        connections: Vec::new(),
    }
}

fn standalone_hox_gene(cluster_gene: ClusterGeneIndex) -> HoxGene {
    HoxGene {
        cluster_gene,
        placement_target: HoxPlacement::Standalone,
        disabled_connections: HashSet::new(),
    }
}

fn hox_placed_on_hox_gene(
    cluster_gene: ClusterGeneIndex,
    hox_gene: HoxGeneIndex,
    target_neuron: ClusterNeuronIndex,
) -> HoxGene {
    HoxGene {
        cluster_gene,
        placement_target: HoxPlacement::HoxGene {
            hox_gene,
            target_neuron,
        },
        disabled_connections: HashSet::new(),
    }
}

fn hox_placed_on_cluster_gene(
    placement_cluster: ClusterGeneIndex,
    placement_target: ClusterGeneIndex,
    target_neuron: ClusterNeuronIndex,
) -> HoxGene {
    HoxGene {
        cluster_gene: placement_cluster,
        placement_target: HoxPlacement::ClusterGene {
            cluster_gene: placement_target,
            target_neuron,
        },
        disabled_connections: HashSet::new(),
    }
}

impl Genome {
    fn add_cluster_gene(mut self, cluster_gene: ClusterGene) -> Self {
        self.cluster_genes.push(cluster_gene);
        self
    }

    fn add_hox_gene(mut self, hox_gene: HoxGene) -> Self {
        self.hox_genes.push(hox_gene);
        self
    }
}
