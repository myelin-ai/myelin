use crate::Point;

/// An axix-aligned bounding box
///
/// ```other
/// ┼─────────────────────────────────────── y
/// │
/// │  Upper left → ┌─────────────┐
/// │               │             │
/// │               │             │
/// │               └─────────────┘ ← Lower right
/// │
/// x
/// ```
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Aabb {
    /// The coordinates of the upper left corner of the box
    pub upper_left: Point,
    /// The coordinates of the lower right corner of the box
    pub lower_right: Point,
}

impl Aabb {
    /// Creates a new [`Aabb`] from two points.
    ///
    /// # Examples
    ///
    /// ## From tuples
    /// ```
    /// use myelin_geometry::Aabb;
    ///
    /// let area = Aabb::new((10.0, 10.0), (20.0, 0.0));
    /// ```
    ///
    /// ## From points
    /// ```
    /// use myelin_geometry::{Aabb, Point};
    ///
    /// let area = Aabb::new(Point { x: 0.0, y: 10.0 }, Point { x: 20.0, y: 20.0 });
    /// ```
    ///
    /// [`Aabb`]: ./struct.Aabb.html
    pub fn new<P1, P2>(upper_left: P1, lower_right: P2) -> Self
    where
        P1: Into<Point>,
        P2: Into<Point>,
    {
        Self {
            upper_left: upper_left.into(),
            lower_right: lower_right.into(),
        }
    }
}
