#[derive(Debug)]
pub struct Object {
    pub body: Polygon,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
