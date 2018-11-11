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
use std::f64::consts::PI;

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
    /// use myelin_geometry::Radians;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radians_new_with_negative_0_point_1_is_none() {
        let radians = Radians::try_new(-0.1);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_new_with_0_is_some() {
        let radians = Radians::try_new(0.0);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_1_point_9_pi_is_some() {
        let radians = Radians::try_new(1.9 * PI);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_2_pi_is_none() {
        let radians = Radians::try_new(2.0 * PI);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_value_returns_1_when_given_1() {
        let value = 1.0;
        let radians = Radians::try_new(value).unwrap();
        assert_eq!(value, radians.value())
    }

}
