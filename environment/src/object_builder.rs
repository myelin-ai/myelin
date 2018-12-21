use crate::object::*;
use myelin_geometry::*;

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
/// use myelin_environment::object::*;
/// use myelin_geometry::*;
/// use std::f64::consts::FRAC_PI_2;
///
/// let object = ObjectBuilder::default()
///     .shape(
///         PolygonBuilder::default()
///             .vertex(-50.0, -50.0)
///             .vertex(50.0, -50.0)
///             .vertex(50.0, 50.0)
///             .vertex(-50.0, 50.0)
///             .build()
///             .unwrap(),
///     )
///     .location(300.0, 450.0)
///     .rotation(Radians::try_new(FRAC_PI_2).unwrap())
///     .kind(Kind::Organism)
///     .mobility(Mobility::Movable(Vector { x: 3.0, y: 5.0 }))
///     .build()
///     .unwrap();
/// ```
#[derive(Default, Debug)]
pub struct ObjectBuilder {
    name: Option<String>,
    shape: Option<Polygon>,
    location: Option<Point>,
    rotation: Option<Radians>,
    mobility: Option<Mobility>,
    kind: Option<Kind>,
    passable: bool,
}

impl ObjectBuilder {
    /// # Examples
    /// ```
    /// use myelin_environment::object::ObjectBuilder;
    ///
    /// ObjectBuilder::default().name(String::from("Foo"));
    /// ```
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<Option<String>>,
    {
        self.name = name.into();
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object::ObjectBuilder;
    /// use myelin_geometry::PolygonBuilder;
    ///
    /// ObjectBuilder::default().shape(
    ///     PolygonBuilder::default()
    ///         .vertex(-50.0, -50.0)
    ///         .vertex(50.0, -50.0)
    ///         .vertex(50.0, 50.0)
    ///         .vertex(-50.0, 50.0)
    ///         .build()
    ///         .unwrap(),
    /// );
    /// ```
    pub fn shape(&mut self, polygon: Polygon) -> &mut Self {
        self.shape = Some(polygon);
        self
    }

    /// # Examples
    /// ```rust,ignore // The compiler fails for this test (TODO: Open issue)
    /// use myelin_environment::object::ObjectBuilder;
    ///
    /// ObjectBuilder::default().location(3.0, 2.0);
    /// ```
    pub fn location(&mut self, x: f64, y: f64) -> &mut Self {
        self.location = Some(Point { x, y });
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object::{Kind, ObjectBuilder};
    ///
    /// ObjectBuilder::default().kind(Kind::Plant);
    /// ```
    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object::{Mobility, ObjectBuilder};
    /// use myelin_geometry::Vector;
    ///
    /// ObjectBuilder::default().mobility(Mobility::Movable(Vector { x: -12.0, y: 4.0 }));
    /// ```
    pub fn mobility(&mut self, mobility: Mobility) -> &mut Self {
        self.mobility = Some(mobility);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object::ObjectBuilder;
    /// use myelin_geometry::Radians;
    ///
    /// ObjectBuilder::default().rotation(Radians::try_new(4.5).unwrap());
    /// ```
    pub fn rotation(&mut self, rotation: Radians) -> &mut Self {
        self.rotation = Some(rotation);
        self
    }

    /// # Examples
    /// ```
    /// use myelin_environment::object::ObjectBuilder;
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
    /// use myelin_environment::object::*;
    /// use myelin_geometry::*;
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let object = ObjectBuilder::default()
    ///     .shape(
    ///         PolygonBuilder::default()
    ///             .vertex(-50.0, -50.0)
    ///             .vertex(50.0, -50.0)
    ///             .vertex(50.0, 50.0)
    ///             .vertex(-50.0, 50.0)
    ///             .build()
    ///             .unwrap(),
    ///     )
    ///     .location(300.0, 450.0)
    ///     .rotation(Radians::try_new(FRAC_PI_2).unwrap())
    ///     .kind(Kind::Organism)
    ///     .mobility(Mobility::Movable(Vector { x: 3.0, y: 5.0 }))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(&mut self) -> Result<ObjectDescription, ObjectBuilderError> {
        let error = ObjectBuilderError {
            missing_shape: self.shape.is_none(),
            missing_location: self.location.is_none(),
            missing_kind: self.kind.is_none(),
            missing_mobility: self.mobility.is_none(),
        };

        let object = ObjectDescription {
            name: self.name.clone(),
            shape: self.shape.take().ok_or_else(|| error.clone())?,
            rotation: self.rotation.take().unwrap_or_else(Default::default),
            location: self.location.take().ok_or_else(|| error.clone())?,
            kind: self.kind.take().ok_or_else(|| error.clone())?,
            mobility: self.mobility.take().ok_or_else(|| error.clone())?,
            passable: self.passable,
        };

        Ok(object)
    }
}

impl From<ObjectDescription> for ObjectBuilder {
    fn from(object_description: ObjectDescription) -> Self {
        let ObjectDescription {
            name,
            shape,
            location,
            rotation,
            mobility,
            kind,
            passable,
        } = object_description;

        ObjectBuilder {
            name,
            shape: Some(shape),
            location: Some(location),
            rotation: Some(rotation),
            mobility: Some(mobility),
            kind: Some(kind),
            passable,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_object_builder_should_error_for_missing_shape() {
        let result = ObjectBuilder::default()
            .location(10.0, 10.0)
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
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
                    .build()
                    .unwrap(),
            )
            .location(10.0, 10.0)
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
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
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
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
                    .build()
                    .unwrap(),
            )
            .rotation(Radians::try_new(0.0).unwrap())
            .location(30.0, 40.0)
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
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
                    .build()
                    .unwrap(),
            )
            .location(30.0, 40.0)
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .build();

        let expected = ObjectDescription {
            name: None,
            shape: PolygonBuilder::default()
                .vertex(0.0, 0.0)
                .vertex(0.0, 1.0)
                .vertex(1.0, 0.0)
                .vertex(1.0, 1.0)
                .build()
                .unwrap(),
            location: Point { x: 30.0, y: 40.0 },
            rotation: Radians::try_new(0.0).unwrap(),
            kind: Kind::Terrain,
            mobility: Mobility::Immovable,
            passable: false,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_object_builder_uses_passable() {
        let result = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
                    .build()
                    .unwrap(),
            )
            .rotation(Radians::try_new(0.0).unwrap())
            .location(30.0, 40.0)
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .passable(true)
            .build();

        let expected = ObjectDescription {
            name: None,
            shape: Polygon::try_new(vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 1.0 },
                Point { x: 1.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
            ])
            .unwrap(),
            location: Point { x: 30.0, y: 40.0 },
            rotation: Radians::try_new(0.0).unwrap(),
            kind: Kind::Terrain,
            mobility: Mobility::Immovable,
            passable: true,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_object_builder_uses_name() {
        let result = ObjectBuilder::default()
            .name(String::from("Foo"))
            .shape(
                PolygonBuilder::default()
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
                    .build()
                    .unwrap(),
            )
            .rotation(Radians::try_new(0.0).unwrap())
            .location(30.0, 40.0)
            .kind(Kind::Terrain)
            .mobility(Mobility::Immovable)
            .build();

        let expected = ObjectDescription {
            name: String::from("Foo").into(),
            shape: Polygon::try_new(vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 1.0 },
                Point { x: 1.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
            ])
            .unwrap(),
            location: Point { x: 30.0, y: 40.0 },
            rotation: Radians::try_new(0.0).unwrap(),
            kind: Kind::Terrain,
            mobility: Mobility::Immovable,
            passable: false,
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
                    .vertex(0.0, 0.0)
                    .vertex(0.0, 1.0)
                    .vertex(1.0, 0.0)
                    .vertex(1.0, 1.0)
                    .build()
                    .unwrap(),
            )
            .mobility(Mobility::Movable(Vector { x: -12.0, y: 5.0 }))
            .kind(Kind::Organism)
            .location(30.0, 40.0)
            .rotation(Radians::try_new(1.1).unwrap())
            .build();

        let expected = ObjectDescription {
            name: None,
            location: Point { x: 30.0, y: 40.0 },
            rotation: Radians::try_new(1.1).unwrap(),
            mobility: Mobility::Movable(Vector { x: -12.0, y: 5.0 }),
            kind: Kind::Organism,
            shape: Polygon::try_new(vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 1.0 },
                Point { x: 1.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
            ])
            .unwrap(),
            passable: false,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn can_create_object_builder_from_object_description() {
        let object_description = ObjectDescription {
            name: None,
            location: Point { x: 30.0, y: 40.0 },
            rotation: Radians::try_new(1.1).unwrap(),
            mobility: Mobility::Movable(Vector { x: -12.0, y: 5.0 }),
            kind: Kind::Organism,
            shape: Polygon::try_new(vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 1.0 },
                Point { x: 1.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
            ])
            .unwrap(),
            passable: true,
        };

        assert_eq!(
            object_description,
            ObjectBuilder::from(object_description.clone())
                .build()
                .unwrap()
        );
    }
}
