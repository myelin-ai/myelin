use crate::simulation::Simulation;

pub(crate) trait InputHandler {
    fn on_timer(&mut self);
}

pub(crate) struct InputHandlerImpl {
    simulation: Box<Simulation>,
}

impl InputHandlerImpl {
    pub(crate) fn new(simulation: Box<Simulation>) -> Self {
        Self { simulation }
    }
}

impl InputHandler for InputHandlerImpl {
    fn on_timer(&mut self) {
        self.simulation.step();
    }
}
