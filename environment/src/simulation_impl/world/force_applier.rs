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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_impl::world::rotation_translator::mock::NphysicsRotationTranslatorMock;
    use crate::simulation_impl::world::NphysicsWorld;

    const DEFAULT_TIMESTEP: f64 = 1.0;

    #[test]
    fn can_be_added() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierImpl::new();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
    }

}
