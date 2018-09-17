//! Convenient builders for [`LocalObject`] and [`LocalPolygon`]
//! # Examples
//! ```
//! use myelin_environment::object::{Kind, Radians};
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
//!     .velocity(4, 5)
//!     .orientation(Radians(FRAC_PI_2))
//!     .kind(Kind::Organism)
//!     .build()
//!     .unwrap();
//! ```
//!
//! [`LocalObject`]: ../object/struct.LocalObject.html
//! [`LocalPolygon`]: ../object/struct.LocalPolygon.html

use crate::object::{Kind, LocalObject, LocalPolygon, LocalVertex, Location, Radians, Velocity};

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
}

/// [`LocalObject`] factory, which can be used in order to configure
/// the properties of a new object.
/// Methods can be chained on it in order to configure it.
///
/// [`LocalObject`]: ../object/struct.LocalObject.html
#[derive(Default, Debug)]
pub struct ObjectBuilder {
    shape: Option<LocalPolygon>,
    velocity: Velocity,
    location: Option<Location>,
    kind: Option<Kind>,
    orientation: Option<Radians>,
}

impl ObjectBuilder {
    /// Generates the base configuration for creating a [`LocalObject`],
    /// from which configuration methods can be chained.
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// let builder = ObjectBuilder::new();
    /// ```
    ///
    /// [`LocalObject`]: ../object/struct.LocalObject.html
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
    pub fn shape(&mut self, polygon: LocalPolygon) -> &mut Self {
        self.shape = Some(polygon);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// ObjectBuilder::new()
    ///     .velocity(-12, 2);
    /// ```
    pub fn velocity(&mut self, x: i32, y: i32) -> &mut Self {
        self.velocity = Velocity { x, y };
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
    /// use myelin_environment::object::Radians;
    /// ObjectBuilder::new()
    ///     .orientation(Radians(4.5));
    /// ```
    pub fn orientation(&mut self, orientation: Radians) -> &mut Self {
        self.orientation = Some(orientation);
        self
    }

    /// Build the [`LocalObject`] with all specified settings
    /// # Errors
    /// If a non-optional member has not specified while building
    /// an error is returned, containing flags specifying which
    /// setting has been omitted
    /// # Examples
    /// ```
    /// use myelin_environment::object::{Kind, Radians};
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
    ///     .velocity(4, 5)
    ///     .orientation(Radians(FRAC_PI_2))
    ///     .kind(Kind::Organism)
    ///     .build()
    ///     .unwrap();
    /// ```
    ///
    /// [`LocalObject`]: ../object/struct.LocalObject.html
    pub fn build(&mut self) -> Result<LocalObject, ObjectBuilderError> {
        let error = ObjectBuilderError {
            missing_shape: self.shape.is_none(),
            missing_location: self.location.is_none(),
            missing_kind: self.kind.is_none(),
        };

        let object = LocalObject {
            shape: self.shape.take().ok_or_else(|| error.clone())?,
            velocity: self.velocity.clone(),
            location: self.location.take().ok_or_else(|| error.clone())?,
            kind: self.kind.take().ok_or(error)?,
            orientation: self.orientation.take().unwrap_or_else(Default::default),
        };

        Ok(object)
    }
}

/// [`LocalPolygon`] factory, which can be used in order to configure
/// the properties of a new polygon.
/// Methods can be chained on it in order to configure it.
///
/// [`LocalPolygon`]: ../object/struct.LocalPolygon.html
#[derive(Default, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<LocalVertex>,
}

impl PolygonBuilder {
    /// Generates the base configuration for creating a [`LocalPolygon`],
    /// from which configuration methods can be chained.
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::PolygonBuilder;
    /// let builder = PolygonBuilder::new();
    /// ```
    ///
    /// [`LocalPolygon`]: ../object/struct.LocalPolygon.html
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
        self.vertices.push(LocalVertex { x, y });
        self
    }

    /// Finishes building the [`LocalPolygon`] with all
    /// vertices that have been configured up to this point
    /// # Errors
    /// This method will return an error if the number of configured
    /// vertices is less than three, as the resulting [`LocalPolygon`]
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
    /// [`LocalPolygon`]: ../object/struct.LocalPolygon.html
    pub fn build(self) -> Result<LocalPolygon, ()> {
        const MINIMUM_VERTICES_IN_A_POLYGON: usize = 3;

        if self.vertices.len() < MINIMUM_VERTICES_IN_A_POLYGON {
            return Err(());
        }

        Ok(LocalPolygon {
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

        let expected = LocalPolygon {
            vertices: vec![
                LocalVertex { x: 0, y: 0 },
                LocalVertex { x: 0, y: 1 },
                LocalVertex { x: 1, y: 0 },
                LocalVertex { x: 1, y: 1 },
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
            ).velocity(10, 10)
            .location(10, 10)
            .orientation(Radians(0.0))
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
    fn test_object_builder_should_error_for_missing_shape() {
        let result = ObjectBuilder::new()
            .velocity(10, 10)
            .location(10, 10)
            .kind(Kind::Organism)
            .orientation(Radians(0.0))
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
    fn test_object_builder_should_use_default_velocity() {
        let result = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            ).location(10, 10)
            .kind(Kind::Organism)
            .orientation(Radians(0.0))
            .build();

        let expected = LocalObject {
            orientation: Radians(0.0),
            shape: LocalPolygon {
                vertices: vec![
                    LocalVertex { x: 0, y: 0 },
                    LocalVertex { x: 0, y: 1 },
                    LocalVertex { x: 1, y: 0 },
                    LocalVertex { x: 1, y: 1 },
                ],
            },
            velocity: Velocity::default(),
            location: Location { x: 10, y: 10 },
            kind: Kind::Organism,
        };

        assert_eq!(Ok(expected), result);
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
            ).velocity(10, 10)
            .kind(Kind::Organism)
            .orientation(Radians(0.0))
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
            ).velocity(10, 20)
            .location(30, 40)
            .kind(Kind::Organism)
            .build();

        let expected = LocalObject {
            orientation: Radians(0.0),
            shape: LocalPolygon {
                vertices: vec![
                    LocalVertex { x: 0, y: 0 },
                    LocalVertex { x: 0, y: 1 },
                    LocalVertex { x: 1, y: 0 },
                    LocalVertex { x: 1, y: 1 },
                ],
            },
            velocity: Velocity { x: 10, y: 20 },
            location: Location { x: 30, y: 40 },
            kind: Kind::Organism,
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
            ).velocity(10, 20)
            .location(30, 40)
            .kind(Kind::Organism)
            .orientation(Radians(1.1))
            .build();

        let expected = LocalObject {
            orientation: Radians(1.1),
            shape: LocalPolygon {
                vertices: vec![
                    LocalVertex { x: 0, y: 0 },
                    LocalVertex { x: 0, y: 1 },
                    LocalVertex { x: 1, y: 0 },
                    LocalVertex { x: 1, y: 1 },
                ],
            },
            velocity: Velocity { x: 10, y: 20 },
            location: Location { x: 30, y: 40 },
            kind: Kind::Organism,
        };

        assert_eq!(Ok(expected), result);
    }
}
