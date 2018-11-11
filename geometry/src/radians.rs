use std::f64::consts::PI;

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Creates a new instance of [`Radians`].
    /// Returns `None` if the given value is outside the range [0.0; 2π)
    ///
    /// ### Examples
    /// ```
    /// use myelin_geometry::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let rotation = Radians::try_new(PI).expect("Value was outside the range [0.0; 2π)");
    /// ```
    pub fn try_new(value: f64) -> Option<Radians> {
        if value >= 0.0 && value < 2.0 * PI {
            Some(Radians { value })
        } else {
            None
        }
    }

    /// Returns the underlying value
    pub fn value(self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radians_new_with_negative_0_point_1_is_none() {
        let radians = Radians::try_new(-0.1);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_new_with_0_is_some() {
        let radians = Radians::try_new(0.0);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_1_point_9_pi_is_some() {
        let radians = Radians::try_new(1.9 * PI);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_2_pi_is_none() {
        let radians = Radians::try_new(2.0 * PI);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_value_returns_1_when_given_1() {
        let value = 1.0;
        let radians = Radians::try_new(value).unwrap();
        assert_eq!(value, radians.value())
    }

}
