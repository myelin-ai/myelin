//! Basic linear and vector geometry for two-dimensional Euclidean geometry

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[macro_use]
extern crate serde_derive;

/// A vector
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

/// A point in space
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct ConvexPolygon {
    pub vertices: Vec<Point>,
}

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Creates a new instance of [`Radians`].
    /// Returns `None` if the given value is outside the range [0.0; 2π)
    ///
    /// ### Examples
    /// ```
    /// use myelin_environment::object::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let rotation = Radians::try_new(PI).expect("Value was outside the range [0.0; 2π)");
    /// ```
    pub fn try_new(value: f64) -> Option<Radians> {
        if value >= 0.0 && value < 2.0 * PI {
            Some(Radians { value })
        } else {
            None
        }
    }

    /// Returns the underlying value
    pub fn value(self) -> f64 {
        self.value
    }
}

/// A position within the world, defined as a combination
/// of location and rotation
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Position {
    /// An absolute location
    pub location: Point,
    /// A rotation defined in radians
    pub rotation: Radians,
}

fn to_global_rotated_vertex(
    vertex: &business_object::Vertex,
    position: &business_object::Position,
) -> view_model::Vertex {
    // See https://en.wikipedia.org/wiki/Rotation_matrix
    let center_x = f64::from(position.location.x);
    let center_y = f64::from(position.location.y);
    let rotation = position.rotation.value();
    let global_x = center_x + f64::from(vertex.x);
    let global_y = center_y + f64::from(vertex.y);
    let rotated_global_x =
        rotation.cos() * (global_x - center_x) + rotation.sin() * (global_y - center_y) + center_x;
    let rotated_global_y =
        -rotation.sin() * (global_x - center_x) + rotation.cos() * (global_y - center_y) + center_y;

    view_model::Vertex {
        x: rotated_global_x.round() as u32,
        y: rotated_global_y.round() as u32,
    }
}
