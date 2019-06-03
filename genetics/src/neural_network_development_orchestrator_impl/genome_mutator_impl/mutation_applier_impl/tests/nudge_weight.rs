use super::*;

const WEIGHT: f64 = 1.0;
const POSITIVE_WEIGHT_DELTA: f64 = 0.5;
const NEGATIVE_WEIGHT_DELTA: f64 = -0.5;

#[test]
fn errors_if_cluster_gene_does_not_exist() {
    let genome = empty_genome();

    let mutation = mutation(POSITIVE_WEIGHT_DELTA);
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn errors_if_connection_does_not_exist() {
    let genome = empty_genome().add_cluster_gene(empty_cluster_gene());

    let mutation = mutation(POSITIVE_WEIGHT_DELTA);
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn nudges_weight_of_connection_with_positive_delta() {
    test_nudges_weight_of_connection(POSITIVE_WEIGHT_DELTA);
}

#[test]
fn nudges_weight_of_connection_with_negative_delta() {
    test_nudges_weight_of_connection(NEGATIVE_WEIGHT_DELTA);
}

fn test_nudges_weight_of_connection(weight_delta: Weight) {
    let expected_weight = WEIGHT + POSITIVE_WEIGHT_DELTA;
    let genome = empty_genome().add_cluster_gene(cluster_gene());

    let expected_cluster_gene = ClusterGene {
        connections: vec![
            Connection {
                from: ClusterNeuronIndex(0),
                to: ClusterNeuronIndex(1),
                weight: WEIGHT,
            },
            Connection {
                from: ClusterNeuronIndex(1),
                to: ClusterNeuronIndex(2),
                weight: expected_weight,
            },
        ],
        ..cluster_gene()
    };
    let expected_genome = empty_genome().add_cluster_gene(expected_cluster_gene);

    let mutation = mutation(weight_delta);
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

fn cluster_gene() -> ClusterGene {
    ClusterGene {
        neurons: vec![Neuron {}; 3],
        connections: vec![
            Connection {
                from: ClusterNeuronIndex(0),
                to: ClusterNeuronIndex(1),
                weight: WEIGHT,
            },
            Connection {
                from: ClusterNeuronIndex(1),
                to: ClusterNeuronIndex(2),
                weight: WEIGHT,
            },
        ],
        ..empty_cluster_gene()
    }
}

fn mutation(weight_delta: Weight) -> Mutation {
    Mutation::NudgeWeight {
        cluster_gene: ClusterGeneIndex(0),
        connection: ClusterConnectionIndex(1),
        weight_delta,
    }
}
