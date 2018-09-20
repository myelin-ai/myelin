//! A generator for a hardcoded simulation

use crate::WorldGenerator;
use myelin_environment::object::{Kind, Location, Object, Position, Radians};
use myelin_environment::object_builder::PolygonBuilder;
use myelin_environment::simulation::{NewObject, Simulation};
use std::f64::consts::FRAC_PI_2;

/// Simulation generation algorithm that creates a fixed simulation
/// inhabited by two forests, a large central lake and
/// a row of organisms. The simulation is framed by terrain.
pub struct HardcodedGenerator {
    simulation_factory: SimulationFactory,
    object_factory: ObjectFactory,
}

pub type SimulationFactory = Box<dyn Fn() -> Box<dyn Simulation>>;
pub type ObjectFactory = Box<dyn Fn(Kind) -> Object>;

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
    pub fn new(simulation_factory: SimulationFactory, object_factory: ObjectFactory) -> Self {
        Self {
            simulation_factory,
            object_factory,
        }
    }

    fn populate_with_terrain(&self, simulation: &mut dyn Simulation) {
        simulation.add_object(self.build_terrain((25, 500), 50, 1000));
        simulation.add_object(self.build_terrain((500, 25), 1000, 50));
        simulation.add_object(self.build_terrain((975, 500), 50, 1000));
        simulation.add_object(self.build_terrain((500, 975), 1000, 50));
    }

    fn build_terrain(&self, location: (u32, u32), width: i32, length: i32) -> NewObject {
        // We add two pixels because of https://github.com/myelin-ai/myelin/issues/60
        let x_offset = width / 2 + 2;
        let y_offset = length / 2 + 2;

        NewObject {
            object: (self.object_factory)(Kind::Terrain),
            shape: PolygonBuilder::new()
                .vertex(-x_offset, -y_offset)
                .vertex(x_offset, -y_offset)
                .vertex(x_offset, y_offset)
                .vertex(-x_offset, y_offset)
                .build()
                .expect("Generated an invalid vertex"),
            position: Position {
                location: Location {
                    x: location.0,
                    y: location.1,
                },
                rotation: Radians::default(),
            },
        }
    }

    fn populate_with_water(&self, simulation: &mut dyn Simulation) {
        let object = NewObject {
            object: (self.object_factory)(Kind::Water),
            shape: PolygonBuilder::new()
                .vertex(-180, 60)
                .vertex(0, 200)
                .vertex(180, 60)
                .vertex(100, -150)
                .vertex(-100, -150)
                .build()
                .expect("Generated an invalid vertex"),
            position: Position {
                location: Location { x: 500, y: 500 },
                rotation: Radians::default(),
            },
        };
        simulation.add_object(object);
    }

    fn populate_with_plants(&self, simulation: &mut dyn Simulation) {
        for i in 0..=10 {
            for j in 0..=7 {
                simulation.add_object(self.build_plant(100 + i * 30, 100 + j * 30));
            }
        }
        for i in 0..=10 {
            for j in 0..=7 {
                simulation.add_object(self.build_plant(600 + i * 30, 100 + j * 30));
            }
        }
    }

    fn build_plant(&self, x: u32, y: u32) -> NewObject {
        NewObject {
            object: (self.object_factory)(Kind::Plant),
            shape: PolygonBuilder::new()
                .vertex(-10, -10)
                .vertex(10, -10)
                .vertex(10, 10)
                .vertex(-10, 10)
                .build()
                .expect("Generated an invalid vertex"),
            position: Position {
                location: Location { x, y },
                rotation: Radians::default(),
            },
        }
    }

    fn populate_with_organisms(&self, simulation: &mut dyn Simulation) {
        simulation.add_object(self.build_organism(300, 800));
        simulation.add_object(self.build_organism(400, 800));
        simulation.add_object(self.build_organism(500, 800));
        simulation.add_object(self.build_organism(600, 800));
        simulation.add_object(self.build_organism(700, 800));
    }

    fn build_organism(&self, x: u32, y: u32) -> NewObject {
        NewObject {
            object: (self.object_factory)(Kind::Organism),
            shape: PolygonBuilder::new()
                .vertex(25, 0)
                .vertex(-25, 20)
                .vertex(-5, 0)
                .vertex(-25, -20)
                .build()
                .expect("Generated an invalid vertex"),
            position: Position {
                location: Location { x, y },
                rotation: Radians(FRAC_PI_2),
            },
        }
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&self) -> Box<dyn Simulation> {
        let mut simulation = (self.simulation_factory)();
        self.populate_with_terrain(&mut *simulation);
        self.populate_with_water(&mut *simulation);
        self.populate_with_plants(&mut *simulation);
        self.populate_with_organisms(&mut *simulation);
        simulation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{
        ImmovableAction, ImmovableObject, Kind, Location, Object, ObjectDescription, Position,
        Radians,
    };
    use myelin_environment::simulation::NewObject;

    #[derive(Debug, Default)]
    struct SimulationMock {
        objects: Vec<NewObject>,
    }

    impl Simulation for SimulationMock {
        fn step(&mut self) {
            panic!("step() called unexpectedly")
        }
        fn add_object(&mut self, object: NewObject) {
            self.objects.push(object)
        }
        fn objects(&self) -> Vec<ObjectDescription> {
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

    #[derive(Debug)]
    struct ObjectMock;
    impl ImmovableObject for ObjectMock {
        fn step(&mut self) -> Vec<ImmovableAction> {
            panic!("step() was called unexpectedly")
        }
        fn kind(&self) -> Kind {
            panic!("kind() was called unexpectedly")
        }
    }

    #[test]
    fn generates_simulation() {
        let simulation_factory =
            Box::new(|| -> Box<dyn Simulation> { Box::new(SimulationMock::default()) });
        let object_factory = Box::new(|_: Kind| Object::Immovable(Box::new(ObjectMock {})));
        let generator = HardcodedGenerator::new(simulation_factory, object_factory);

        let _simulation = generator.generate();
    }
}
