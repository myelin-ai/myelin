use crate::genome::Genome;
use crate::orchestrator_impl::{NeuralNetworkConfigurator, NeuralNetworkDeveloper};
use crate::NeuralNetworkDevelopmentConfiguration;

#[cfg(test)]
mod tests;

/// Bootstraps the neural network based on the genome
#[derive(Debug, Clone)]
pub struct GeneticNeuralNetworkDeveloper {
    development_configuration: NeuralNetworkDevelopmentConfiguration,
    genome: Genome,
}

impl GeneticNeuralNetworkDeveloper {
    /// Creates a new [`GeneticNeuralNetworkDeveloper`].
    ///
    /// [`GeneticNeuralNetworkDeveloper`]: ./struct.GeneticNeuralNetworkDeveloper.html
    pub fn new(
        development_configuration: NeuralNetworkDevelopmentConfiguration,
        genome: Genome,
    ) -> Self {
        Self {
            development_configuration,
            genome,
        }
    }
}

impl NeuralNetworkDeveloper for GeneticNeuralNetworkDeveloper {
    fn develop_neural_network(self: Box<Self>, _configurator: &mut dyn NeuralNetworkConfigurator) {
        unimplemented!()
    }
}
