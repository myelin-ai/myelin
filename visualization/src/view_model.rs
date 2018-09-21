#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ViewModel {
    pub(crate) objects: Vec<ObjectBehavior>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ObjectBehavior {
    pub(crate) shape: Polygon,
    pub(crate) kind: Kind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Polygon {
    pub(crate) vertices: Vec<Vertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Vertex {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
