//! Types relating to 2D convex polygons and their construction

use super::*;
mod builder;
pub use self::builder::*;

/// A convex polygon
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Polygon {
    /// The vertices of the polygon
    pub vertices: Vec<Point>,
}

impl Polygon {
    fn to_global_polygon(&self, position: &Position) -> Self {
        let geo_polygon = GeoPolygon::new(
            self.vertices
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
}

#[cfg(test)]
mod test {
    use self::builder::PolygonBuilder;
    use super::*;
    use std::f64::consts::PI;

    fn polygon() -> Polygon {
        PolygonBuilder::default()
            .vertex(-10.0, -10.0)
            .vertex(10.0, -10.0)
            .vertex(10.0, 10.0)
            .vertex(-10.0, 10.0)
            .build()
            .unwrap()
    }

    fn position(rotation: Radians) -> Position {
        Position {
            location: Point { x: 30.0, y: 40.0 },
            rotation,
        }
    }

    #[test]
    fn converts_to_global_object_with_no_orientation() {
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point { x: 20.0, y: 30.0 },
                    Point { x: 40.0, y: 30.0 },
                    Point { x: 40.0, y: 50.0 },
                    Point { x: 20.0, y: 50.0 },
                ],
            },
            polygon().to_global_polygon(&position(Radians::default()))
        );
    }

    #[test]
    fn converts_to_global_object_with_pi_orientation() {
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point { x: 40.0, y: 50.0 },
                    Point { x: 20.0, y: 50.0 },
                    Point { x: 20.0, y: 30.0 },
                    Point { x: 40.0, y: 30.0 },
                ],
            },
            polygon().to_global_polygon(&position(Radians::try_new(PI).unwrap()))
        );
    }

    #[test]
    fn converts_to_global_object_with_arbitrary_orientation() {
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: 41.311125046603124,
                        y: 48.48872488540578
                    },
                    Point {
                        x: 21.51127511459422,
                        y: 51.311125046603124
                    },
                    Point {
                        x: 18.688874953396876,
                        y: 31.51127511459422
                    },
                    Point {
                        x: 38.48872488540578,
                        y: 28.688874953396876
                    },
                ],
            },
            polygon().to_global_polygon(&position(Radians::try_new(3.0).unwrap()))
        );
    }
}
