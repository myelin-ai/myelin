//! Implementation for [`NphysicsRotationTranslator`]

use super::{NphysicsRotationTranslator, NphysicsRotationTranslatorError};
use myelin_geometry::Radians;
use std::f64::consts::PI;

/// Translates the rotation from [`Radians`] to the range (-π; π] defined by nphysics
#[derive(Default, Debug)]
pub struct NphysicsRotationTranslatorImpl {}

impl NphysicsRotationTranslator for NphysicsRotationTranslatorImpl {
    fn to_nphysics_rotation(&self, orientation: Radians) -> f64 {
        if orientation.value() <= PI {
            orientation.value()
        } else {
            orientation.value() - (2.0 * PI)
        }
    }

    fn to_radians(
        &self,
        nphysics_rotation: f64,
    ) -> Result<Radians, NphysicsRotationTranslatorError> {
        const EPSILON: f64 = 1.0e-15;
        let rounded_rotation = if nphysics_rotation.abs() < EPSILON {
            0.0
        } else {
            nphysics_rotation
        };

        let adjusted_rotation = if rounded_rotation >= 0.0 {
            rounded_rotation
        } else {
            (2.0 * PI) + rounded_rotation
        };

        Radians::try_new(adjusted_rotation)
            .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue)
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub struct NphysicsRotationTranslatorMock {
        expect_to_nphysics_rotation_and_return: Option<(Radians, f64)>,
        expect_to_radians_and_return:
            Option<(f64, Result<Radians, NphysicsRotationTranslatorError>)>,

        to_nphysics_rotation_was_called: RefCell<bool>,
        to_radians_was_called: RefCell<bool>,
    }

    impl NphysicsRotationTranslatorMock {
        pub fn expect_to_nphysics_rotation_and_return(
            &mut self,
            input_value: Radians,
            return_value: f64,
        ) {
            self.expect_to_nphysics_rotation_and_return = Some((input_value, return_value))
        }

        pub fn expect_to_radians_and_return(
            &mut self,
            input_value: f64,
            return_value: Result<Radians, NphysicsRotationTranslatorError>,
        ) {
            self.expect_to_radians_and_return = Some((input_value, return_value))
        }
    }

    impl NphysicsRotationTranslator for NphysicsRotationTranslatorMock {
        fn to_nphysics_rotation(&self, orientation: Radians) -> f64 {
            *self.to_nphysics_rotation_was_called.borrow_mut() = true;

            if let Some((expected_input, expected_output)) =
                self.expect_to_nphysics_rotation_and_return
            {
                if orientation != expected_input {
                    panic!(
                        "to_nphysics_rotation() was called with {:?}, expected {:?}",
                        orientation, expected_input
                    )
                }

                expected_output
            } else {
                panic!("to_nphysics_rotation() was called unexpectedly")
            }
        }

        fn to_radians(
            &self,
            nphysics_rotation: f64,
        ) -> Result<Radians, NphysicsRotationTranslatorError> {
            *self.to_radians_was_called.borrow_mut() = true;

            if let Some((expected_input, expected_output)) =
                self.expect_to_radians_and_return.clone()
            {
                if nphysics_rotation != expected_input {
                    panic!(
                        "to_radians() was called with {:?}, expected {:?}",
                        nphysics_rotation, expected_input
                    )
                }

                expected_output
            } else {
                panic!("to_radians() was called unexpectedly")
            }
        }
    }

    impl Drop for NphysicsRotationTranslatorMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_to_nphysics_rotation_and_return.is_some() {
                assert!(
                    *self.to_nphysics_rotation_was_called.borrow(),
                    "to_nphysics_rotation() was not called, but was expected"
                )
            }
            if self.expect_to_radians_and_return.is_some() {
                assert!(
                    *self.to_radians_was_called.borrow(),
                    "to_radians() was not called, but was expected"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn to_nphysics_rotation_returns_0_when_passed_0() {
        verify_to_nphysics_rotation_returns_expected_result(Radians::try_new(0.0).unwrap(), 0.0)
    }

    #[test]
    fn to_nphysics_rotation_returns_half_pi_when_passed_half_pi() {
        verify_to_nphysics_rotation_returns_expected_result(
            Radians::try_new(FRAC_PI_2).unwrap(),
            FRAC_PI_2,
        )
    }

    #[test]
    fn to_nphysics_rotation_returns_pi_when_passed_pi() {
        verify_to_nphysics_rotation_returns_expected_result(Radians::try_new(PI).unwrap(), PI)
    }

    #[test]
    fn to_nphysics_rotation_returns_negative_half_pi_when_passed_one_and_a_half_pi() {
        verify_to_nphysics_rotation_returns_expected_result(
            Radians::try_new(3.0 * FRAC_PI_2).unwrap(),
            -FRAC_PI_2,
        )
    }

    #[test]
    fn to_radians_returns_0_when_passed_0() {
        verify_to_radians_returns_expected_result(
            0.0,
            Radians::try_new(0.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        )
    }

    #[test]
    fn to_radians_returns_half_pi_when_passed_half_pi() {
        verify_to_radians_returns_expected_result(
            FRAC_PI_2,
            Radians::try_new(FRAC_PI_2)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        )
    }

    #[test]
    fn to_radians_returns_returns_pi_when_passed_pi() {
        verify_to_radians_returns_expected_result(
            PI,
            Radians::try_new(PI).map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        )
    }

    #[test]
    fn to_radians_returns_one_and_a_half_pi_when_passed_negative_half_pi() {
        verify_to_radians_returns_expected_result(
            -FRAC_PI_2,
            Radians::try_new(3.0 * FRAC_PI_2)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        )
    }

    #[test]
    fn to_radians_works_with_almost_zero_value() {
        verify_to_radians_returns_expected_result(
            -0.000_000_000_000_000_275_574_467_583_596_6,
            Radians::try_new(0.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        )
    }

    #[test]
    fn to_radians_works_with_value_close_to_epsilon() {
        let translator = NphysicsRotationTranslatorImpl::default();
        assert!(translator
            .to_radians(-0.000_000_000_000_002_755_744_675_835_966)
            .is_ok());
    }

    fn verify_to_nphysics_rotation_returns_expected_result(input: Radians, expected: f64) {
        let translator = NphysicsRotationTranslatorImpl::default();
        assert_eq!(expected, translator.to_nphysics_rotation(input));
    }

    fn verify_to_radians_returns_expected_result(
        input: f64,
        expected: Result<Radians, NphysicsRotationTranslatorError>,
    ) {
        let translator = NphysicsRotationTranslatorImpl::default();
        assert_eq!(expected, translator.to_radians(input));
    }

}
