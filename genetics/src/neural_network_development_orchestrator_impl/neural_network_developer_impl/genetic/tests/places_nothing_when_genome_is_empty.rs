use super::*;

#[test]
fn places_nothing_when_genome_is_empty() {
    let genome = GenomeStubBuilder::new().build();
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    developer.develop_neural_network(&mut configurator);
}
