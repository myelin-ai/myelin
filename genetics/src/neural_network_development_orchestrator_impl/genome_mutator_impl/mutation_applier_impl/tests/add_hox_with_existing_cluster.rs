use super::*;

#[test]
fn adds_hox_gene_if_cluster_gene_does_not_exist() {
    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: vec![],
    };

    let expected_genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
            standalone_hox_gene(ClusterGeneIndex(5)),
        ],
        cluster_genes: vec![],
    };

    let mutation = Mutation::AddHoxWithExistingCluster {
        new_hox_gene: standalone_hox_gene(ClusterGeneIndex(5)),
    };

    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

#[test]
fn add_hox_with_existing_cluster_gene() {
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
