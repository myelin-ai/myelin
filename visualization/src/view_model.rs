#[derive(Debug)]
pub(crate) struct ViewModel {
    pub(crate) objects: Vec<Object>,
}

#[derive(Debug)]
pub(crate) struct Object {
    pub(crate) body: Polygon,
    pub(crate) kind: Kind,
}

#[derive(Debug)]
pub(crate) struct Polygon {
    pub(crate) vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub(crate) struct Vertex {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
