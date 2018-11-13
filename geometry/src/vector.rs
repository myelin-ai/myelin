use std::ops::{Add, Sub};

/// A vector
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Vector {
    /// The x component of the Vector
    pub x: f64,
    /// The y component of the Vector
    pub y: f64,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self::Output) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Self::Output) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal_to_itself() {
        let vector = Vector { x: -12.9, y: 45.1 };
        assert_eq!(vector, vector);
    }

    #[test]
    fn is_equal_to_itself_when_zero() {
        let vector = Vector { x: 0.0, y: 0.0 };
        assert_eq!(vector, vector);
    }

    #[test]
    fn is_no_equal_to_other_vector() {
        let vector = Vector { x: 12.3, y: 89.0 };
        let different_vector = Vector { x: 12.4, y: 89.0 };
        assert!(vector != different_vector);
    }

    #[test]
    fn adds_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let additional_vector = Vector { x: 0.0, y: 0.0 };
        let expected_vector = original_vector;
        let added_vector = original_vector + additional_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_other_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let additional_vector = Vector { x: 3.0, y: 1.0 };
        let expected_vector = Vector { x: 15.0, y: 44.0 };
        let added_vector = original_vector + additional_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_negative_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let additional_vector = Vector { x: -10.0, y: -20.0 };
        let expected_vector = Vector { x: 2.0, y: 23.0 };
        let added_vector = original_vector + additional_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_to_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let additional_vector = Vector { x: -12.0, y: -43.0 };
        let expected_vector = Vector { x: 0.0, y: 0.0 };
        let added_vector = original_vector + additional_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn subtracts_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let subtracted_vector = Vector { x: 0.0, y: 0.0 };
        let expected_vector = original_vector;
        let added_vector = original_vector - subtracted_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn subtracts_other_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let subtracted_vector = Vector { x: 3.0, y: 1.0 };
        let expected_vector = Vector { x: 9.0, y: 42.0 };
        let added_vector = original_vector - subtracted_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn subtracts_negative_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let subtracted_vector = Vector { x: -10.0, y: -20.0 };
        let expected_vector = Vector { x: 22.0, y: 63.0 };
        let added_vector = original_vector - subtracted_vector;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn subtracts_to_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let subtracted_vector = original_vector;
        let expected_vector = Vector { x: 0.0, y: 0.0 };
        let added_vector = original_vector - subtracted_vector;
        assert_eq!(expected_vector, added_vector);
    }
}
