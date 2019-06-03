use super::*;

#[test]
fn change_target_neuron_fails_if_hox_gene_does_not_exist() {
    let mutation = Mutation::ChangeTargetNeuron {
        hox_gene: HoxGeneIndex(2),
        new_target_neuron: ClusterNeuronIndex(1),
    };

    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn change_target_neuron_fails_if_hox_gene_has_standalone_placement() {
    let mutation = Mutation::ChangeTargetNeuron {
        hox_gene: HoxGeneIndex(1),
        new_target_neuron: ClusterNeuronIndex(1),
    };

    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn change_target_neuron() {
    let mutation = Mutation::ChangeTargetNeuron {
        hox_gene: HoxGeneIndex(1),
        new_target_neuron: ClusterNeuronIndex(2),
    };

    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    let expected_genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(2)),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}
