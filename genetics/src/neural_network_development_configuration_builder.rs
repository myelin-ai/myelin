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

    pub fn build(
        &self,
    ) -> Result<
        NeuralNetworkDevelopmentConfiguration,
        NeuralNetworkDevelopmentConfigurationBuilderrError,
    > {
        unimplemented!()
    }
}

/// An error representing the values that have
/// wrongly been ommited when building finished
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct NeuralNetworkDevelopmentConfigurationBuilderrError {
    /// Flag signaling that `parent_genomes` was never called
    pub missing_parent_genomes: bool,
    /// Possible misconfigurations of `input_neuron_count`
    pub input_neuron_count: Option<ErrorState>,
    /// Possible misconfigurations of `output_neuron_count`
    pub output_neuron_count: Option<ErrorState>,
}

/// Simple representation of a misconfiguration
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorState {
    /// Value signaling that a mandatory configuration was never called
    Missing,
    /// Value signaling that a given configuration value is not valid
    InvalidValue(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_error_when_building_with_no_configuration() {}
}
