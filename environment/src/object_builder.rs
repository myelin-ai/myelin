use crate::object::{Kind, Location, Object, Polygon, Velocity, Vertex};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ObjectBuilderError {
    pub missing_body: bool,
    pub missing_velocity: bool,
    pub missing_location: bool,
    pub missing_kind: bool,
}

#[derive(Default, Debug)]
pub struct ObjectBuilder {
    body: Option<Polygon>,
    velocity: Option<Velocity>,
    location: Option<Location>,
    kind: Option<Kind>,
}

impl ObjectBuilder {
    pub fn body(mut self, polygon: Polygon) -> Self {
        self.body = Some(polygon);
        self
    }

    pub fn velocity(mut self, x: i32, y: i32) -> Self {
        self.velocity = Some(Velocity { x, y });
        self
    }

    pub fn location(mut self, x: u32, y: u32) -> Self {
        self.location = Some(Location { x, y });
        self
    }

    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn build(self) -> Result<Object, ObjectBuilderError> {
        let Self {
            body,
            velocity,
            location,
            kind,
        } = self;

        let error = ObjectBuilderError {
            missing_body: body.is_none(),
            missing_velocity: velocity.is_none(),
            missing_location: location.is_none(),
            missing_kind: kind.is_none(),
        };

        let object = Object {
            body: body.ok_or_else(|| error.clone())?,
            velocity: velocity.ok_or_else(|| error.clone())?,
            location: location.ok_or_else(|| error.clone())?,
            kind: kind.ok_or(error)?,
        };

        Ok(object)
    }
}

#[derive(Default, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<Vertex>,
}

impl PolygonBuilder {
    pub fn vertex(mut self, x: u32, y: u32) -> Self {
        self.vertices.push(Vertex { x, y });
        self
    }

    pub fn build(self) -> Polygon {
        Polygon {
            vertices: self.vertices,
        }
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
            .build();

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
    fn test_object_builder_should_error_for_missing_kind() {
        let result = ObjectBuilder::default()
            .body(
                PolygonBuilder::default()
                    .vertex(0, 0)
                    .vertex(0, 1)
                    .vertex(1, 0)
                    .vertex(1, 1)
                    .build(),
            ).velocity(10, 10)
            .location(10, 10)
            .build();

        assert_eq!(
            Err(ObjectBuilderError {
                missing_kind: true,
                ..Default::default()
            }),
            result
        );
    }
}
