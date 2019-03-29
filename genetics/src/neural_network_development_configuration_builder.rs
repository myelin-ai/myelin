use crate::Genome;

/// Builder for [`NeuralNetworkDevelopmentConfiguration`]
///
/// [`NeuralNetworkDevelopmentConfiguration`]: ../struct.NeuralNetworkDevelopmentConfiguration.html
#[derive(Debug)]
pub struct NeuralNetworkDevelopmentConfigurationBuilder {
    parent_genomes: Option<(Genome, Genome)>,

    input_neuron_count: Option<usize>,

    output_neuron_count: Option<usize>,
}

impl NeuralNetworkDevelopmentConfigurationBuilder {}
