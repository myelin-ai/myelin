//! A generator for a hardcoded simulation

use crate::WorldGenerator;
use myelin_environment::object::{Kind, LocalBody, LocalObject, Radians};
use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
use myelin_environment::simulation::Simulation;
use std::f64::consts::FRAC_PI_2;

/// Simulation generation algorithm that creates a fixed simulation
/// inhabited by two forests, a large central lake and
/// a row of organisms. The simulation is framed by terrain.
pub struct HardcodedGenerator {
    simulation_factory: SimulationFactory,
}

pub type SimulationFactory = Box<dyn Fn() -> Box<dyn Simulation>>;

impl HardcodedGenerator {
    /// Creates a new generator, injecting a simulation factory, i.e.
    /// a function that returns a specific [`Simulation`] that
    /// is going to be populated by the simulation generator.
    ///
    /// [`Simulation`]: ../../myelin_environment/simulation/trait.Simulation.html
    ///
    /// # Examples
    /// ```
    /// use myelin_environment::simulation::{Simulation, NphysicsSimulation};
    /// use myelin_simulationgen::SimulationGenerator;
    /// use myelin_simulationgen::generator::HardcodedGenerator;
    ///
    /// let simulation_factory = Box::new(|| -> Box<dyn Simulation> { Box::new(NphysicsSimulation::with_timestep(1.0)) });
    /// let simulationgen = HardcodedGenerator::new(simulation_factory);
    /// let generated_simulation = simulationgen.generate();
    pub fn new(simulation_factory: SimulationFactory) -> Self {
        Self { simulation_factory }
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&self) -> Box<dyn Simulation> {
        let mut simulation = (self.simulation_factory)();
        populate_with_terrain(&mut *simulation);
        populate_with_water(&mut *simulation);
        populate_with_plants(&mut *simulation);
        populate_with_organisms(&mut *simulation);
        simulation
    }
}

fn populate_with_terrain(simulation: &mut dyn Simulation) {
    simulation.add_object(build_terrain((25, 500), 50, 1000));
    simulation.add_object(build_terrain((500, 25), 1000, 50));
    simulation.add_object(build_terrain((975, 500), 50, 1000));
    simulation.add_object(build_terrain((500, 975), 1000, 50));
}

fn build_terrain(location: (u32, u32), width: i32, length: i32) -> LocalObject {
    // We add two pixels because of https://github.com/myelin-ai/myelin/issues/60
    let x_offset = width / 2 + 2;
    let y_offset = length / 2 + 2;
    let body = ObjectBuilder::new()
        .shape(
            PolygonBuilder::new()
                .vertex(-x_offset, -y_offset)
                .vertex(x_offset, -y_offset)
                .vertex(x_offset, y_offset)
                .vertex(-x_offset, y_offset)
                .build()
                .expect("Generated an invalid vertex"),
        ).location(location.0, location.1)
        .build()
        .expect("Generated an invalid object");
    LocalObject {
        body,
        behavior: unimplemented!(),
    }
}

fn populate_with_water(simulation: &mut dyn Simulation) {
    let body = ObjectBuilder::new()
        .shape(
            PolygonBuilder::new()
                .vertex(-180, 60)
                .vertex(0, 200)
                .vertex(180, 60)
                .vertex(100, -150)
                .vertex(-100, -150)
                .build()
                .expect("Generated an invalid vertex"),
        ).location(500, 500)
        .build()
        .expect("Generated an invalid object");
    let object = LocalObject {
        body,
        behavior: unimplemented!(),
    };
    simulation.add_object(object);
}

fn populate_with_plants(simulation: &mut dyn Simulation) {
    for i in 0..=10 {
        for j in 0..=7 {
            simulation.add_object(build_plant(100 + i * 30, 100 + j * 30));
        }
    }
    for i in 0..=10 {
        for j in 0..=7 {
            simulation.add_object(build_plant(600 + i * 30, 100 + j * 30));
        }
    }
}

fn build_plant(x: u32, y: u32) -> LocalObject {
    let body = ObjectBuilder::new()
        .shape(
            PolygonBuilder::new()
                .vertex(-10, -10)
                .vertex(10, -10)
                .vertex(10, 10)
                .vertex(-10, 10)
                .build()
                .expect("Generated an invalid vertex"),
        ).location(x, y)
        .build()
        .expect("Generated an invalid object");
    LocalObject {
        body,
        behavior: unimplemented!(),
    }
}

fn populate_with_organisms(simulation: &mut dyn Simulation) {
    simulation.add_object(build_organism(300, 800));
    simulation.add_object(build_organism(400, 800));
    simulation.add_object(build_organism(500, 800));
    simulation.add_object(build_organism(600, 800));
    simulation.add_object(build_organism(700, 800));
}

fn build_organism(x: u32, y: u32) -> LocalObject {
    let body = ObjectBuilder::new()
        .shape(
            PolygonBuilder::new()
                .vertex(25, 0)
                .vertex(-25, 20)
                .vertex(-5, 0)
                .vertex(-25, -20)
                .build()
                .expect("Generated an invalid vertex"),
        ).location(x, y)
        .orientation(Radians(FRAC_PI_2))
        .build()
        .expect("Generated an invalid object");
    LocalObject {
        body,
        behavior: unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{GlobalObject, LocalObject};

    #[derive(Debug, Default)]
    struct SimulationMock {
        objects: Vec<LocalObject>,
    }

    impl Simulation for SimulationMock {
        fn step(&mut self) {
            panic!("step() called unexpectedly")
        }
        fn add_object(&mut self, object: LocalObject) {
            self.objects.push(object)
        }
        fn objects(&self) -> Vec<GlobalObject> {
            panic!("objects() called unexpectedly")
        }
        fn set_simulated_timestep(&mut self, _: f64) {
            panic!("set_simulated_timestep() called unexpectedly");
        }
    }
    impl Drop for SimulationMock {
        fn drop(&mut self) {
            assert!(self.objects.len() > 0);
        }
    }

    #[test]
    fn generates_simulation() {
        let simulation_factory = || -> Box<dyn Simulation> { Box::new(SimulationMock::default()) };
        let generator = HardcodedGenerator::new(Box::new(simulation_factory));

        let _simulation = generator.generate();
    }
}
