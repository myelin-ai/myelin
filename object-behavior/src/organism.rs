//! Behavior of an organism that can interact with its surroundings

use myelin_engine::prelude::*;
use myelin_genetics::genome::Genome;
use myelin_genetics::{
    DevelopedNeuralNetwork, NeuralNetworkDeveloperFacade, NeuralNetworkDevelopmentConfiguration,
};
use std::any::Any;

/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
#[derive(Debug, Clone)]
pub struct OrganismBehavior {
    developed_neural_network: DevelopedNeuralNetwork,
    neural_network_developer: Box<dyn NeuralNetworkDeveloperFacade>,
}

impl OrganismBehavior {
    /// Create a new `OrganismBehavior` from a pair of parent [`Genome`]s.
    /// The [`NeuralNetworkDeveloper`] is used to create this organism's [`NeuralNetwork`]
    /// and its eventual offspring.
    ///
    /// [`Genome`]: ../myelin-genetics/struct.Genome.html
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    pub fn new(
        parent_genomes: (Genome, Genome),
        neural_network_developer: Box<dyn NeuralNetworkDeveloperFacade>,
    ) -> Self {
        /// Arbitrary number
        const INPUT_NEURON_COUNT: u32 = 5;

        /// Arbitrary number
        const OUTPUT_NEURON_COUNT: u32 = 5;

        let metadata = NeuralNetworkDevelopmentConfiguration {
            parent_genomes,
            input_neuron_count: INPUT_NEURON_COUNT,
            output_neuron_count: OUTPUT_NEURON_COUNT,
        };

        Self {
            developed_neural_network: neural_network_developer.develop_neural_network(metadata),
            neural_network_developer,
        }
    }
}

impl ObjectBehavior for OrganismBehavior {
    fn step(
        &mut self,
        _own_description: &ObjectDescription,
        _world_interactor: &dyn WorldInteractor,
    ) -> Option<Action> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
