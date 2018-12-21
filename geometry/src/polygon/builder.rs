//! Builder pattern for a convex [`Polygon`]

use super::Polygon;
use crate::Point;

/// [`Polygon`] factory, which can be used in order to configure
/// the properties of a new polygon.
/// Methods can be chained on it in order to configure it.
/// # Examples
/// ```
/// use myelin_geometry::PolygonBuilder;
/// let builder = PolygonBuilder::default();
/// ```
///
/// [`Polygon`]: ./struct.Polygon.html
#[derive(Default, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<Point>,
}

impl PolygonBuilder {
    /// Adds a vertex to the polygon
    /// # Examples
    /// ```
    /// use myelin_geometry::PolygonBuilder;
    /// let unfinished_builder = PolygonBuilder::default()
    ///     .vertex(-50.0, -50.0)
    ///     .vertex(50.0, -50.0)
    ///     .vertex(50.0, 50.0)
    ///     .vertex(-50.0, 50.0);
    /// ```
    pub fn vertex(mut self, x: f64, y: f64) -> Self {
        self.vertices.push(Point { x, y });
        self
    }

    /// Finishes building the [`Polygon`] with all
    /// vertices that have been configured up to this point
    /// # Errors
    /// This method will return an error if the number of configured
    /// vertices is less than three, as the resulting [`Polygon`]
    /// would not be two-dimensional.
    /// # Examples
    /// ```rust,ignore // The compiler fails for this test (TODO: Open issue)
    /// use myelin_geometry::PolygonBuilder;
    ///
    /// let square = PolygonBuilder::default()
    ///     .vertex(-50.0, -50.0)
    ///     .vertex(50.0, -50.0)
    ///     .vertex(50.0, 50.0)
    ///     .vertex(-50.0, 50.0)
    ///     .build()
    ///     .unwrap();
    /// ```
    ///
    /// [`Polygon`]: ../object/struct.Polygon.html
    pub fn build(self) -> Result<Polygon, ()> {
        Polygon::try_new(self.vertices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_builder_works() {
        let polygon = PolygonBuilder::default()
            .vertex(0.0, 0.0)
            .vertex(0.0, 1.0)
            .vertex(1.0, 0.0)
            .vertex(1.0, 1.0)
            .build()
            .unwrap();

        let expected = Polygon {
            vertices: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 1.0 },
                Point { x: 1.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
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
        assert_eq!(Err(()), PolygonBuilder::default().vertex(1.0, 1.0).build());
    }

    #[test]
    fn test_polygon_builder_panicks_for_two_vertices() {
        assert_eq!(
            Err(()),
            PolygonBuilder::default()
                .vertex(0.0, 0.0)
                .vertex(1.0, 1.0)
                .build()
        );
    }
}
