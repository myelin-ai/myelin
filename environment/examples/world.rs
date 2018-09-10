use myelin_environment::object::Kind;
use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
use myelin_environment::world::{World, WorldImpl};

fn main() {
    let mut world = WorldImpl::new();

    world.add_object(
        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0, 0)
                    .vertex(10, 0)
                    .vertex(0, 10)
                    .vertex(10, 10)
                    .build()
                    .unwrap(),
            ).location(30, 40)
            .velocity(4, 5)
            .kind(Kind::Organism)
            .build()
            .unwrap(),
    );

    println!("{:#?}", world);
}
