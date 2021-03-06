//! A generator for a hardcoded simulation

use crate::NameProvider;
use crate::WorldGenerator;
use myelin_engine::prelude::*;
use myelin_object_data::{AdditionalObjectDescription, Kind, ObjectDescription};
use nameof::name_of;
use std::f64::consts::FRAC_PI_2;
use std::fmt::{self, Debug, Formatter};

/// Simulation generation algorithm that creates a fixed simulation
/// inhabited by two forests, a large central lake and
/// a row of organisms. The simulation is framed by terrain.
pub struct HardcodedGenerator<'a> {
    simulation_factory: SimulationFactory<'a>,
    plant_factory: PlantFactory,
    organism_factory: OrganismFactory,
    terrain_factory: TerrainFactory,
    water_factory: WaterFactory,
    name_provider: Box<dyn NameProvider>,
}

/// A factory for creating simulations
pub struct SimulationFactory<'a>(
    pub Box<dyn Fn() -> Box<dyn Simulation<AdditionalObjectDescription> + 'a> + 'a>,
);
impl<'a> Debug for SimulationFactory<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", name_of!(type SimulationFactory<'_>))
    }
}

/// A factory for creating plants
pub struct PlantFactory(pub Box<dyn Fn() -> Box<dyn ObjectBehavior<AdditionalObjectDescription>>>);
impl Debug for PlantFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", name_of!(type PlantFactory))
    }
}

/// A factory for creating organisms
pub struct OrganismFactory(
    pub Box<dyn Fn() -> Box<dyn ObjectBehavior<AdditionalObjectDescription>>>,
);
impl Debug for OrganismFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", name_of!(type OrganismFactory))
    }
}

/// A factory for creating terrain
pub struct TerrainFactory(
    pub Box<dyn Fn() -> Box<dyn ObjectBehavior<AdditionalObjectDescription>>>,
);
impl Debug for TerrainFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", name_of!(type TerrainFactory))
    }
}

/// A factory for creating water
pub struct WaterFactory(pub Box<dyn Fn() -> Box<dyn ObjectBehavior<AdditionalObjectDescription>>>);
impl Debug for WaterFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", name_of!(type WaterFactory))
    }
}

impl<'a> HardcodedGenerator<'a> {
    /// Creates a new generator, injecting a simulation factory, i.e.
    /// a function that returns a specific [`Simulation`] that
    /// is going to be populated by the simulation generator.
    ///
    /// [`Simulation`]: ../../myelin_engine/simulation/trait.Simulation.html
    ///
    /// # Examples
    /// ```
    /// use myelin_engine::prelude::*;
    /// use myelin_engine::simulation::SimulationBuilder;
    /// use myelin_object_behavior::Static;
    /// use myelin_object_data::{AdditionalObjectDescription, Kind};
    /// use myelin_worldgen::*;
    /// use std::fs::read_to_string;
    /// use std::path::Path;
    /// use std::sync::{Arc, RwLock};
    ///
    /// let simulation_factory = SimulationFactory(Box::new(
    ///     || -> Box<dyn Simulation<AdditionalObjectDescription>> { SimulationBuilder::new().build() },
    /// ));
    ///
    /// let plant_factory = PlantFactory(Box::new(
    ///     || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> { Box::new(Static::default()) },
    /// ));
    /// let organism_factory = OrganismFactory(Box::new(
    ///     || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> { Box::new(Static::default()) },
    /// ));
    /// let terrain_factory = TerrainFactory(Box::new(
    ///     || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> { Box::new(Static::default()) },
    /// ));
    /// let water_factory = WaterFactory(Box::new(
    ///     || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> { Box::new(Static::default()) },
    /// ));
    ///
    /// let mut name_provider_builder = NameProviderBuilder::new(Box::new(|names| {
    ///     Box::new(NameProviderImpl::new(names)) as Box<dyn NameProvider>
    /// }));
    ///
    /// let organism_names: Vec<String> = read_to_string(Path::new("../object-names/organisms.txt"))
    ///     .expect("Error while reading file")
    ///     .lines()
    ///     .map(String::from)
    ///     .collect();
    /// name_provider_builder.add_names(&organism_names, Kind::Organism);
    ///
    /// let name_provider = name_provider_builder.build();
    ///
    /// let mut worldgen = HardcodedGenerator::new(
    ///     simulation_factory,
    ///     plant_factory,
    ///     organism_factory,
    ///     terrain_factory,
    ///     water_factory,
    ///     name_provider,
    /// );
    /// let generated_simulation = worldgen.generate();
    /// ```
    pub fn new(
        simulation_factory: SimulationFactory<'a>,
        plant_factory: PlantFactory,
        organism_factory: OrganismFactory,
        terrain_factory: TerrainFactory,
        water_factory: WaterFactory,
        name_provider: Box<dyn NameProvider>,
    ) -> Self {
        Self {
            simulation_factory,
            plant_factory,
            organism_factory,
            terrain_factory,
            water_factory,
            name_provider,
        }
    }

    fn populate_with_terrain(&self, simulation: &mut dyn Simulation<AdditionalObjectDescription>) {
        simulation.add_object(
            self.build_terrain((25.0, 500.0), 50.0, 1000.0),
            (self.terrain_factory.0)(),
        );
        simulation.add_object(
            self.build_terrain((500.0, 25.0), 1000.0, 50.0),
            (self.terrain_factory.0)(),
        );
        simulation.add_object(
            self.build_terrain((975.0, 500.0), 50.0, 1000.0),
            (self.terrain_factory.0)(),
        );
        simulation.add_object(
            self.build_terrain((500.0, 975.0), 1000.0, 50.0),
            (self.terrain_factory.0)(),
        );
    }

    fn populate_with_water(&self, simulation: &mut dyn Simulation<AdditionalObjectDescription>) {
        let object_data = AdditionalObjectDescription {
            name: None,
            kind: Kind::Water,
            height: 0.1,
        };

        let object_description = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-180.0, 60.0)
                    .vertex(0.0, 200.0)
                    .vertex(180.0, 60.0)
                    .vertex(100.0, -150.0)
                    .vertex(-100.0, -150.0)
                    .build()
                    .expect("Generated an invalid polygon"),
            )
            .location(500.0, 500.0)
            .mobility(Mobility::Immovable)
            .associated_data(object_data)
            .build()
            .expect("Failed to build water");

        simulation.add_object(object_description, (self.water_factory.0)());
    }

    fn populate_with_plants(&self, simulation: &mut dyn Simulation<AdditionalObjectDescription>) {
        const HALF_OF_PLANT_WIDTH_AND_HEIGHT: f64 = 10.0;
        const PADDING: f64 = 1.0;
        const DISPLACEMENT: f64 = HALF_OF_PLANT_WIDTH_AND_HEIGHT * 2.0 + PADDING;
        const NUMBER_OF_PLANT_COLUMNS: u32 = 11;
        const NUMBER_OF_PLANT_ROWS: u32 = 8;
        for i in 0..NUMBER_OF_PLANT_COLUMNS {
            for j in 0..NUMBER_OF_PLANT_ROWS {
                let left_horizontal_position = 103.0 + f64::from(i) * DISPLACEMENT;
                let right_horizontal_position = 687.0 + f64::from(i) * DISPLACEMENT;
                let vertical_position = 103.0 + f64::from(j) * DISPLACEMENT;

                let mut add_plant = |plant: ObjectDescription| {
                    simulation.add_object(plant, (self.plant_factory.0)());
                };
                add_plant(self.build_plant(
                    HALF_OF_PLANT_WIDTH_AND_HEIGHT,
                    left_horizontal_position,
                    vertical_position,
                ));
                add_plant(self.build_plant(
                    HALF_OF_PLANT_WIDTH_AND_HEIGHT,
                    right_horizontal_position,
                    vertical_position,
                ));
            }
        }
    }

    fn populate_with_organisms(
        &mut self,
        simulation: &mut dyn Simulation<AdditionalObjectDescription>,
    ) {
        let coordinates = [
            (300.0, 800.0),
            (400.0, 800.0),
            (500.0, 800.0),
            (600.0, 800.0),
            (700.0, 800.0),
        ];

        for coordinate in coordinates.iter() {
            let name = self.name_provider.get_name(Kind::Organism);

            simulation.add_object(
                self.build_organism(coordinate.0, coordinate.1, name),
                (self.organism_factory.0)(),
            );
        }
    }

    fn build_terrain(&self, location: (f64, f64), width: f64, length: f64) -> ObjectDescription {
        let object_data = AdditionalObjectDescription {
            name: None,
            kind: Kind::Terrain,
            height: 10.0,
        };

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
                    .expect("Generated an invalid polygon"),
            )
            .location(location.0, location.1)
            .mobility(Mobility::Immovable)
            .associated_data(object_data)
            .build()
            .expect("Failed to build terrain")
    }

    fn build_plant(&self, half_of_width_and_height: f64, x: f64, y: f64) -> ObjectDescription {
        let object_data = AdditionalObjectDescription {
            name: None,
            kind: Kind::Plant,
            height: 0.5,
        };

        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-half_of_width_and_height, -half_of_width_and_height)
                    .vertex(half_of_width_and_height, -half_of_width_and_height)
                    .vertex(half_of_width_and_height, half_of_width_and_height)
                    .vertex(-half_of_width_and_height, half_of_width_and_height)
                    .build()
                    .expect("Generated an invalid polygon"),
            )
            .location(x, y)
            .mobility(Mobility::Immovable)
            .passable(true)
            .associated_data(object_data)
            .build()
            .expect("Failed to build plant")
    }

    fn build_organism(&self, x: f64, y: f64, name: Option<String>) -> ObjectDescription {
        let object_data = AdditionalObjectDescription {
            name,
            kind: Kind::Organism,
            height: 1.0,
        };

        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(25.0, 0.0)
                    .vertex(-25.0, 20.0)
                    .vertex(-30.0, 0.0)
                    .vertex(-25.0, -20.0)
                    .build()
                    .expect("Generated an invalid polygon"),
            )
            .location(x, y)
            .rotation(Radians::try_new(FRAC_PI_2).unwrap())
            .mobility(Mobility::Movable(Vector::default()))
            .associated_data(object_data)
            .build()
            .expect("Failed to build organism")
    }
}

impl<'a> WorldGenerator<'a> for HardcodedGenerator<'a> {
    fn generate(&mut self) -> Box<dyn Simulation<AdditionalObjectDescription> + 'a> {
        let mut simulation = (self.simulation_factory.0)();
        self.populate_with_terrain(&mut *simulation);
        self.populate_with_water(&mut *simulation);
        self.populate_with_plants(&mut *simulation);
        self.populate_with_organisms(&mut *simulation);
        simulation
    }
}

impl<'a> fmt::Debug for HardcodedGenerator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type HardcodedGenerator<'_>))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NameProviderMock;

    #[test]
    fn generates_simulation() {
        let behavior = box ObjectBehaviorMock::new();
        let behavior_ref = behavior.as_ref();
        let simulation_factory = SimulationFactory(
            box || -> Box<dyn Simulation<AdditionalObjectDescription> + '_> {
                let description = ObjectBuilder::default()
                    .shape(
                        PolygonBuilder::default()
                            .vertex(-5.0, -5.0)
                            .vertex(5.0, -5.0)
                            .vertex(5.0, 5.0)
                            .vertex(-5.0, 5.0)
                            .build()
                            .unwrap(),
                    )
                    .location(5.0, 5.0)
                    .mobility(Mobility::Immovable)
                    .associated_data(AdditionalObjectDescription {
                        name: None,
                        kind: Kind::Organism,
                        height: 1.0,
                    })
                    .build()
                    .unwrap();
                let mut simulation = SimulationMock::new();
                simulation
                    .expect_add_object(|arg| arg.any(), |arg| arg.any())
                    .times(1..)
                    .returns(Object {
                        id: 1,
                        description,
                        behavior: behavior_ref,
                    });
                box simulation
            },
        );
        let plant_factory = PlantFactory(
            box || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
                box ObjectBehaviorMock::new()
            },
        );
        let organism_factory = OrganismFactory(
            box || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
                box ObjectBehaviorMock::new()
            },
        );
        let terrain_factory = TerrainFactory(
            box || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
                box ObjectBehaviorMock::new()
            },
        );

        let water_factory = WaterFactory(
            box || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
                box ObjectBehaviorMock::new()
            },
        );

        let mut name_provider = box NameProviderMock::new();
        name_provider
            .expect_get_name(|arg| arg.partial_eq(Kind::Organism))
            .returns(None)
            .times(5);

        let mut generator = HardcodedGenerator::new(
            simulation_factory,
            plant_factory,
            organism_factory,
            terrain_factory,
            water_factory,
            name_provider,
        );

        let _simulation = generator.generate();
    }
}
