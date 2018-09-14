use crate::WorldGenerator;
use myelin_environment::object::{Kind, LocalObject, Radians};
use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
use myelin_environment::world::World;
use std::f32::consts::FRAC_PI_2;

type WorldFactory = Box<dyn Fn() -> Box<dyn World>>;

pub struct HardcodedGenerator {
    world_factory: WorldFactory,
}

impl HardcodedGenerator {
    pub fn new(world_factory: WorldFactory) -> Self {
        Self { world_factory }
    }
}

impl WorldGenerator for HardcodedGenerator {
    fn generate(&self) -> Box<dyn World> {
        let mut world = (self.world_factory)();
        populate_with_terrain(&mut *world);
        populate_with_water(&mut *world);
        populate_with_plants(&mut *world);
        populate_with_organisms(&mut *world);
        world
    }
}

fn populate_with_terrain(world: &mut dyn World) {
    world.add_object(build_terrain((25, 500), 50, 1000));
    world.add_object(build_terrain((500, 25), 1000, 50));
    world.add_object(build_terrain((975, 500), 50, 1000));
    world.add_object(build_terrain((500, 975), 1000, 50));
}

fn build_terrain(location: (u32, u32), width: i32, length: i32) -> LocalObject {
    // We add two pixels because of https://github.com/myelin-ai/myelin/issues/60
    let x_offset = width / 2 + 2;
    let y_offset = length / 2 + 2;
    ObjectBuilder::new()
        .shape(
            PolygonBuilder::new()
                .vertex(-x_offset, -y_offset)
                .vertex(x_offset, -y_offset)
                .vertex(x_offset, y_offset)
                .vertex(-x_offset, y_offset)
                .build()
                .expect("Generated an invalid vertex"),
        ).location(location.0, location.1)
        .velocity(0, 0)
        .kind(Kind::Terrain)
        .build()
        .expect("Generated an invalid object")
}

fn populate_with_water(world: &mut dyn World) {
    world.add_object(
        ObjectBuilder::new()
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
            .velocity(0, 0)
            .kind(Kind::Water)
            .build()
            .expect("Generated an invalid object"),
    );
}

fn populate_with_plants(world: &mut dyn World) {
    for i in 0..=10 {
        for j in 0..=7 {
            world.add_object(build_plant(100 + i * 30, 100 + j * 30));
        }
    }
    for i in 0..=10 {
        for j in 0..=7 {
            world.add_object(build_plant(600 + i * 30, 100 + j * 30));
        }
    }
}

fn build_plant(x: u32, y: u32) -> LocalObject {
    ObjectBuilder::new()
        .shape(
            PolygonBuilder::new()
                .vertex(-10, -10)
                .vertex(10, -10)
                .vertex(10, 10)
                .vertex(-10, 10)
                .build()
                .expect("Generated an invalid vertex"),
        ).location(x, y)
        .velocity(0, 0)
        .kind(Kind::Plant)
        .build()
        .expect("Generated an invalid object")
}

fn populate_with_organisms(world: &mut dyn World) {
    world.add_object(build_organism(300, 800));
    world.add_object(build_organism(400, 800));
    world.add_object(build_organism(500, 800));
    world.add_object(build_organism(600, 800));
    world.add_object(build_organism(700, 800));
}

fn build_organism(x: u32, y: u32) -> LocalObject {
    ObjectBuilder::new()
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
        .velocity(0, 0)
        .kind(Kind::Organism)
        .build()
        .expect("Generated an invalid object")
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{GlobalObject, LocalObject};

    #[derive(Debug, Default)]
    struct WorldMock {
        objects: Vec<LocalObject>,
    }

    impl World for WorldMock {
        fn step(&mut self) {
            panic!("step() called unexpectedly")
        }
        fn add_object(&mut self, object: LocalObject) {
            self.objects.push(object)
        }
        fn objects(&self) -> Vec<GlobalObject> {
            panic!("objects() called unexpectedly")
        }
    }
    impl Drop for WorldMock {
        fn drop(&mut self) {
            assert!(self.objects.len() > 0);
        }
    }

    #[test]
    fn generates_world() {
        let world_factory = || -> Box<dyn World> { Box::new(WorldMock::default()) };
        let generator = HardcodedGenerator::new(Box::new(world_factory));

        let _world = generator.generate();
    }
}
