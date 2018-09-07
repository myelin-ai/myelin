#[derive(Debug)]
pub(crate) struct ViewModel {
    pub(crate) objects: Vec<Object>,
}

#[derive(Debug)]
pub(crate) struct Object {
    pub(crate) location: Location,
    pub(crate) orientation: Orietation,
    pub(crate) shape: Shape,
    pub(crate) kind: Kind,
}

#[derive(Debug)]
pub(crate) struct Location {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Debug)]
pub(crate) struct Orietation {
    pub(crate) radians: f64,
}

#[derive(Debug)]
pub(crate) enum Shape {
    Rectangle(RectangleShape),
}

#[derive(Debug)]
pub(crate) enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}

#[derive(Debug)]
pub(crate) struct RectangleShape {
    pub(crate) width: u32,
    pub(crate) length: u32,
}
