use super::*;

const VALID_PLACEMENT_NEURON: ClusterNeuronIndex = ClusterNeuronIndex(1);

#[test]
fn errors_when_cluster_gene_does_not_exist() {
    let genome = empty_genome();
    let mutation = mutation(VALID_PLACEMENT_NEURON);
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn updates_placement_neuron() {
    test_updates_placement_neuron(VALID_PLACEMENT_NEURON);
}

#[test]
fn updates_placement_neuron_even_if_neuron_does_not_exist() {
    const INVALID_PLACEMENT_NEURON: ClusterNeuronIndex = ClusterNeuronIndex(50);
    test_updates_placement_neuron(INVALID_PLACEMENT_NEURON);
}

fn test_updates_placement_neuron(new_placement_neuron: ClusterNeuronIndex) {
    let genome = empty_genome().add_cluster_gene(cluster_gene());

    let expected_cluster_gene = ClusterGene {
        placement_neuron: new_placement_neuron,
        ..cluster_gene()
    };
    let expected_genome = empty_genome().add_cluster_gene(expected_cluster_gene);

    let mutation = mutation(new_placement_neuron);
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

fn cluster_gene() -> ClusterGene {
    ClusterGene {
        neurons: vec![Neuron {}; 2],
        ..empty_cluster_gene()
    }
}

fn mutation(new_placement_neuron: ClusterNeuronIndex) -> Mutation {
    Mutation::ChangePlacementNeuron {
        cluster_gene: ClusterGeneIndex(0),
        new_placement_neuron,
    }
}
