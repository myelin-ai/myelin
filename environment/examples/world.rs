use myelin_environment::object::{Kind, Location, Object, Polygon, Radians, Velocity, Vertex};
use myelin_environment::world::{World, WorldImpl};

fn main() {
    let mut world = WorldImpl::new();

    world.add_object(Object {
        orientation: Radians(0.0),
        shape: Polygon {
            vertices: vec![
                Vertex { x: 0, y: 0 },
                Vertex { x: 10, y: 0 },
                Vertex { x: 0, y: 10 },
                Vertex { x: 10, y: 10 },
            ],
        },
        location: Location { x: 30, y: 40 },
        velocity: Velocity { x: 4, y: 5 },
        kind: Kind::Organism,
    });

    println!("{:#?}", world);
}
