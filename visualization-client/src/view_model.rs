#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Object {
    pub shape: Polygon,
    pub kind: Kind,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}
