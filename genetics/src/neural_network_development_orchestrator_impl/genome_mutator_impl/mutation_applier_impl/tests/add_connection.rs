use super::*;

#[test]
fn errors_when_cluster_gene_does_not_exist() {
    let genome = empty_genome();
    let mutation = Mutation::AddConnection {
        cluster: ClusterGeneIndex(0),
        connection: connection(),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn adds_new_connection() {
    let genome = empty_genome().add_cluster_gene(cluster_gene());
    let mutation = Mutation::AddConnection {
        cluster: ClusterGeneIndex(0),
        connection: connection(),
    };
    let expected_cluster_gene = ClusterGene {
        connections: vec![connection()],
        ..cluster_gene()
    };
    let expected_genome = empty_genome().add_cluster_gene(expected_cluster_gene);
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
        ..emtpy_cluster_gene()
    }
}

fn connection() -> Connection {
    Connection {
        from: ClusterNeuronIndex(0),
        to: ClusterNeuronIndex(1),
        weight: 1.0,
    }
}
