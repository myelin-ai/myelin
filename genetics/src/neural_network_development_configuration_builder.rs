use crate::{Genome, NeuralNetworkDevelopmentConfiguration};

/// Builder for [`NeuralNetworkDevelopmentConfiguration`]
///
/// [`NeuralNetworkDevelopmentConfiguration`]: ../struct.NeuralNetworkDevelopmentConfiguration.html
#[derive(Debug)]
pub struct NeuralNetworkDevelopmentConfigurationBuilder {
    parent_genomes: Option<(Genome, Genome)>,

    input_neuron_count: Option<usize>,

    output_neuron_count: Option<usize>,
}

impl NeuralNetworkDevelopmentConfigurationBuilder {
    pub fn parent_genomes(&mut self, parent_genomes: (Genome, Genome)) -> Self {
        unimplemented!()
    }

    pub fn input_neuron_count(&mut self, input_neuron_count: usize) -> Self {
        unimplemented!()
    }

    pub fn output_neuron_count(&mut self, output_neuron_count: usize) -> Self {
        unimplemented!()
    }

    pub fn build(&self) -> Result<NeuralNetworkDevelopmentConfiguration, ()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
