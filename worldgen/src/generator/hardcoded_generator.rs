//! A generator for a hardcoded simulation

use crate::WorldGenerator;
use myelin_environment::object::*;
use myelin_environment::object_builder::ObjectBuilder;
use myelin_environment::Simulation;
use myelin_geometry::*;
use std::f64::consts::FRAC_PI_2;
use std::fmt;

/// Simulation generation algorithm that creates a fixed simulation
/// inhabited by two forests, a large central lake and
/// a row of organisms. The simulation is framed by terrain.
pub struct HardcodedGenerator {
    simulation_factory: SimulationFactory,
    object_factory: ObjectFactory,
}

pub type SimulationFactory = Box<dyn Fn() -> Box<dyn Simulation>>;
pub type ObjectFactory = Box<dyn Fn(Kind) -> Box<dyn ObjectBehavior>>;

impl HardcodedGenerator {
    /// Creates a new generator, injecting a simulation factory, i.e.
    /// a function that returns a specific [`Simulation`] that
    /// is going to be populated by the simulation generator.
    ///
    /// [`Simulation`]: ../../myelin_environment/simulation/trait.Simulation.html
    ///
    /// # Examples
    /// ```
    /// use myelin_environment::Simulation;
    /// use myelin_environment::simulation_impl::{
    ///     SimulationImpl, world::NphysicsWorld, world::rotation_translator::NphysicsRotationTranslatorImpl
    /// };
    /// use myelin_environment::simulation_impl::world::collision_filter::IgnoringCollisionFilterImpl;
    /// use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
    /// use myelin_environment::object::{Kind, ObjectBehavior};
    /// use myelin_worldgen::WorldGenerator;
    /// use myelin_worldgen::generator::HardcodedGenerator;
    /// use myelin_object_behavior::Static;
    /// use std::sync::{Arc, RwLock};
    ///
    /// let simulation_factory = Box::new(|| -> Box<dyn Simulation> {
    ///     let rotation_translator = NphysicsRotationTranslatorImpl::default();
    ///     let force_applier = SingleTimeForceApplierImpl::default();
    ///     let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterImpl::default()));
    ///     let world = Box::new(NphysicsWorld::with_timestep(
    ///         1.0,
    ///         Box::new(rotation_translator),
    ///         Box::new(force_applier),
    ///         collision_filter,
    ///     ));
    ///     Box::new(SimulationImpl::new(world))
    /// });
    ///
    /// let object_factory = Box::new(|_: Kind| -> Box<dyn ObjectBehavior> { Box::new(Static::default()) });
    /// let worldgen = HardcodedGenerator::new(simulation_factory, object_factory);
    /// let generated_simulation = worldgen.generate();
    pub fn new(simulation_factory: SimulationFactory, object_factory: ObjectFactory) -> Self {
        Self {
            simulation_factory,
            object_factory,
        }
    }

    fn populate_with_terrain(&self, simulation: &mut dyn Simulation) {
        simulation.add_object(
            build_terrain((25.0, 500.0), 50.0, 1000.0),
            (self.object_factory)(Kind::Terrain),
        );
        simulation.add_object(
            build_terrain((500.0, 25.0), 1000.0, 50.0),
            (self.object_factory)(Kind::Terrain),
        );
        simulation.add_object(
            build_terrain((975.0, 500.0), 50.0, 1000.0),
            (self.object_factory)(Kind::Terrain),
        );
        simulation.add_object(
            build_terrain((500.0, 975.0), 1000.0, 50.0),
            (self.object_factory)(Kind::Terrain),
        );
    }

    fn populate_with_water(&self, simulation: &mut dyn Simulation) {
        let object_description = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-180.0, 60.0)
                    .vertex(0.0, 200.0)
                    .vertex(180.0, 60.0)
                    .vertex(100.0, -150.0)
                    .vertex(-100.0, -150.0)
                    .build()
                    .expect("Generated an invalid vertex"),
            )
            .location(500.0, 500.0)
            .mobility(Mobility::Immovable)
            .kind(Kind::Water)
            .build()
            .expect("Failed to build water");

        simulation.add_object(object_description, (self.object_factory)(Kind::Water));
    }

    fn populate_with_plants(&self, simulation: &mut dyn Simulation) {
        for i in 0..=10 {
            for j in 0..=7 {
                simulation.add_object(
                    build_plant(100.0 + f64::from(i) * 30.0, 100.0 + f64::from(j) * 30.0),
                    (self.object_factory)(Kind::Plant),
                );
            }
        }
        for i in 0..=10 {
            for j in 0..=7 {
                simulation.add_object(
                    build_plant(600.0 + f64::from(i) * 30.0, 100.0 + f64::from(j) * 30.0),
                    (self.object_factory)(Kind::Plant),
                );
            }
        }
    }

    fn populate_with_organisms(&self, simulation: &mut dyn Simulation) {
        simulation.add_object(
            build_organism(300.0, 800.0),
            (self.object_factory)(Kind::Organism),
        );
        simulation.add_object(
            build_organism(400.0, 800.0),
            (self.object_factory)(Kind::Organism),
        );
        simulation.add_object(
            build_organism(500.0, 800.0),
            (self.object_factory)(Kind::Organism),
        );
        simulation.add_object(
            build_organism(600.0, 800.0),
            (self.object_factory)(Kind::Organism),
        );
        simulation.add_object(
            build_organism(700.0, 800.0),
            (self.object_factory)(Kind::Organism),
        );
    }
}
fn build_terrain(location: (f64, f64), width: f64, length: f64) -> ObjectDescription {
    let x_offset = width / 2.0;
    let y_offset = length / 2.0;
    ObjectBuilder::default()
        .shape(
            PolygonBuilder::default()
                .vertex(-x_offset, -y_offset)
                .vertex(x_offset, -y_offset)
                .vertex(x_offset, y_offset)
                .vertex(-x_offset, y_offset)
                .build()
                .expect("Generated an invalid vertex"),
        )
        .location(location.0, location.1)
        .mobility(Mobility::Immovable)
        .kind(Kind::Terrain)
        .build()
        .expect("Failed to build terrain")
}

fn build_plant(x: f64, y: f64) -> ObjectDescription {
    ObjectBuilder::default()
        .shape(
            PolygonBuilder::default()
                .vertex(-10.0, -10.0)
                .vertex(10.0, -10.0)
                .vertex(10.0, 10.0)
                .vertex(-10.0, 10.0)
                .build()
                .expect("Generated an invalid vertex"),
        )
        .location(x, y)
        .mobility(Mobility::Immovable)
        .kind(Kind::Plant)
        .sensor(Sensor {
            shape: PolygonBuilder::default()
                .vertex(-25.0, -25.0)
                .vertex(25.0, -25.0)
                .vertex(25.0, 25.0)
                .vertex(-25.0, 25.0)
                .build()
                .expect("Generated an invalid vertex"),
            location: Point::default(),
            rotation: Radians::default(),
        })
        .passable(true)
        .build()
        .expect("Failed to build plant")
}

fn build_organism(x: f64, y: f64) -> ObjectDescription {
    ObjectBuilder::default()
        .shape(
            PolygonBuilder::default()
                .vertex(25.0, 0.0)
                .vertex(-25.0, 20.0)
                .vertex(-5.0, 0.0)
                .vertex(-25.0, -20.0)
                .build()
                .expect("Generated an invalid vertex"),
        )
        .location(x, y)
        .rotation(Radians::try_new(FRAC_PI_2).unwrap())
        .mobility(Mobility::Movable(Vector::default()))
        .kind(Kind::Organism)
        .sensor(Sensor {
            shape: PolygonBuilder::default()
                .vertex(-25.0, -25.0)
                .vertex(25.0, -25.0)
                .vertex(25.0, 25.0)
                .vertex(-25.0, 25.0)
                .build()
                .expect("Generated an invalid vertex"),
            location: Point::default(),
            rotation: Radians::default(),
        })
        .build()
        .expect("Failed to build organism")
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

impl fmt::Debug for HardcodedGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HardcodedGenerator").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::Id;
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    struct SimulationMock {
        objects: Vec<(ObjectDescription, Box<dyn ObjectBehavior>)>,
    }

    impl Simulation for SimulationMock {
        fn step(&mut self) {
            panic!("step() called unexpectedly")
        }
        fn add_object(
            &mut self,
            object_description: ObjectDescription,
            object_behavior: Box<dyn ObjectBehavior>,
        ) {
            self.objects.push((object_description, object_behavior))
        }
        fn objects(&self) -> HashMap<Id, ObjectDescription> {
            panic!("objects() called unexpectedly")
        }
        fn set_simulated_timestep(&mut self, _: f64) {
            panic!("set_simulated_timestep() called unexpectedly");
        }
    }
    impl Drop for SimulationMock {
        fn drop(&mut self) {
            assert!(!self.objects.is_empty());
        }
    }

    #[derive(Debug, Clone)]
    struct ObjectBehaviorMock;
    impl ObjectBehavior for ObjectBehaviorMock {
        fn step(
            &mut self,
            _own_description: &ObjectDescription,
            _sensor_collisions: &[ObjectDescription],
        ) -> Option<Action> {
            panic!("step() was called unexpectedly")
        }
    }

    #[test]
    fn generates_simulation() {
        let simulation_factory = box || -> Box<dyn Simulation> { box SimulationMock::default() };
        let object_factory = box |_: Kind| -> Box<dyn ObjectBehavior> { box ObjectBehaviorMock {} };
        let generator = HardcodedGenerator::new(simulation_factory, object_factory);

        let _simulation = generator.generate();
    }
}
