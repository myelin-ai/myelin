#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ViewModel {
    pub objects: Vec<Object>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Object {
    pub shape: Polygon,
    pub kind: Kind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
