#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub shape: Polygon,
    pub kind: Kind,
    pub name: Option<(Point, String)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
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
