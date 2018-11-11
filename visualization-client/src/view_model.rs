#[derive(Debug, PartialEq, Clone)]
pub struct ViewModel {
    pub objects: Vec<Object>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub shape: Polygon,
    pub kind: Kind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
