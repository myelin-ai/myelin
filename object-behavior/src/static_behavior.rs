//! Contains the [`Static`] behavior.

use myelin_engine::prelude::*;
use std::any::Any;

/// A purely static and non-interactive behavior.
/// This type will never perform any actions.
#[derive(Debug, Default, Clone)]
pub struct Static;

impl ObjectBehavior for Static {
    fn step(&mut self, _world_interactor: &dyn WorldInteractor) -> Option<Action> {
        None
    }

    fn as_any(&self) -> &'_ dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_no_actions() {
        let mut object = Static::default();
        let action = object.step(&WorldInteractorMock::new());
        assert!(action.is_none());
    }
}
