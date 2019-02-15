use myelin_engine::prelude::*;
use myelin_neural_network::NeuralNetwork;

#[derive(Debug, Clone)]
/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
pub struct OrganismBehavior {
    neural_network: Box<dyn NeuralNetwork>,
}

impl OrganismBehavior {
    /// Create a new `OrganismBehavior` with a `neural_network`
    pub fn new(neural_network: Box<dyn NeuralNetwork>) -> Self {
        Self { neural_network }
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
}
