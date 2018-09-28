use super::NphysicsRotationTranslator;
use crate::object::Radians;
use std::f64::consts::PI;

/// We define the rotation as [0; 2π), whereas nphysics defines it as (-π; π]
///
/// So 0°, 90°, 180° and 270° are as follows
/// in npyhisics: 0, -0.5π, π, 0.5π
/// in our notation: 0, 0.5π, π, 1.5π
#[derive(Debug)]
pub struct NphysicsRotationTranslatorImpl {}

impl NphysicsRotationTranslator for NphysicsRotationTranslatorImpl {
    fn to_nphysics_rotation(&self, orientation: Radians) -> f64 {
        if orientation.0 <= PI {
            orientation.0
        } else {
            orientation.0 - (2.0 * PI)
        }
    }

    fn to_radians(&self, nphysics_rotation: f64) -> Radians {
        if nphysics_rotation >= 0.0 {
            Radians(nphysics_rotation)
        } else {
            Radians((2.0 * PI) + nphysics_rotation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    fn test_to_nphysics_rotation(input: Radians, expected: f64) {
        let translator = NphysicsRotationTranslatorImpl {};
        assert_eq!(expected, translator.to_nphysics_rotation(input));
    }

    #[test]
    fn test_to_nphysics_rotation_0() {
        test_to_nphysics_rotation(Radians(0.0), 0.0)
    }

    #[test]
    fn test_to_nphysics_rotation_90() {
        test_to_nphysics_rotation(Radians(FRAC_PI_2), FRAC_PI_2)
    }

    #[test]
    fn test_to_nphysics_rotation_180() {
        test_to_nphysics_rotation(Radians(PI), PI)
    }

    #[test]
    fn test_to_nphysics_rotation_270() {
        test_to_nphysics_rotation(Radians(3.0 * FRAC_PI_2), -FRAC_PI_2)
    }

    fn test_to_radians(input: f64, expected: Radians) {
        let translator = NphysicsRotationTranslatorImpl {};
        assert_eq!(expected, translator.to_radians(input));
    }

    #[test]
    fn test_to_radians_0() {
        test_to_radians(0.0, Radians(0.0))
    }

    #[test]
    fn test_to_radians_90() {
        test_to_radians(FRAC_PI_2, Radians(FRAC_PI_2))
    }

    #[test]
    fn test_to_radians_180() {
        test_to_radians(PI, Radians(PI))
    }

    #[test]
    fn test_to_radians_270() {
        test_to_radians(-FRAC_PI_2, Radians(3.0 * FRAC_PI_2))
    }
}
