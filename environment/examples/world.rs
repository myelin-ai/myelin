use myelin_environment::object::Kind;
use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
use myelin_environment::world::{NphysicsWorld, World};

fn main() {
    let mut world = NphysicsWorld::with_timestep(1000.0);

    world.add_object(
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(10, 0)
                    .vertex(0, 10)
                    .vertex(10, 10)
                    .build()
                    .unwrap(),
            ).location(30, 40)
            .kind(Kind::Organism)
            .build()
            .unwrap(),
    );
    println!("{:#?}", world);
}
