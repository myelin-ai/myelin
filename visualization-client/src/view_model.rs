use std::cmp::{Ordering, PartialOrd};

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub shape: Polygon,
    pub kind: Kind,
    pub name_label: Option<Label>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq)]
pub enum Kind {
    Organism,
    Plant,
    Water,
    Terrain,
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Kind) -> Option<Ordering> {
        Some(match (self, other) {
            (Kind::Organism, _) => Ordering::Greater,
            (_, Kind::Organism) => Ordering::Less,
            _ => Ordering::Equal,
        })
    }
}

/// A text label that can be drawn anywhere on the screen
#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    /// The text to draw
    pub text: String,

    /// The absolute location
    pub location: Point,

    /// The font color to use
    pub font_color: String,
}
