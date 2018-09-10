use myelin_environment::object::Object;

pub(crate) trait Simulation {
    fn step(&mut self);
}
pub(crate) trait Presenter {
    fn present_objects(&self, objects: &[Object]);
}

pub(crate) struct SimulationImpl {
    presenter: Box<Presenter>,
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        self.presenter.present_objects(&[]);
    }
}

impl SimulationImpl {
    pub(crate) fn new(presenter: Box<Presenter>) -> Self {
        Self { presenter }
    }
}
