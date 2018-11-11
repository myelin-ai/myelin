//! Convenient builders for [`ObjectDescription`] and [`Polygon`]
//! # Examples
//! ```
//! use myelin_environment::object::{Kind, Radians, Velocity, Mobility};
//! use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
//! use std::f64::consts::FRAC_PI_2;
//!
//! let object = ObjectBuilder::default()
//!     .shape(
//!         PolygonBuilder::default()
//!             .vertex(-50, -50)
//!             .vertex(50, -50)
//!             .vertex(50, 50)
//!             .vertex(-50, 50)
//!             .build()
//!             .unwrap(),
//!     ).location(300, 450)
//!     .rotation(Radians::try_new(FRAC_PI_2).unwrap())
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
/// # Examples
/// ```
/// use myelin_environment::object_builder::ObjectBuilder;
/// let builder = ObjectBuilder::default();
/// ```
///
/// [`ObjectDescription`]: ../object/struct.ObjectDescription.html
#[derive(Default, Debug)]
pub struct ObjectBuilder {
    shape: Option<Polygon>,
    location: Option<Location>,
    rotation: Option<Radians>,
    mobility: Option<Mobility>,
    kind: Option<Kind>,
    sensor: Option<Sensor>,
    passable: bool,
}

impl ObjectBuilder {
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    /// ObjectBuilder::default()
    ///     .shape(
    ///         PolygonBuilder::default()
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
    /// ObjectBuilder::default()
    ///     .location(3, 2);
    /// ```
    pub fn location(&mut self, x: f64, y: f64) -> &mut Self {
        self.location = Some(Location { x, y });
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// use myelin_environment::object::Kind;
    /// ObjectBuilder::default()
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
    /// ObjectBuilder::default()
    ///     .mobility(Mobility::Movable(Velocity { x: -12, y: 4 }));
    /// ```
    pub fn mobility(&mut self, mobility: Mobility) -> &mut Self {
        self.mobility = Some(mobility);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    /// use myelin_environment::object::{Sensor, Position};
    /// ObjectBuilder::default()
    ///     .sensor( Sensor {
    ///         shape: PolygonBuilder::default()
    ///             .vertex(-50, -50)
    ///             .vertex(50, -50)
    ///             .vertex(50, 50)
    ///             .vertex(-50, 50)
    ///             .build()
    ///             .unwrap(),
    ///         position: Position::default()
    ///     });
    /// ```
    pub fn sensor(&mut self, sensor: Sensor) -> &mut Self {
        self.sensor = Some(sensor);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    /// use myelin_environment::object::Radians;
    /// ObjectBuilder::default()
    ///     .rotation(Radians::try_new(4.5).unwrap());
    /// ```
    pub fn rotation(&mut self, rotation: Radians) -> &mut Self {
        self.rotation = Some(rotation);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::ObjectBuilder;
    ///
    /// let builder = ObjectBuilder::default();
    /// ```
    pub fn passable(&mut self, passable: bool) -> &mut Self {
        self.passable = passable;
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
    /// let object = ObjectBuilder::default()
    ///     .shape(
    ///         PolygonBuilder::default()
    ///             .vertex(-50, -50)
    ///             .vertex(50, -50)
    ///             .vertex(50, 50)
    ///             .vertex(-50, 50)
    ///             .build()
    ///             .unwrap(),
    ///     ).location(300, 450)
    ///     .rotation(Radians::try_new(FRAC_PI_2).unwrap())
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
                rotation: self.rotation.take().unwrap_or_else(Default::default),
            },
            kind: self.kind.take().ok_or_else(|| error.clone())?,
            mobility: self.mobility.take().ok_or_else(|| error.clone())?,
            sensor: self.sensor.take(),
            passable: self.passable,
        };

        Ok(object)
    }
}

/// [`Polygon`] factory, which can be used in order to configure
/// the properties of a new polygon.
/// Methods can be chained on it in order to configure it.
/// # Examples
/// ```
/// use myelin_environment::object_builder::PolygonBuilder;
/// let builder = PolygonBuilder::default();
/// ```
///
/// [`Polygon`]: ../object/struct.Polygon.html
#[derive(Default, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<Vertex>,
}

impl PolygonBuilder {
    /// Adds a vertex to the polygon
    /// # Examples
    /// ```
    /// use myelin_environment::object_builder::PolygonBuilder;
    /// let unfinished_builder = PolygonBuilder::default()
    ///     .vertex(-50, -50)
    ///     .vertex(50, -50)
    ///     .vertex(50, 50)
    ///     .vertex(-50, 50);
    /// ```
    pub fn vertex(mut self, x: f64, y: f64) -> Self {
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
    /// let square = PolygonBuilder::default()
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
        let polygon = PolygonBuilder::default()
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
        assert_eq!(Err(()), PolygonBuilder::default().build());
    }

    #[test]
    fn test_polygon_builder_errors_for_one_vertex() {
        assert_eq!(Err(()), PolygonBuilder::default().vertex(1, 1).build());
    }

    #[test]
    fn test_polygon_builder_panicks_for_two_vertices() {
        assert_eq!(
            Err(()),
            PolygonBuilder::default().vertex(0, 0).vertex(1, 1).build()
        );
    }

    #[test]
    fn test_object_builder_should_error_for_missing_shape() {
        let result = ObjectBuilder::default()
            .location(10, 10)
            .rotation(Radians::try_new(0.0).unwrap())
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
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .location(10, 10)
            .rotation(Radians::try_new(0.0).unwrap())
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
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .rotation(Radians::try_new(0.0).unwrap())
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
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .rotation(Radians::try_new(0.0).unwrap())
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
    fn test_object_builder_should_use_default_rotation() {
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
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
                rotation: Radians::try_new(0.0).unwrap(),
                location: Location { x: 30, y: 40 },
            },
            kind: Kind::Terrain,
            mobility: Mobility::Immovable,
            sensor: None,
            passable: false,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_object_builder_uses_passable() {
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build()
                    .unwrap(),
            )
            .rotation(Radians::try_new(0.0).unwrap())
            .location(30, 40)
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .passable(true)
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
                rotation: Radians::try_new(0.0).unwrap(),
                location: Location { x: 30, y: 40 },
            },
            kind: Kind::Terrain,
            mobility: Mobility::Immovable,
            sensor: None,
            passable: true,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_object_builder_should_error_with_everything_missing() {
        let result = ObjectBuilder::default().build();

        assert_eq!(
            Err(ObjectBuilderError {
                missing_shape: true,
                missing_location: true,
                missing_kind: true,
                missing_mobility: true,
            }),
            result
        );
    }

    #[test]
    fn test_object_builder_should_build_object() {
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
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
            .rotation(Radians::try_new(1.1).unwrap())
            .sensor(Sensor {
                shape: PolygonBuilder::default()
                    .vertex(2, 0)
                    .vertex(-2, 0)
                    .vertex(0, 1)
                    .build()
                    .unwrap(),
                position: Position {
                    location: Location { x: 12, y: 42 },
                    rotation: Radians::try_new(1.2).unwrap(),
                },
            })
            .build();

        let expected = ObjectDescription {
            position: Position {
                location: Location { x: 30, y: 40 },
                rotation: Radians::try_new(1.1).unwrap(),
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
            sensor: Some(Sensor {
                shape: Polygon {
                    vertices: vec![
                        Vertex { x: 2, y: 0 },
                        Vertex { x: -2, y: 0 },
                        Vertex { x: 0, y: 1 },
                    ],
                },
                position: Position {
                    location: Location { x: 12, y: 42 },
                    rotation: Radians::try_new(1.2).unwrap(),
                },
            }),
            passable: false,
        };

        assert_eq!(Ok(expected), result);
    }
}
