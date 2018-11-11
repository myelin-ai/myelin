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

use geo::algorithm::{rotate::RotatePoint, translate::Translate};
use geo_types::{Point as GeoPoint, Polygon as GeoPolygon};

mod radians;
pub use self::radians::*;

/// A vector
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Vector {
    /// The x component of the Vector
    pub x: f64,
    /// The y component of the Vector
    pub y: f64,
}

/// A point in space
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Point {
    /// The x coordinate of the Point
    pub x: f64,
    /// The y coordinate of the Point
    pub y: f64,
}

/// A convex polygon
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Polygon {
    /// The vertices of the polygon
    pub vertices: Vec<Point>,
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

fn to_global_polygon(polygon: &Polygon, position: &Position) -> Polygon {
    let geo_polygon = GeoPolygon::new(
        polygon
            .vertices
            .iter()
            .map(|vertex| (vertex.x, vertex.y))
            .collect::<Vec<_>>()
            .into(),
        vec![],
    );
    let center = GeoPoint::new(position.location.x, position.location.y);
    let geo_polygon = geo_polygon.translate(center.x(), center.y());
    let rotation_angle_in_degrees = position.rotation.value().to_degrees();
    let geo_polygon = geo_polygon.rotate_around_point(rotation_angle_in_degrees, center);
    let vertices = geo_polygon
        .exterior
        .into_points()
        .iter()
        .map(|point| Point {
            x: point.x(),
            y: point.y(),
        })
        .collect();
    Polygon { vertices }
}
