use super::*;

#[test]
fn errors_when_hox_gene_does_not_exist() {
    let genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(hox_gene());

    const NON_EXISTENT_HOX_GENE: HoxGeneIndex = HoxGeneIndex(1);
    let mutation = Mutation::DesyncCluster {
        hox_gene: NON_EXISTENT_HOX_GENE,
    };

    test_errors(genome, mutation);
}

#[test]
fn errors_when_cluster_gene_referenced_by_hox_gene_does_not_exist() {
    let genome = empty_genome().add_hox_gene(hox_gene());
    let mutation = mutation();
    test_errors(genome, mutation());
}

fn test_errors(genome: Genome, mutation: Mutation) {
    let expected_genome = genome.clone();

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn copies_cluster_referenced_by_hox_gene_and_updates_hox_gene() {
    let cluster_gene = cluster_gene();
    let hox_gene = hox_gene();
    let genome = empty_genome()
        .add_cluster_gene(cluster_gene.clone())
        .add_hox_gene(hox_gene.clone());

    const COPIED_CLUSTER_GENE: ClusterGeneIndex = ClusterGeneIndex(1);
    let expected_hox_gene = HoxGene {
        cluster_gene: COPIED_CLUSTER_GENE,
        ..hox_gene
    };
    let expected_genome = empty_genome()
        .add_cluster_gene(cluster_gene.clone())
        .add_cluster_gene(cluster_gene)
        .add_hox_gene(expected_hox_gene);

    let mutation = mutation();

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

fn mutation() -> Mutation {
    Mutation::DesyncCluster {
        hox_gene: HoxGeneIndex(0),
    }
}

fn cluster_gene() -> ClusterGene {
    empty_cluster_gene()
}

fn hox_gene() -> HoxGene {
    standalone_hox_gene(ClusterGeneIndex(0))
}
