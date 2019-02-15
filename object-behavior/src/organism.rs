use myelin_engine::prelude::*;

#[derive(Debug, Clone)]
pub struct OrganismBehavior;

impl ObjectBehavior for OrganismBehavior {
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        world_interactor: &dyn WorldInteractor,
    ) -> Option<Action> {
        None
    }
}
