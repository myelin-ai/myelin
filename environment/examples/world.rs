use myelin_environment::object::{Kind, Location, Object, Polygon, Velocity, Vertex};
use myelin_environment::world::{World, WorldImpl};

fn main() {
    let mut world = WorldImpl::new();

    world.add_object(Object {
        body: Polygon {
            vertices: vec![Vertex { x: 10, y: 30 }],
        },
        location: Location { x: 1, y: 2 },
        velocity: Velocity { x: 4, y: 5 },
        kind: Kind::Organism,
    });

    println!("{:#?}", world);
}
