#![feature(box_syntax)]

use myelin_environment::object::{ObjectBehavior, Sensor};
use myelin_environment::simulation_impl::world::collision_filter::IgnoringCollisionFilterImpl;
use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
use myelin_environment::simulation_impl::world::NphysicsWorld;
use myelin_environment::{simulation_impl::SimulationImpl, Simulation};
use myelin_object_behavior::stochastic_spreading::{RandomChanceCheckerImpl, StochasticSpreading};
use myelin_object_behavior::Static;
use myelin_worldgen::generator::HardcodedGenerator;
use myelin_worldgen::WorldGenerator;
use std::sync::{Arc, RwLock};

fn main() {
    const SIMULATED_TIMESTEP_IN_SI_UNITS: f64 = 1.0 / 60.0;

    let plant_factory = box |sensor: Sensor| -> Box<dyn ObjectBehavior> {
        box StochasticSpreading::new(1.0 / 100.0, sensor, box RandomChanceCheckerImpl::new())
    };
    let organism_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };
    let terrain_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };
    let water_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };

    let worldgen = HardcodedGenerator::new(
        box || {
            let rotation_translator = NphysicsRotationTranslatorImpl::default();
            let force_applier = SingleTimeForceApplierImpl::default();
            let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterImpl::default()));

            let world = NphysicsWorld::with_timestep(
                SIMULATED_TIMESTEP_IN_SI_UNITS,
                box rotation_translator,
                box force_applier,
                collision_filter,
            );

            box SimulationImpl::new(box world)
        },
        plant_factory,
        organism_factory,
        terrain_factory,
        water_factory,
    );

    let mut simulation = worldgen.generate();

    simulation.step();
    // simulation.step();

     println!("{:#?}", simulation);
}
