#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Object {
    pub location: Location,
    pub shape: Polygon,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    Organism,
    Wall,
    Plant,
    Water,
}
