#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Object {
    pub(crate) shape: Polygon,
    pub(crate) kind: Kind,
    pub(crate) height: f64,
    pub(crate) name_label: Option<Label>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Polygon {
    pub(crate) vertices: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}

/// A text label that can be drawn anywhere on the screen
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Label {
    /// The text to draw
    pub(crate) text: String,

    /// The absolute location
    pub(crate) location: Point,

    /// The font color to use
    pub(crate) font_color: String,
}
