//! Implementation for NphysicsRotationTranslator

use super::NphysicsRotationTranslator;
use crate::object::Radians;
use std::f64::consts::PI;

/// Translates the rotation from Radians to the range (-π; π] defined by nphysics
#[derive(Default, Debug)]
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

    #[test]
    fn to_nphysics_rotation_returns_0_when_passed_0() {
        verify_to_nphysics_rotation_returns_exoected_result(Radians(0.0), 0.0)
    }

    #[test]
    fn to_nphysics_rotation_returns_half_pi_when_passed_half_pi() {
        verify_to_nphysics_rotation_returns_exoected_result(Radians(FRAC_PI_2), FRAC_PI_2)
    }

    #[test]
    fn to_nphysics_rotation_returns_pi_when_passed_pi() {
        verify_to_nphysics_rotation_returns_exoected_result(Radians(PI), PI)
    }

    #[test]
    fn to_nphysics_rotation_returns_negative_half_pi_when_passed_one_and_a_half_pi() {
        verify_to_nphysics_rotation_returns_exoected_result(Radians(3.0 * FRAC_PI_2), -FRAC_PI_2)
    }

    #[test]
    fn to_radians_returns_0_when_passed_0() {
        verify_to_radians_returns_expected_result(0.0, Radians(0.0))
    }

    #[test]
    fn to_radians_returns_half_pi_when_passed_half_pi() {
        verify_to_radians_returns_expected_result(FRAC_PI_2, Radians(FRAC_PI_2))
    }

    #[test]
    fn to_radians_returns_returns_pi_when_passed_pi() {
        verify_to_radians_returns_expected_result(PI, Radians(PI))
    }

    #[test]
    fn to_radians_returns_one_and_a_half_pi_when_passed_negative_half_pi() {
        verify_to_radians_returns_expected_result(-FRAC_PI_2, Radians(3.0 * FRAC_PI_2))
    }

    fn verify_to_nphysics_rotation_returns_exoected_result(input: Radians, expected: f64) {
        let translator = NphysicsRotationTranslatorImpl::default();
        assert_eq!(expected, translator.to_nphysics_rotation(input));
    }

    fn verify_to_radians_returns_expected_result(input: f64, expected: Radians) {
        let translator = NphysicsRotationTranslatorImpl::default();
        assert_eq!(expected, translator.to_radians(input));
    }

}
