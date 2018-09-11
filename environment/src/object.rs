#[derive(Debug, PartialEq, Clone)]
pub struct LocalObject {
    pub shape: LocalPolygon,
    pub location: Location,
    pub orientation: Radians,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalObject {
    pub shape: GlobalPolygon,
    pub orientation: Radians,
    pub velocity: Velocity,
    pub kind: Kind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalPolygon {
    pub vertices: Vec<LocalVertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalPolygon {
    pub vertices: Vec<GlobalVertex>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalVertex {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalVertex {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Radians(pub f32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    pub x: u32,
    pub y: u32,
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
