use std::f64::consts::PI;

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Radians {
    value: f64,
}

impl Radians {
    pub fn new(value: f64) -> Option<Radians> {
        if Self::is_in_range(value) {
            Some(Radians { value })
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn is_in_range(value: f64) -> bool {
    value >= 0.0 && value < 2.0 * PI
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn radians_new_with_negative_0_point_1_is_none() {
        let radians = Radians::new(-0.1);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_new_with_0_is_some() {
        let radians = Radians::new(0.0);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_1_point_9_pi_is_some() {
        let radians = Radians::new(1.9 * PI);
        assert!(radians.is_some())
    }

    #[test]
    fn radians_new_with_2_pi_is_none() {
        let radians = Radians::new(2.0 * PI);
        assert!(radians.is_none())
    }

    #[test]
    fn radians_value_returns_1_when_given_1() {
        let value = 1.0;
        let radians = Radians::new(value).unwrap();
        assert_eq!(value, radians.value())
    }

}
