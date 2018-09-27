//! Convenient builders for [`ObjectDescription`] and [`Polygon`]
//! # Examples
//! ```
//! use myelin_environment::object::{Kind, Radians, Velocity, Mobility};
//! use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
//! use std::f64::consts::FRAC_PI_2;
//!
//! let object = ObjectBuilder::new()
//!     .shape(
//!         PolygonBuilder::new()
//!             .vertex(-50, -50)
//!             .vertex(50, -50)
//!             .vertex(50, 50)
//!             .vertex(-50, 50)
//!             .build()
//!             .unwrap(),
//!     ).location(300, 450)
//!     .orientation(Radians(FRAC_PI_2))
//!     .kind(Kind::Organism)
//!     .mobility(Mobility::Movable(Velocity{x: 3, y: 5}))
//!     .build()
//!     .unwrap();
//! ```
//!
//! [`ObjectDescription`]: ../object/struct.ObjectDescription.html
//! [`Polygon`]: ../object/struct.Polygon.html

use crate::object::*;

/// An error representing the values that have
/// wrongly been ommited when building finished
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ObjectBuilderError {
    /// Flag signaling that .shape(...) was never called
    pub missing_shape: bool,
    /// Flag signaling that .location(...) was never called
    pub missing_location: bool,
    /// Flag signaling that .kind(...) was never called
    pub missing_kind: bool,
    /// Flag signaling that .mobility(...) was never called
    pub missing_mobility: bool,
}

/// [`ObjectDescription`] factory, which can be used in order to configure
/// the properties of a new object.
/// Methods can be chained on it in order to configure it.
///
/// [`ObjectDescription`]: ../object/struct.ObjectDescription.html
#[derive(Default, Debug)]
pub struct ObjectBuilder {
    shape: Option<Polygon>,
    location: Option<Location>,
    orientation: Option<Radians>,
    mobility: Option<Mobility>,
    kind: Option<Kind>,
}

impl ObjectBuilder {
    /// Generates the base configuration for creating a [`ObjectDescription`],
    /// from which configuration methods can be chained.
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// let builder = ObjectBuilder::new();
    /// ```
    ///
    /// [`ObjectDescription`]: ../object/struct.ObjectDescription.html
    pub fn new() -> Self {
        Default::default()
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    /// ObjectBuilder::new()
    ///     .shape(
    ///         PolygonBuilder::new()
    ///             .vertex(-50, -50)
    ///             .vertex(50, -50)
    ///             .vertex(50, 50)
    ///             .vertex(-50, 50)
    ///             .build()
    ///             .unwrap(),
    ///     );
    /// ```
    pub fn shape(&mut self, polygon: Polygon) -> &mut Self {
        self.shape = Some(polygon);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// ObjectBuilder::new()
    ///     .location(3, 2);
    /// ```
    pub fn location(&mut self, x: u32, y: u32) -> &mut Self {
        self.location = Some(Location { x, y });
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// use myelin_environment::object::Kind;
    /// ObjectBuilder::new()
    ///     .kind(Kind::Plant);
    /// ```
    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// use myelin_environment::object::{Mobility, Velocity};
    /// ObjectBuilder::new()
    ///     .mobility(Mobility::Movable(Velocity { x: -12, y: 4 }));
    /// ```
    pub fn mobility(&mut self, mobility: Mobility) -> &mut Self {
        self.mobility = Some(mobility);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// use myelin_environment::object::Radians;
    /// ObjectBuilder::new()
    ///     .orientation(Radians(4.5));
    /// ```
    pub fn orientation(&mut self, orientation: Radians) -> &mut Self {
        self.orientation = Some(orientation);
        self
    }

    /// Build the [`ObjectDescription`] with all specified settings
    /// # Errors
    /// If a non-optional member has not specified while building
    /// an error is returned, containing flags specifying which
    /// setting has been omitted
    /// # Examples
    /// ```
    /// use myelin_environment::object::{Kind, Radians, Mobility, Velocity};
    /// use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let object = ObjectBuilder::new()
    ///     .shape(
    ///         PolygonBuilder::new()
    ///             .vertex(-50, -50)
    ///             .vertex(50, -50)
    ///             .vertex(50, 50)
    ///             .vertex(-50, 50)
    ///             .build()
    ///             .unwrap(),
    ///     ).location(300, 450)
    ///     .orientation(Radians(FRAC_PI_2))
    ///     .kind(Kind::Organism)
    ///     .mobility(Mobility::Movable(Velocity{x: 3, y: 5}))
    ///     .build()
    ///     .unwrap();
    /// ```
    ///
    /// [`ObjectDescription`]: ../object/struct.ObjectDescription.html
    pub fn build(&mut self) -> Result<ObjectDescription, ObjectBuilderError> {
        let error = ObjectBuilderError {
            missing_shape: self.shape.is_none(),
            missing_location: self.location.is_none(),
            missing_kind: self.kind.is_none(),
            missing_mobility: self.mobility.is_none(),
        };

        let object = ObjectDescription {
            shape: self.shape.take().ok_or_else(|| error.clone())?,
            position: Position {
                location: self.location.take().ok_or_else(|| error.clone())?,
                rotation: self.orientation.take().unwrap_or_else(Default::default),
            },
            kind: self.kind.take().ok_or_else(|| error.clone())?,
            mobility: self.mobility.take().ok_or(error)?,
        };

        Ok(object)
    }
}

/// [`Polygon`] factory, which can be used in order to configure
/// the properties of a new polygon.
/// Methods can be chained on it in order to configure it.
///
/// [`Polygon`]: ../object/struct.Polygon.html
#[derive(Default, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<Vertex>,
}

impl PolygonBuilder {
    /// Generates the base configuration for creating a [`Polygon`],
    /// from which configuration methods can be chained.
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::PolygonBuilder;
    /// let builder = PolygonBuilder::new();
    /// ```
    ///
    /// [`Polygon`]: ../object/struct.Polygon.html
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a vertex to the polygon
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::PolygonBuilder;
    /// let unfinished_builder = PolygonBuilder::new()
    ///     .vertex(-50, -50)
    ///     .vertex(50, -50)
    ///     .vertex(50, 50)
    ///     .vertex(-50, 50);
    /// ```
    pub fn vertex(mut self, x: i32, y: i32) -> Self {
        self.vertices.push(Vertex { x, y });
        self
    }

    /// Finishes building the [`Polygon`] with all
    /// vertices that have been configured up to this point
    /// # Errors
    /// This method will return an error if the number of configured
    /// vertices is less than three, as the resulting [`Polygon`]
    /// would not be two-dimensional.
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::PolygonBuilder;
    ///
    /// let square = PolygonBuilder::new()
    ///     .vertex(-50, -50)
    ///     .vertex(50, -50)
    ///     .vertex(50, 50)
    ///     .vertex(-50, 50)
    ///     .build()
    ///     .unwrap();
    /// ```
    ///
    /// [`Polygon`]: ../object/struct.Polygon.html
    pub fn build(self) -> Result<Polygon, ()> {
        const MINIMUM_VERTICES_IN_A_POLYGON: usize = 3;

        if self.vertices.len() < MINIMUM_VERTICES_IN_A_POLYGON {
            return Err(());
        }

        Ok(Polygon {
            vertices: self.vertices,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_polygon_builder_works() {
        let polygon = PolygonBuilder::new()
            .vertex(0, 0)
            .vertex(0, 1)
            .vertex(1, 0)
            .vertex(1, 1)
            .build()
            .unwrap();

        let expected = Polygon {
            vertices: vec![
                Vertex { x: 0, y: 0 },
                Vertex { x: 0, y: 1 },
                Vertex { x: 1, y: 0 },
                Vertex { x: 1, y: 1 },
            ],
        };

        assert_eq!(expected, polygon);
    }

    #[test]
    fn test_polygon_builder_errors_for_no_vertices() {
        assert_eq!(Err(()), PolygonBuilder::new().build());
    }

    #[test]
    fn test_polygon_builder_errors_for_one_vertex() {
        assert_eq!(Err(()), PolygonBuilder::new().vertex(1, 1).build());
    }

    #[test]
    fn test_polygon_builder_panicks_for_two_vertices() {
        assert_eq!(
            Err(()),
            PolygonBuilder::new().vertex(0, 0).vertex(1, 1).build()
        );
    }

    #[test]
    fn test_object_builder_should_error_for_missing_shape() {
        let result = ObjectBuilder::new()
            .location(10, 10)
            .orientation(Radians(0.0))
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .build();

        assert_eq!(
            Err(ObjectBuilderError {
                missing_shape: true,
                ..Default::default()
            }),
            result
        );
    }

    #[test]
    fn test_object_builder_should_error_for_missing_kind() {
        let result = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .location(10, 10)
            .orientation(Radians(0.0))
            .mobility(Mobility::Immovable)
            .build();
        assert_eq!(
            Err(ObjectBuilderError {
                missing_kind: true,
                ..Default::default()
            }),
            result
        );
    }

    #[test]
    fn test_object_builder_should_error_for_missing_location() {
        let result = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .orientation(Radians(0.0))
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .build();

        assert_eq!(
            Err(ObjectBuilderError {
                missing_location: true,
                ..Default::default()
            }),
            result
        );
    }

    #[test]
    fn test_object_builder_should_error_for_missing_mobility() {
        let result = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .orientation(Radians(0.0))
            .location(30, 40)
            .kind(Kind::Plant)
            .build();

        assert_eq!(
            Err(ObjectBuilderError {
                missing_mobility: true,
                ..Default::default()
            }),
            result
        );
    }

    #[test]
    fn test_object_builder_should_use_default_orientation() {
        let result = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .location(30, 40)
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .build();

        let expected = ObjectDescription {
            shape: Polygon {
                vertices: vec![
                    Vertex { x: 0, y: 0 },
                    Vertex { x: 0, y: 1 },
                    Vertex { x: 1, y: 0 },
                    Vertex { x: 1, y: 1 },
                ],
            },
            position: Position {
                rotation: Radians(0.0),
                location: Location { x: 30, y: 40 },
            },
            kind: Kind::Terrain,
            mobility: Mobility::Immovable,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_object_builder_should_error_with_everything_missing() {
        let result = ObjectBuilder::new().build();

        assert_eq!(
            Err(ObjectBuilderError {
                missing_shape: true,
                missing_location: true,
                missing_kind: true,
                missing_mobility: true
            }),
            result
        );
    }

    #[test]
    fn test_object_builder_should_build_object() {
        let result = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .mobility(Mobility::Movable(Velocity { x: -12, y: 5 }))
            .kind(Kind::Organism)
            .location(30, 40)
            .orientation(Radians(1.1))
            .build();

        let expected = ObjectDescription {
            position: Position {
                location: Location { x: 30, y: 40 },
                rotation: Radians(1.1),
            },
            mobility: Mobility::Movable(Velocity { x: -12, y: 5 }),
            kind: Kind::Organism,
            shape: Polygon {
                vertices: vec![
                    Vertex { x: 0, y: 0 },
                    Vertex { x: 0, y: 1 },
                    Vertex { x: 1, y: 0 },
                    Vertex { x: 1, y: 1 },
                ],
            },
        };

        assert_eq!(Ok(expected), result);
    }
}
