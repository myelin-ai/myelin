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
    /// Apply translation specified by `translation`, represented as
    /// a relative point
    pub fn translate(&self, translation: Point) -> Self {
        let translated_vertices = self
            .vertices
            .iter()
            .map(|&vertex| vertex + translation)
            .collect();

        Polygon {
            vertices: translated_vertices,
        }
    }

    /// Rotate polygon by a `rotation` around a `point`
    pub fn rotate_around_point(&self, rotation: Radians, point: Point) -> Self {
        let rotation = rotation.value();
        let rotated_vertices = self
            .vertices
            .iter()
            .map(|&vertex| {
                // See https://en.wikipedia.org/wiki/Rotation_matrix
                let delta = vertex - point;
                let (rotation_sin, rotation_cos) = rotation.sin_cos();
                let rotated_x = rotation_cos * delta.x + rotation_sin * delta.y + point.x;
                let rotated_y = -rotation_sin * delta.x + rotation_cos * delta.y + point.y;
                Point {
                    x: rotated_x,
                    y: rotated_y,
                }
            })
            .collect();
        Self {
            vertices: rotated_vertices,
        }
    }
}

#[cfg(test)]
mod test {
    use self::builder::PolygonBuilder;
    use super::*;
    use std::f64::consts::PI;

    fn polygon_and_center() -> (Polygon, Point) {
        (
            PolygonBuilder::default()
                .vertex(-10.0, -10.0)
                .vertex(10.0, -10.0)
                .vertex(10.0, 10.0)
                .vertex(-10.0, 10.0)
                .build()
                .unwrap(),
            Point { x: 0.0, y: 0.0 },
        )
    }

    fn translation() -> Point {
        Point { x: 30.0, y: 40.0 }
    }

    #[test]
    fn translates() {
        let (polygon, _) = polygon_and_center();
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point { x: 20.0, y: 30.0 },
                    Point { x: 40.0, y: 30.0 },
                    Point { x: 40.0, y: 50.0 },
                    Point { x: 20.0, y: 50.0 },
                ],
            },
            polygon.translate(translation())
        );
    }

    #[test]
    fn rotates_by_pi() {
        let (polygon, center) = polygon_and_center();

        const FLOATING_POINT_INACCURACY: f64 = 0.000000000000002;
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: 10.0 - FLOATING_POINT_INACCURACY,
                        y: 10.0 + FLOATING_POINT_INACCURACY
                    },
                    Point {
                        x: -10.0 - FLOATING_POINT_INACCURACY,
                        y: 10.0 - FLOATING_POINT_INACCURACY
                    },
                    Point {
                        x: -10.0 + FLOATING_POINT_INACCURACY,
                        y: -10.0 - FLOATING_POINT_INACCURACY
                    },
                    Point {
                        x: 10.0 + FLOATING_POINT_INACCURACY,
                        y: -10.0 + FLOATING_POINT_INACCURACY
                    },
                ],
            },
            polygon.rotate_around_point(Radians::try_new(PI).unwrap(), center)
        );
    }

    #[test]
    fn rotates_by_arbitrary_orientation() {
        let (polygon, center) = polygon_and_center();

        const ROTATION_A: f64 = 8.488724885405782;
        const ROTATION_B: f64 = 11.311125046603125;

        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: ROTATION_A,
                        y: ROTATION_B
                    },
                    Point {
                        x: -ROTATION_B,
                        y: ROTATION_A
                    },
                    Point {
                        x: -ROTATION_A,
                        y: -ROTATION_B
                    },
                    Point {
                        x: ROTATION_B,
                        y: -ROTATION_A
                    },
                ],
            },
            polygon.rotate_around_point(Radians::try_new(3.0).unwrap(), center)
        );
    }

    #[test]
    fn translates_and_rotates() {
        let (polygon, _) = polygon_and_center();
        let translation = translation();
        let translated_polygon = polygon.translate(translation);

        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: 38.48872488540578,
                        y: 51.311125046603124
                    },
                    Point {
                        x: 18.688874953396876,
                        y: 48.48872488540578
                    },
                    Point {
                        x: 21.51127511459422,
                        y: 28.688874953396876
                    },
                    Point {
                        x: 41.311125046603124,
                        y: 31.51127511459422
                    },
                ],
            },
            translated_polygon.rotate_around_point(Radians::try_new(3.0).unwrap(), translation)
        );
    }
}
