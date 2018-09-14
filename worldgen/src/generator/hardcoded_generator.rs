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
    let x_offset = width / 2;
    let y_offset = length / 2;
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
    world.add_object(
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-150, -100)
                    .vertex(100, -100)
                    .vertex(100, 300)
                    .vertex(-150, 300)
                    .build()
                    .expect("Generated an invalid vertex"),
            ).location(800, 100)
            .velocity(0, 0)
            .kind(Kind::Plant)
            .build()
            .expect("Generated an invalid object"),
    );
}
fn populate_with_organisms(world: &mut dyn World) {
    world.add_object(
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-100, -100)
                    .vertex(100, -100)
                    .vertex(100, 100)
                    .vertex(-100, 100)
                    .build()
                    .expect("Generated an invalid vertex"),
            ).location(700, 700)
            .orientation(Radians(FRAC_PI_2))
            .velocity(4, 5)
            .kind(Kind::Organism)
            .build()
            .expect("Generated an invalid object"),
    );
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
