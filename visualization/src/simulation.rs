pub(crate) trait Simulation {}
pub(crate) trait Presenter {}

pub(crate) struct SimulationImpl {
    presenter: Box<Presenter>,
}

impl Simulation for SimulationImpl {}

impl SimulationImpl {
    pub(crate) fn new(presenter: Box<Presenter>) -> Self {
        Self { presenter }
    }
}
