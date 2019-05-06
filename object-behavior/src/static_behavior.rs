//! Contains the [`Static`] behavior.

use myelin_engine::prelude::*;
use myelin_object_data::AdditionalObjectDescription;

/// A purely static and non-interactive behavior.
/// This type will never perform any actions.
#[derive(Debug, Default, Clone)]
pub struct Static;

impl ObjectBehavior<AdditionalObjectDescription> for Static {
    fn step(
        &mut self,
        _world_interactor: Box<dyn WorldInteractor<AdditionalObjectDescription> + '_>,
    ) -> Option<Action<AdditionalObjectDescription>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_no_actions() {
        let mut object = Static::default();
        let action = object.step(box WorldInteractorMock::new());
        assert!(action.is_none());
    }
}
