use super::*;

#[test]
<<<<<<< HEAD
fn errors_if_cluster_gene_does_not_exist() {
    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 2],
    };
    let expected_genome = genome.clone();
=======
fn errors_if_cluster_does_not_exist() {
    let mutation = Mutation::AddHoxWithExistingCluster {
        new_hox_gene: standalone_hox_gene(ClusterGeneIndex(2)),
    };

    let genome = genome();
>>>>>>> Reduce code duplication

    let mutation = Mutation::AddHoxWithExistingCluster {
        new_hox_gene: standalone_hox_gene(ClusterGeneIndex(2)),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn add_hox_with_existing_cluster_gene() {
    let mutation = Mutation::AddHoxWithExistingCluster {
        new_hox_gene: standalone_hox_gene(ClusterGeneIndex(1)),
    };

    let genome = genome();

    let expected_genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: genome.cluster_genes.clone(),
    };

    let mutation = Mutation::AddHoxWithExistingCluster {
        new_hox_gene: standalone_hox_gene(ClusterGeneIndex(1)),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

fn genome() -> Genome {
    Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 2],
    }
}
