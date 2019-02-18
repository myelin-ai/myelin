//! A generator for a hardcoded simulation

use crate::NameProvider;
use crate::WorldGenerator;
use myelin_engine::prelude::*;
use myelin_object_data::{
    AdditionalObjectDescription, AdditionalObjectDescriptionSerializer, Kind,
};
use std::f64::consts::FRAC_PI_2;
use std::fmt;

/// Simulation generation algorithm that creates a fixed simulation
/// inhabited by two forests, a large central lake and
/// a row of organisms. The simulation is framed by terrain.
pub struct HardcodedGenerator {
    simulation_factory: SimulationFactory,
    plant_factory: PlantFactory,
    organism_factory: OrganismFactory,
    terrain_factory: TerrainFactory,
    water_factory: WaterFactory,
    name_provider: Box<dyn NameProvider>,
    additional_object_description_serializer: Box<dyn AdditionalObjectDescriptionSerializer>,
}

pub type SimulationFactory = Box<dyn Fn() -> Box<dyn Simulation>>;
pub type PlantFactory = Box<dyn Fn() -> Box<dyn ObjectBehavior>>;
pub type OrganismFactory = Box<dyn Fn() -> Box<dyn ObjectBehavior>>;
pub type TerrainFactory = Box<dyn Fn() -> Box<dyn ObjectBehavior>>;
pub type WaterFactory = Box<dyn Fn() -> Box<dyn ObjectBehavior>>;

impl HardcodedGenerator {
    /// Creates a new generator, injecting a simulation factory, i.e.
    /// a function that returns a specific [`Simulation`] that
    /// is going to be populated by the simulation generator.
    ///
    /// [`Simulation`]: ../../myelin_engine/simulation/trait.Simulation.html
    ///
    /// # Examples
    /// ```
    /// use myelin_engine::prelude::*;
    /// use myelin_engine::simulation::world::{
    ///     rotation_translator::NphysicsRotationTranslatorImpl, NphysicsWorld,
    /// };
    /// use myelin_engine::simulation::SimulationBuilder;
    /// use myelin_engine::simulation::SimulationImpl;
    /// use myelin_engine::world_interactor::WorldInteractorImpl;
    /// use myelin_object_behavior::Static;
    /// use myelin_object_data::{AdditionalObjectDescriptionBincodeSerializer, Kind};
    /// use myelin_worldgen::{HardcodedGenerator, NameProviderBuilder, WorldGenerator};
    /// use std::fs::read_to_string;
    /// use std::path::Path;
    /// use std::sync::{Arc, RwLock};
    ///
    /// let simulation_factory =
    ///     Box::new(|| -> Box<dyn Simulation> { SimulationBuilder::new().build() });
    ///
    /// let plant_factory = Box::new(|| -> Box<dyn ObjectBehavior> { Box::new(Static::default()) });
    /// let organism_factory = Box::new(|| -> Box<dyn ObjectBehavior> { Box::new(Static::default()) });
    /// let terrain_factory = Box::new(|| -> Box<dyn ObjectBehavior> { Box::new(Static::default()) });
    /// let water_factory = Box::new(|| -> Box<dyn ObjectBehavior> { Box::new(Static::default()) });
    ///
    /// let mut name_provider_builder = NameProviderBuilder::default();
    ///
    /// let organism_names: Vec<String> = read_to_string(Path::new("../object-names/organisms.txt"))
    ///     .expect("Error while reading file")
    ///     .lines()
    ///     .map(String::from)
    ///     .collect();
    /// name_provider_builder.add_names(&organism_names, Kind::Organism);
    ///
    /// let name_provider = name_provider_builder.build_randomized();
    ///
    /// let additional_object_description_serializer =
    ///     Box::new(AdditionalObjectDescriptionBincodeSerializer::default());
    ///
    /// let mut worldgen = HardcodedGenerator::new(
    ///     simulation_factory,
    ///     plant_factory,
    ///     organism_factory,
    ///     terrain_factory,
    ///     water_factory,
    ///     name_provider,
    ///     additional_object_description_serializer,
    /// );
    /// let generated_simulation = worldgen.generate();
    /// ```
    pub fn new(
        simulation_factory: SimulationFactory,
        plant_factory: PlantFactory,
        organism_factory: OrganismFactory,
        terrain_factory: TerrainFactory,
        water_factory: WaterFactory,
        name_provider: Box<dyn NameProvider>,
        additional_object_description_serializer: Box<dyn AdditionalObjectDescriptionSerializer>,
    ) -> Self {
        Self {
            simulation_factory,
            plant_factory,
            organism_factory,
            terrain_factory,
            water_factory,
            name_provider,
            additional_object_description_serializer,
        }
    }

    fn populate_with_terrain(&self, simulation: &mut dyn Simulation) {
        simulation.add_object(
            self.build_terrain((25.0, 500.0), 50.0, 1000.0),
            (self.terrain_factory)(),
        );
        simulation.add_object(
            self.build_terrain((500.0, 25.0), 1000.0, 50.0),
            (self.terrain_factory)(),
        );
        simulation.add_object(
            self.build_terrain((975.0, 500.0), 50.0, 1000.0),
            (self.terrain_factory)(),
        );
        simulation.add_object(
            self.build_terrain((500.0, 975.0), 1000.0, 50.0),
            (self.terrain_factory)(),
        );
    }

    fn populate_with_water(&self, simulation: &mut dyn Simulation) {
        let object_data = AdditionalObjectDescription {
            name: None,
            kind: Kind::Water,
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
            .associated_data(
                self.additional_object_description_serializer
                    .serialize(&object_data),
            )
            .build()
            .expect("Failed to build water");

        simulation.add_object(object_description, (self.water_factory)());
    }

    fn populate_with_plants(&self, simulation: &mut dyn Simulation) {
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
                    simulation.add_object(plant, (self.plant_factory)());
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

    fn populate_with_organisms(&mut self, simulation: &mut dyn Simulation) {
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
                (self.organism_factory)(),
            );
        }
    }

    fn build_terrain(&self, location: (f64, f64), width: f64, length: f64) -> ObjectDescription {
        let object_data = AdditionalObjectDescription {
            name: None,
            kind: Kind::Terrain,
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
            .associated_data(
                self.additional_object_description_serializer
                    .serialize(&object_data),
            )
            .build()
            .expect("Failed to build terrain")
    }

    fn build_plant(&self, half_of_width_and_height: f64, x: f64, y: f64) -> ObjectDescription {
        let object_data = AdditionalObjectDescription {
            name: None,
            kind: Kind::Plant,
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
            .associated_data(
                self.additional_object_description_serializer
                    .serialize(&object_data),
            )
            .build()
            .expect("Failed to build plant")
    }

    fn build_organism(&self, x: f64, y: f64, name: Option<String>) -> ObjectDescription {
        let object_data = AdditionalObjectDescription {
            name,
            kind: Kind::Organism,
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
            .associated_data(
                self.additional_object_description_serializer
                    .serialize(&object_data),
            )
            .build()
            .expect("Failed to build organism")
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&mut self) -> Box<dyn Simulation> {
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
        f.debug_struct(name_of_type!(HardcodedGenerator)).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NameProviderMock;
    use mockiato::{partial_eq, partial_eq_owned};
    use myelin_object_data::AdditionalObjectDescriptionSerializerMock;

    #[test]
    fn generates_simulation() {
        let simulation_factory = box || -> Box<dyn Simulation> {
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
                .build()
                .unwrap();
            let behavior = Box::new(ObjectBehaviorMock::new());

            let mut simulation = SimulationMock::new();
            simulation.expect_add_object_any_times_and_return((1, description, behavior));
            box simulation
        };
        let plant_factory = box || -> Box<dyn ObjectBehavior> { box ObjectBehaviorMock::new() };
        let organism_factory = box || -> Box<dyn ObjectBehavior> { box ObjectBehaviorMock::new() };
        let terrain_factory = box || -> Box<dyn ObjectBehavior> { box ObjectBehaviorMock::new() };

        let water_factory = box || -> Box<dyn ObjectBehavior> { box ObjectBehaviorMock::new() };

        let mut name_provider = box NameProviderMock::new();
        name_provider
            .expect_get_name(partial_eq(Kind::Organism))
            .returns(None)
            .times(5);

        let mut additional_object_description_serializer =
            box AdditionalObjectDescriptionSerializerMock::new();

        additional_object_description_serializer
            .expect_serialize(partial_eq_owned(AdditionalObjectDescription {
                name: None,
                kind: Kind::Terrain,
            }))
            .returns(Vec::new())
            .times(4);

        additional_object_description_serializer
            .expect_serialize(partial_eq_owned(AdditionalObjectDescription {
                name: None,
                kind: Kind::Water,
            }))
            .returns(Vec::new())
            .times(1);

        additional_object_description_serializer
            .expect_serialize(partial_eq_owned(AdditionalObjectDescription {
                name: None,
                kind: Kind::Plant,
            }))
            .returns(Vec::new())
            .times(176);

        additional_object_description_serializer
            .expect_serialize(partial_eq_owned(AdditionalObjectDescription {
                name: None,
                kind: Kind::Organism,
            }))
            .returns(Vec::new())
            .times(5);

        let mut generator = HardcodedGenerator::new(
            simulation_factory,
            plant_factory,
            organism_factory,
            terrain_factory,
            water_factory,
            name_provider,
            additional_object_description_serializer,
        );

        let _simulation = generator.generate();
    }
}
