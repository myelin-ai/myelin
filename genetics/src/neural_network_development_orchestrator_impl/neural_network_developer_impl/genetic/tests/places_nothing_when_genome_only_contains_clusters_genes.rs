use super::*;

#[test]
fn places_nothing_when_genome_only_contains_clusters_genes() {
    let genome = GenomeStubBuilder::new().add_cluster().build();
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    developer.develop_neural_network(&mut configurator);
}

impl GenomeStubBuilder {
    fn add_cluster(&mut self) -> &mut Self {
        self.genome.cluster_genes.push(ClusterGene {
            neurons: vec![Neuron::new(); 2],
            connections: vec![ConnectionDefinition {
                from: NeuronClusterLocalIndex(0),
                to: NeuronClusterLocalIndex(1),
                weight: 0.5,
            }],
            placement_neuron: NeuronClusterLocalIndex(1),
            specialization: ClusterGeneSpecialization::default(),
        });
        self
    }
}
