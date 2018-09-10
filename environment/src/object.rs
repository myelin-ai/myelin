#[derive(Debug, Eq, PartialEq)]
pub struct Object {
    pub body: Polygon,
    pub location: Location,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
