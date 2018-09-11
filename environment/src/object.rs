#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub shape: Polygon,
    pub orientation: Radians,
    pub location: Location,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Radians(pub f32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
