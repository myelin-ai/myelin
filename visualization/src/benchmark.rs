use crate::controller::{Controller, ControllerImpl};
use crate::presenter::CanvasPresenter;
use crate::presenter::View;
use crate::view::constant::SIMULATED_TIMESTEP;
use crate::view_model::ViewModel;
use myelin_environment::object::{Kind, ObjectBehavior};
use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
use myelin_environment::simulation_impl::world::NphysicsWorld;
use myelin_environment::{simulation_impl::SimulationImpl, Simulation};
use myelin_object::{
    organism::StaticOrganism, plant::StaticPlant, terrain::StaticTerrain, water::StaticWater,
};
use myelin_worldgen::generator::HardcodedGenerator;

#[derive(Debug)]
struct TerminalView;

impl View for TerminalView {
    fn draw_objects(&self, _view_model: &ViewModel) {}
    fn flush(&self) {}
}

pub fn run_benchmark() {
    let view = Box::new(TerminalView);
    let presenter = Box::new(CanvasPresenter::new(view));
    let simulation_factory = Box::new(|| -> Box<dyn Simulation> {
        let rotation_translator = NphysicsRotationTranslatorImpl::default();
        let world = Box::new(NphysicsWorld::with_timestep(
            SIMULATED_TIMESTEP,
            Box::new(rotation_translator),
        ));
        Box::new(SimulationImpl::new(world))
    });
    let object_factory = Box::new(|kind: Kind| match kind {
        Kind::Plant => ObjectBehavior::Movable(Box::new(StaticPlant::new())),
        Kind::Organism => ObjectBehavior::Movable(Box::new(StaticOrganism::new())),
        Kind::Water => ObjectBehavior::Immovable(Box::new(StaticWater::new())),
        Kind::Terrain => ObjectBehavior::Immovable(Box::new(StaticTerrain::new())),
    });
    let worldgen = HardcodedGenerator::new(simulation_factory, object_factory);
    let mut controller = ControllerImpl::new(presenter, &worldgen);

    loop {
        controller.step();
    }
}
