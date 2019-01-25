#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub shape: Polygon,
    pub kind: Kind,
    pub name_label: Option<Label>,
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

/// A text label that can be drawn anywhere on the screen
#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    /// The text to draw
    pub text: String,

    /// The absolute location
    pub location: Point,

    /// The font color to use
    pub font_color: String,
}
