use myelin_environment::object::GlobalObject;
use myelin_environment::world::World;

pub(crate) trait Simulation {
    fn step(&mut self);
}
pub(crate) trait Presenter {
    fn present_objects(&self, objects: &[GlobalObject]);
}

pub(crate) struct SimulationImpl {
    presenter: Box<Presenter>,
    world: Box<World>,
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        self.world.step();
        let objects = self.world.objects();
        self.presenter.present_objects(&objects);
    }
}

impl SimulationImpl {
    pub(crate) fn new(presenter: Box<Presenter>, world: Box<World>) -> Self {
        Self { presenter, world }
    }
}
