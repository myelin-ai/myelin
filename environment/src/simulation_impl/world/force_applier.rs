use super::{PhysicsType, SingleTimeForceApplier};
use crate::object::Force;
use nphysics2d::force_generator::ForceGenerator;
use nphysics2d::math::Force as NphysicsForce;
use nphysics2d::object::{BodyHandle, BodySet};
use nphysics2d::solver::IntegrationParameters;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SingleTimeForceApplierImpl {
    forces_to_apply: HashMap<BodyHandle, Force>,
}

impl SingleTimeForceApplierImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SingleTimeForceApplier for SingleTimeForceApplierImpl {
    fn register_force(&mut self, handle: BodyHandle, force: Force) {
        self.forces_to_apply.insert(handle, force);
    }
}

impl ForceGenerator<PhysicsType> for SingleTimeForceApplierImpl {
    fn apply(
        &mut self,
        _: &IntegrationParameters<PhysicsType>,
        bodies: &mut BodySet<PhysicsType>,
    ) -> bool {
        for (body_handle, force) in self.forces_to_apply.drain() {
            if bodies.contains(body_handle) {
                let mut body = bodies.body_part_mut(body_handle);
                let nphysics_force = NphysicsForce::from_slice(&[
                    PhysicsType::from(force.linear.x),
                    PhysicsType::from(force.linear.y),
                    force.torque.0,
                ]);
                body.apply_force(&nphysics_force);
            }
        }

        const KEEP_FORCE_GENERATOR_AFTER_APPLICATION: bool = true;
        KEEP_FORCE_GENERATOR_AFTER_APPLICATION
    }
}

#[derive(Debug)]
pub struct GenericSingleTimeForceApplierWrapper {
    force_applier: Box<dyn SingleTimeForceApplier>,
}

impl GenericSingleTimeForceApplierWrapper {
    pub fn new(force_applier: Box<dyn SingleTimeForceApplier>) -> Self {
        Self { force_applier }
    }
}

impl ForceGenerator<PhysicsType> for GenericSingleTimeForceApplierWrapper {
    fn apply(
        &mut self,
        integration_parameters: &IntegrationParameters<PhysicsType>,
        body_set: &mut BodySet<PhysicsType>,
    ) -> bool {
        self.force_applier.apply(integration_parameters, body_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::*;
    use crate::object_builder::PolygonBuilder;
    use crate::simulation_impl::world::rotation_translator::mock::NphysicsRotationTranslatorMock;
    use crate::simulation_impl::world::{NphysicsWorld, PhysicalBody, World};

    const DEFAULT_TIMESTEP: f64 = 1.0;

    #[test]
    fn can_be_injected() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierImpl::new();
        let world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            GenericSingleTimeForceApplierWrapper::new(Box::new(force_applier)),
        );
    }

    #[test]
    fn force_does_nothing_before_step() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(0.0), 0.0);
        rotation_translator.expect_to_radians_and_return(0.0, Radians(0.0));
        let force_applier = SingleTimeForceApplierImpl::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            GenericSingleTimeForceApplierWrapper::new(Box::new(force_applier)),
        );

        let expected_object = physical_body();
        let handle = world.add_body(expected_object.clone());

        let force = Force {
            linear: LinearForce { x: 1000, y: 2000 },
            torque: Torque(9.0),
        };
        world
            .apply_force(handle, force)
            .expect("Invalid object handle");

        let actual_body = world.body(handle);
        assert_eq!(Some(expected_object), actual_body);
    }

    #[test]
    fn zero_force_is_ignored() {
        let object = physical_body();
        let force = Force {
            linear: LinearForce::default(),
            torque: Torque::default(),
        };
        let expected_position = object.position.clone();
        test_force(object, expected_position, force);
    }

    #[test]
    fn torque_with_no_linear_force_only_changes_rotation() {
        let object = physical_body();
        let force = Force {
            linear: LinearForce::default(),
            torque: Torque(1.0),
        };
        let expected_position = Position {
            // To do: Use actual values
            rotation: Radians(1.1),
            ..object.position.clone()
        };
        test_force(object, expected_position, force);
    }

    #[test]
    fn negative_torque_results_in_negative_rotation() {
        let object = physical_body();
        let force = Force {
            linear: LinearForce::default(),
            torque: Torque(-2.0),
        };
        let expected_position = Position {
            // To do: Use actual values
            rotation: Radians(-1.1),
            ..object.position.clone()
        };
        test_force(object, expected_position, force);
    }

    #[test]
    fn linear_force_with_no_torque_changes_only_location() {
        let object = physical_body();
        let force = Force {
            linear: LinearForce { x: 5, y: 5 },
            torque: Torque::default(),
        };
        let expected_position = Position {
            // To do: Use actual values
            location: Location { x: 10, y: 10 },
            ..object.position.clone()
        };
        test_force(object, expected_position, force);
    }

    #[test]
    fn negative_linear_force_results_in_lower_location() {
        let object = physical_body();
        let force = Force {
            linear: LinearForce { x: -5, y: -5 },
            torque: Torque::default(),
        };
        let expected_position = Position {
            // To do: Use actual values
            location: Location { x: 1, y: 1 },
            ..object.position.clone()
        };
        test_force(object, expected_position, force);
    }

    #[test]
    fn linear_force_and_torque_can_be_combined() {
        let object = physical_body();
        let force = Force {
            linear: LinearForce { x: -5, y: -5 },
            torque: Torque(1.5),
        };
        let expected_position = Position {
            // To do: Use actual values
            location: Location { x: 1, y: 1 },
            rotation: Radians(2.0),
        };
        test_force(object, expected_position, force);
    }

    fn physical_body() -> PhysicalBody {
        PhysicalBody {
            position: Position {
                location: Location { x: 5, y: 5 },
                rotation: Radians(3.0),
            },
            mobility: Mobility::Movable(Velocity::default()),
            shape: PolygonBuilder::new()
                .vertex(-5, -5)
                .vertex(-5, 5)
                .vertex(5, 5)
                .vertex(5, -5)
                .build()
                .unwrap(),
        }
    }

    fn test_force(body: PhysicalBody, expected_position: Position, force: Force) {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(0.0), 0.0);
        rotation_translator.expect_to_radians_and_return(0.0, Radians(0.0));
        let force_applier = SingleTimeForceApplierImpl::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            GenericSingleTimeForceApplierWrapper::new(Box::new(force_applier)),
        );

        let handle = world.add_body(body.clone());

        const BODY_HANDLE_ERROR: &str = "Invalid object handle";
        world.apply_force(handle, force).expect(BODY_HANDLE_ERROR);

        world.step();
        world.step();

        let actual_body = world.body(handle).expect(BODY_HANDLE_ERROR);
        let expected_body = PhysicalBody {
            position: expected_position,
            ..body
        };
        assert_eq!(expected_body, actual_body);
    }

}
