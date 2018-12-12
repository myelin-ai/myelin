use std::error::Error;
use std::f64::consts::PI;
use std::fmt;
/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Creates a new instance of [`Radians`].
    ///
    /// ### Errors
    /// Returns a [`RadiansError`] if the given value is outside the range [0.0; 2π)
    ///
    /// ### Examples
    /// ```
    /// use myelin_geometry::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let rotation = Radians::try_new(PI).expect("Value was outside the range [0.0; 2π)");
    /// ```
    pub fn try_new(value: f64) -> Result<Radians, RadiansError> {
        if value >= 0.0 && value < 2.0 * PI {
            Ok(Radians { value })
        } else {
            Err(RadiansError::OutOfRange)
        }
    }

    /// Returns the underlying value
    pub fn value(self) -> f64 {
        self.value
    }
}

/// The reason why a [`Radians`] instance could not be created
#[derive(Debug)]
pub enum RadiansError {
    /// The given value was not in the range [0.0; 2π)
    OutOfRange,
}

impl fmt::Display for RadiansError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Given value is not in range [0.0; 2π)")
    }
}

impl Error for RadiansError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn radians_new_with_negative_0_point_1_is_none() {
        let radians = Radians::try_new(-0.1);
        assert!(radians.is_err())
    }

    #[test]
    fn radians_new_with_0_is_some() {
        let radians = Radians::try_new(0.0);
        assert!(radians.is_ok())
    }

    #[test]
    fn radians_new_with_1_point_9_pi_is_some() {
        let radians = Radians::try_new(1.9 * PI);
        assert!(radians.is_ok())
    }

    #[test]
    fn radians_new_with_2_pi_is_none() {
        let radians = Radians::try_new(2.0 * PI);
        assert!(radians.is_err())
    }

    #[test]
    fn radians_value_returns_1_when_given_1() {
        let value = 1.0;
        let radians = Radians::try_new(value).unwrap();
        assert_nearly_eq!(value, radians.value())
    }

}
