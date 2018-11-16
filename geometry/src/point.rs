use crate::Vector;
use std::ops::{Add, Sub};

/// A point in space
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Point {
    /// The x coordinate of the Point
    pub x: f64,
    /// The y coordinate of the Point
    pub y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self::Output) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self::Output) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<Vector> for Point {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal_to_itself() {
        let point = Point { x: -12.9, y: 45.1 };
        assert_eq!(point, point);
    }

    #[test]
    fn is_equal_to_itself_when_zero() {
        let point = Point { x: 0.0, y: 0.0 };
        assert_eq!(point, point);
    }

    #[test]
    fn is_no_equal_to_other_point() {
        let point = Point { x: 12.3, y: 89.0 };
        let different_point = Point { x: 12.4, y: 89.0 };
        assert!(point != different_point);
    }

    #[test]
    fn adds_zero_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_add = Point { x: 0.0, y: 0.0 };
        let expected_point = original_point;
        let added_point = original_point + point_to_add;
        assert_eq!(expected_point, added_point);
    }

    #[test]
    fn adds_other_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_add = Point { x: 3.0, y: 1.0 };
        let expected_point = Point { x: 15.0, y: 44.0 };
        let added_point = original_point + point_to_add;
        assert_eq!(expected_point, added_point);
    }

    #[test]
    fn adds_negative_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_add = Point { x: -10.0, y: -20.0 };
        let expected_point = Point { x: 2.0, y: 23.0 };
        let added_point = original_point + point_to_add;
        assert_eq!(expected_point, added_point);
    }

    #[test]
    fn adds_to_zero_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_add = Point { x: -12.0, y: -43.0 };
        let expected_point = Point { x: 0.0, y: 0.0 };
        let added_point = original_point + point_to_add;
        assert_eq!(expected_point, added_point);
    }

    #[test]
    fn adds_when_negative() {
        let original_point = Point { x: -12.0, y: -43.0 };
        let point_to_add = Point { x: -4.0, y: -2.0 };
        let expected_point = Point { x: -16.0, y: -45.0 };
        let added_point = original_point + point_to_add;
        assert_eq!(expected_point, added_point);
    }

    #[test]
    fn subtracts_zero_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_subtract = Point { x: 0.0, y: 0.0 };
        let expected_point = original_point;
        let substracted_point = original_point - point_to_subtract;
        assert_eq!(expected_point, substracted_point);
    }

    #[test]
    fn subtracts_other_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_subtract = Point { x: 3.0, y: 1.0 };
        let expected_point = Point { x: 9.0, y: 42.0 };
        let substracted_point = original_point - point_to_subtract;
        assert_eq!(expected_point, substracted_point);
    }

    #[test]
    fn subtracts_negative_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_subtract = Point { x: -10.0, y: -20.0 };
        let expected_point = Point { x: 22.0, y: 63.0 };
        let substracted_point = original_point - point_to_subtract;
        assert_eq!(expected_point, substracted_point);
    }

    #[test]
    fn subtracts_to_zero_point() {
        let original_point = Point { x: 12.0, y: 43.0 };
        let point_to_subtract = original_point;
        let expected_point = Point { x: 0.0, y: 0.0 };
        let point_to_subtract = original_point - point_to_subtract;
        assert_eq!(expected_point, point_to_subtract);
    }

    #[test]
    fn subtracts_when_negative() {
        let original_point = Point { x: -12.0, y: -43.0 };
        let point_to_subtract = Point { x: -4.0, y: -2.0 };
        let expected_point = Point { x: -8.0, y: -41.0 };
        let substracted_point = original_point - point_to_subtract;
        assert_eq!(expected_point, substracted_point);
    }
}
