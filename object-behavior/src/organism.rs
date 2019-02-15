use myelin_engine::prelude::*;

#[derive(Debug, Clone)]
/// An organism that can interact with its surroundings via a neural network,
/// built from a set of genes
pub struct OrganismBehavior;

impl ObjectBehavior for OrganismBehavior {
    fn step(
        &mut self,
        _own_description: &ObjectDescription,
        _world_interactor: &dyn WorldInteractor,
    ) -> Option<Action> {
        None
    }
}
