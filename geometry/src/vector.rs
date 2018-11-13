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

impl Vector {
    /// Calculates the dot product of itself and another vector
    /// # Examples
    /// ```
    /// use myelin_geometry::Vector;
    /// // a · b = c
    /// let a = Vector { x: 2.0, y: 3.0 };
    /// let b = Vector { x: -4.0, y: 10.0 };
    /// let c = a.dot_product(b);
    /// assert_eq!(22.0, c);
    /// ```
    pub fn dot_product(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the cross product of itself and another vector
    /// # Examples
    /// ```
    /// use myelin_geometry::Vector;
    /// // a × b = c
    /// let a = Vector { x: 2.0, y: 3.0 };
    /// let b = Vector { x: -4.0, y: 10.0 };
    /// let c = a.cross_product(b);
    /// assert_eq!(32.0, c);
    /// ```
    pub fn cross_product(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
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
        let vector_to_add = Vector { x: 0.0, y: 0.0 };
        let expected_vector = original_vector;
        let added_vector = original_vector + vector_to_add;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_other_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_add = Vector { x: 3.0, y: 1.0 };
        let expected_vector = Vector { x: 15.0, y: 44.0 };
        let added_vector = original_vector + vector_to_add;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_negative_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_add = Vector { x: -10.0, y: -20.0 };
        let expected_vector = Vector { x: 2.0, y: 23.0 };
        let added_vector = original_vector + vector_to_add;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_to_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_add = Vector { x: -12.0, y: -43.0 };
        let expected_vector = Vector { x: 0.0, y: 0.0 };
        let added_vector = original_vector + vector_to_add;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn adds_when_negative() {
        let original_vector = Vector { x: -12.0, y: -43.0 };
        let vector_to_add = Vector { x: -4.0, y: -2.0 };
        let expected_vector = Vector { x: -16.0, y: -45.0 };
        let added_vector = original_vector + vector_to_add;
        assert_eq!(expected_vector, added_vector);
    }

    #[test]
    fn subtracts_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_subtract = Vector { x: 0.0, y: 0.0 };
        let expected_vector = original_vector;
        let substracted_vector = original_vector - vector_to_subtract;
        assert_eq!(expected_vector, substracted_vector);
    }

    #[test]
    fn subtracts_other_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_subtract = Vector { x: 3.0, y: 1.0 };
        let expected_vector = Vector { x: 9.0, y: 42.0 };
        let substracted_vector = original_vector - vector_to_subtract;
        assert_eq!(expected_vector, substracted_vector);
    }

    #[test]
    fn subtracts_negative_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_subtract = Vector { x: -10.0, y: -20.0 };
        let expected_vector = Vector { x: 22.0, y: 63.0 };
        let substracted_vector = original_vector - vector_to_subtract;
        assert_eq!(expected_vector, substracted_vector);
    }

    #[test]
    fn subtracts_to_zero_vector() {
        let original_vector = Vector { x: 12.0, y: 43.0 };
        let vector_to_subtract = original_vector;
        let expected_vector = Vector { x: 0.0, y: 0.0 };
        let vector_to_subtract = original_vector - vector_to_subtract;
        assert_eq!(expected_vector, vector_to_subtract);
    }

    #[test]
    fn subtracts_when_negative() {
        let original_vector = Vector { x: -12.0, y: -43.0 };
        let vector_to_subtract = Vector { x: -4.0, y: -2.0 };
        let expected_vector = Vector { x: -8.0, y: -41.0 };
        let substracted_vector = original_vector - vector_to_subtract;
        assert_eq!(expected_vector, substracted_vector);
    }

    #[test]
    fn calculates_dot_product() {
        let a = Vector { x: 2.0, y: 3.0 };
        let b = Vector { x: -4.0, y: 10.0 };
        let expected_dot_product = 22.0;
        let dot_product = a.dot_product(b);
        assert_eq!(expected_dot_product, dot_product);
    }

    #[test]
    fn calculates_negative_dot_product() {
        let a = Vector { x: 2.0, y: 3.0 };
        let b = Vector { x: -40.0, y: 10.0 };
        let expected_dot_product = -50.0;
        let dot_product = a.dot_product(b);
        assert_eq!(expected_dot_product, dot_product);
    }

    #[test]
    fn dot_product_is_zero_when_one_side_is_zero() {
        let a = Vector { x: 2.0, y: 3.0 };
        let b = Vector { x: 0.0, y: 0.0 };
        let expected_dot_product = 0.0;
        let dot_product = a.dot_product(b);
        assert_eq!(expected_dot_product, dot_product);
    }

    #[test]
    fn dot_product_is_zero_when_both_sides_are_zero() {
        let a = Vector { x: 0.0, y: 0.0 };
        let b = Vector { x: 0.0, y: 0.0 };
        let expected_dot_product = 0.0;
        let dot_product = a.dot_product(b);
        assert_eq!(expected_dot_product, dot_product);
    }

    #[test]
    fn calculates_cross_product() {
        let a = Vector { x: 2.0, y: 3.0 };
        let b = Vector { x: -4.0, y: 10.0 };
        let expected_cross_product = 32.0;
        let cross_product = a.cross_product(b);
        assert_eq!(expected_cross_product, cross_product);
    }

    #[test]
    fn calculates_negative_cross_product() {
        let a = Vector { x: 2.0, y: 3.0 };
        let b = Vector { x: 40.0, y: 10.0 };
        let expected_cross_product = -100.0;
        let cross_product = a.cross_product(b);
        assert_eq!(expected_cross_product, cross_product);
    }

    #[test]
    fn cross_product_is_zero_when_one_side_is_zero() {
        let a = Vector { x: 2.0, y: 3.0 };
        let b = Vector { x: 0.0, y: 0.0 };
        let expected_cross_product = 0.0;
        let cross_product = a.cross_product(b);
        assert_eq!(expected_cross_product, cross_product);
    }

    #[test]
    fn cross_product_is_zero_when_both_sides_are_zero() {
        let a = Vector { x: 0.0, y: 0.0 };
        let b = Vector { x: 0.0, y: 0.0 };
        let expected_cross_product = 0.0;
        let cross_product = a.cross_product(b);
        assert_eq!(expected_cross_product, cross_product);
    }
}
