//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.Body.html
mod force_applier;
mod physics_world_wrapper;
pub mod rotation_translator;

use self::force_applier::GenericSingleTimeForceApplierWrapper;
pub use self::force_applier::*;
use self::physics_world_wrapper::PhysicsWorldWrapper;
use self::rotation_translator::*;
use super::{BodyHandle, PhysicalBody, World};

use crate::prelude::*;
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::bounding_volume::AABB as NcollideAabb;
use ncollide2d::math::Point as NcollidePoint;
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::force_generator::{ForceGenerator, ForceGeneratorHandle};
use nphysics2d::math::{Isometry, Point as NPhysicsPoint, Vector as NPhysicsVector};
use nphysics2d::object::{
    BodyHandle as NphysicsBodyHandle, Collider, ColliderHandle, Material, RigidBody,
};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::fmt;

use std::collections::HashSet;

/// An implementation of [`World`] that uses nphysics
/// in the background.
///
/// [`World`]: ./trait.World.html
#[derive(Debug)]
pub struct NphysicsWorld {
    physics_world: PhysicsWorldWrapper,
    rotation_translator: Box<dyn NphysicsRotationTranslator>,
    force_generator_handle: ForceGeneratorHandle,
    passable_bodies: HashSet<BodyHandle>,
}

impl NphysicsWorld {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::prelude::*;
    /// use myelin_environment::simulation::simulation_impl::world::nphysics_world::{
    ///     rotation_translator::NphysicsRotationTranslatorImpl, NphysicsWorld,
    ///     SingleTimeForceApplierImpl,
    /// };
    /// use std::sync::{Arc, RwLock};
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl::default();
    /// let force_applier = SingleTimeForceApplierImpl::default();
    /// let mut world =
    ///     NphysicsWorld::with_timestep(1.0, Box::new(rotation_translator), Box::new(force_applier));
    /// ```
    pub fn with_timestep(
        timestep: f64,
        rotation_translator: Box<dyn NphysicsRotationTranslator>,
        force_applier: Box<dyn SingleTimeForceApplier>,
    ) -> Self {
        let mut physics_world = PhysicsWorldWrapper(PhysicsWorld::new());

        physics_world.set_timestep(timestep);
        let generic_wrapper = GenericSingleTimeForceApplierWrapper::new(force_applier);
        let force_generator_handle = physics_world.add_force_generator(generic_wrapper);

        let passable_bodies = HashSet::new();

        Self {
            physics_world,
            rotation_translator,
            force_generator_handle,
            passable_bodies,
        }
    }

    fn get_body_from_handle(&self, collider_handle: ColliderHandle) -> Option<PhysicalBody> {
        let collider = self.physics_world.collider(collider_handle)?;

        let shape = self.get_shape(&collider);
        let location = self.get_location(&collider);
        let rotation = self.get_rotation(&collider);
        let mobility = self.get_mobility(&collider);
        let passable = self.passable(&collider);

        Some(PhysicalBody {
            shape,
            location,
            rotation,
            mobility,
            passable,
        })
    }

    fn get_shape(&self, collider: &Collider<f64>) -> Polygon {
        let convex_polygon: &ConvexPolygon<_> = collider
            .shape()
            .as_shape()
            .expect("Failed to cast shape to a ConvexPolygon");
        let vertices: Vec<_> = convex_polygon
            .points()
            .iter()
            .map(|point| Point {
                x: point.x,
                y: point.y,
            })
            .collect();
        Polygon::try_new(vertices).expect("The polygon from nphysics was not valid")
    }

    fn get_mobility(&self, collider: &Collider<f64>) -> Mobility {
        let body_handle = collider.data().body();
        if body_handle.is_ground() {
            Mobility::Immovable
        } else {
            let rigid_body = self
                .physics_world
                .rigid_body(body_handle)
                .expect("Body handle did not correspond to any rigid body");

            let linear_velocity = rigid_body.velocity().linear;
            let (x, y) = elements(&linear_velocity);
            Mobility::Movable(Vector { x, y })
        }
    }

    fn get_location(&self, collider: &Collider<f64>) -> Point {
        let position = collider.position();
        let (x, y) = elements(&position.translation.vector);
        Point { x, y }
    }

    fn get_rotation(&self, collider: &Collider<f64>) -> Radians {
        let position = collider.position();
        let rotation = position.rotation.angle();

        self.rotation_translator
            .to_radians(rotation)
            .expect("Rotation of a collider could not be translated into Radians")
    }

    fn passable(&self, collider: &Collider<f64>) -> bool {
        let body_handle = to_body_handle(collider.handle());
        self.passable_bodies.contains(&body_handle)
    }
}

/// A [`ForceGenerator`] that applies a given force exactly once
pub trait SingleTimeForceApplier: fmt::Debug + ForceGenerator<f64> {
    /// Registers a [`Force`] to be applied to the body identified by `handle`
    /// in the next step
    ///
    /// [`Force`]: ../../object/struct.Force.html
    fn register_force(&mut self, handle: NphysicsBodyHandle, force: Force);
}

fn elements<N>(vector: &Vector2<N>) -> (N, N)
where
    N: Scalar,
{
    let mut iter = vector.iter();

    (*iter.next().unwrap(), *iter.next().unwrap())
}

fn to_nphysics_isometry(
    location: Point,
    rotation: Radians,
    rotation_translator: &dyn NphysicsRotationTranslator,
) -> Isometry<f64> {
    Isometry::new(
        NPhysicsVector::new(location.x, location.y),
        rotation_translator.to_nphysics_rotation(rotation),
    )
}

fn translate_shape(shape: &Polygon) -> ShapeHandle<f64> {
    let points: Vec<_> = shape
        .vertices()
        .iter()
        .map(|vertex| NPhysicsPoint::new(vertex.x, vertex.y))
        .collect();

    ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"))
}

impl World for NphysicsWorld {
    fn step(&mut self) {
        self.physics_world.step();
    }

    fn add_body(&mut self, body: PhysicalBody) -> BodyHandle {
        let shape = translate_shape(&body.shape);
        /// Arbitrary value
        const OBJECT_DENSITY: f64 = 0.1;
        let local_inertia = shape.inertia(OBJECT_DENSITY);
        let local_center_of_mass = shape.center_of_mass();

        let isometry =
            to_nphysics_isometry(body.location, body.rotation, &*self.rotation_translator);
        let material = Material::default();

        const COLLIDER_MARGIN: f64 = 0.04;
        let handle = match body.mobility {
            Mobility::Immovable => self.physics_world.add_collider(
                COLLIDER_MARGIN,
                shape,
                NphysicsBodyHandle::ground(),
                isometry,
                material,
            ),
            Mobility::Movable(velocity) => {
                let rigid_body_handle = self.physics_world.add_rigid_body(
                    isometry,
                    local_inertia,
                    local_center_of_mass,
                );
                let mut rigid_body = self
                    .physics_world
                    .rigid_body_mut(rigid_body_handle)
                    .expect("Invalid body handle");
                set_velocity(&mut rigid_body, &velocity);
                self.physics_world.add_collider(
                    COLLIDER_MARGIN,
                    shape,
                    rigid_body_handle,
                    Isometry::identity(),
                    material,
                )
            }
        };

        let body_handle = to_body_handle(handle);

        if body.passable {
            self.passable_bodies.insert(body_handle);
        }

        body_handle
    }

    #[must_use]
    fn remove_body(&mut self, body_handle: BodyHandle) -> Option<PhysicalBody> {
        let physical_body = self.body(body_handle)?;
        let collider_handle = to_collider_handle(body_handle);
        let nphysics_body_handle = self.physics_world.collider_body_handle(collider_handle)?;
        if physical_body.passable {
            self.passable_bodies.remove(&body_handle);
        }
        self.physics_world.remove_bodies(&[nphysics_body_handle]);
        Some(physical_body)
    }

    #[must_use]
    fn body(&self, handle: BodyHandle) -> Option<PhysicalBody> {
        let collider_handle = to_collider_handle(handle);
        self.get_body_from_handle(collider_handle)
    }

    #[must_use]
    fn apply_force(&mut self, body_handle: BodyHandle, force: Force) -> Option<()> {
        let collider_handle = to_collider_handle(body_handle);
        let nphysics_body_handle = self.physics_world.collider_body_handle(collider_handle)?;
        self.physics_world
            .force_generator_mut(self.force_generator_handle)
            .downcast_mut::<GenericSingleTimeForceApplierWrapper>()
            .expect("Stored force generator was not of type GenericSingleTimeForceApplierWrapper")
            .inner_mut()
            .register_force(nphysics_body_handle, force);
        Some(())
    }

    fn set_simulated_timestep(&mut self, timestep: f64) {
        self.physics_world.set_timestep(timestep);
    }

    fn is_body_passable(&self, body_handle: BodyHandle) -> bool {
        self.passable_bodies.contains(&body_handle)
    }

    fn bodies_in_area(&self, area: Aabb) -> Vec<BodyHandle> {
        self.physics_world
            .interferences_with_aabb(&to_ncollide_aabb(area))
            .map(|collision| to_body_handle(collision.handle()))
            .collect()
    }
}

fn set_velocity(rigid_body: &mut RigidBody<f64>, velocity: &Vector) {
    let nphysics_velocity = nphysics2d::algebra::Velocity2::linear(velocity.x, velocity.y);
    rigid_body.set_velocity(nphysics_velocity);
}

fn to_body_handle(collider_handle: ColliderHandle) -> BodyHandle {
    BodyHandle(collider_handle.uid())
}

fn to_collider_handle(object_handle: BodyHandle) -> ColliderHandle {
    CollisionObjectHandle(object_handle.0)
}

fn to_ncollide_aabb(aabb: Aabb) -> NcollideAabb<f64> {
    NcollideAabb::new(
        to_ncollide_point(aabb.upper_left),
        to_ncollide_point(aabb.lower_right),
    )
}

fn to_ncollide_point(point: Point) -> NcollidePoint<f64> {
    NcollidePoint::from(Vector2::new(point.x, point.y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use nphysics2d::object::BodySet;
    use nphysics2d::solver::IntegrationParameters;
    use std::f64::consts::FRAC_PI_2;
    use std::sync::RwLock;
    use std::thread::panicking;

    const DEFAULT_TIMESTEP: f64 = 1.0;

    #[test]
    fn returns_none_when_calling_body_with_invalid_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::new();
        let force_applier = SingleTimeForceApplierMock::default();
        let world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let invalid_handle = BodyHandle(1337);
        let body = world.body(invalid_handle);
        assert!(body.is_none())
    }

    #[test]
    fn returns_none_when_removing_invalid_object() {
        let rotation_translator = NphysicsRotationTranslatorMock::new();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let invalid_handle = BodyHandle(123);
        let physical_body = world.remove_body(invalid_handle);
        assert!(physical_body.is_none())
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let movable_body = movable_body();

        let handle = world.add_body(movable_body);

        world.body(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let body = immovable_body();

        let handle = world.add_body(body);

        world.body(handle);
    }

    #[test]
    fn removing_object_returns_physical_body() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = movable_body();

        let handle = world.add_body(expected_body.clone());

        let physical_body = world.remove_body(handle).expect("Invalid handle");
        assert_eq!(expected_body, physical_body);
    }

    #[test]
    fn removed_object_cannot_be_accessed() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = movable_body();

        let handle = world.add_body(expected_body.clone());

        let _physical_body = world.remove_body(handle).expect("Invalid handle");
        let removed_body = world.body(handle);
        assert!(removed_body.is_none())
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::new();
        rotation_translator
            .expect_to_radians(partial_eq(FRAC_PI_2))
            .returns(Ok(Radians::try_new(FRAC_PI_2).unwrap()))
            .times(2);
        rotation_translator
            .expect_to_nphysics_rotation(partial_eq(Radians::try_new(FRAC_PI_2).unwrap()))
            .returns(FRAC_PI_2)
            .times(2);

        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let rigid_object = movable_body();
        let grounded_object = immovable_body();

        let rigid_handle = world.add_body(rigid_object);
        let grounded_handle = world.add_body(grounded_object);

        let _rigid_body = world.body(rigid_handle);
        let _grounded_body = world.body(grounded_handle);
    }

    #[test]
    fn returns_correct_rigid_body() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = movable_body();
        let handle = world.add_body(expected_body.clone());

        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_correct_grounded_body() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = immovable_body();
        let handle = world.add_body(expected_body.clone());

        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn timestep_is_respected() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );

        let local_object = movable_body();
        let handle = world.add_body(local_object.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            location: Point { x: 6.0, y: 6.0 },
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn timestep_can_be_changed() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        world.set_simulated_timestep(2.0);

        let local_object = movable_body();
        let handle = world.add_body(local_object.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            location: Point { x: 7.0, y: 7.0 },
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn step_is_ignored_for_rigid_objects_with_no_movement() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = immovable_body();
        let handle = world.add_body(expected_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn step_is_ignored_for_grounded_objects() {
        let rotation_translator = rotation_translator_for_adding_and_reading_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let body = immovable_body();
        let still_body = PhysicalBody {
            mobility: Mobility::Movable(Vector { x: 0.0, y: 0.0 }),
            ..body
        };
        let handle = world.add_body(still_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(still_body), actual_body)
    }

    #[test]
    fn applied_force_is_propagated() {
        let rotation_translator = rotation_translator_for_adding_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        let expected_force = Force {
            linear: Vector { x: 4.0, y: 10.0 },
            torque: Torque(2.0),
        };
        force_applier.expect_register_force(expected_force.clone());

        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = movable_body();
        let handle = world.add_body(expected_body.clone());

        world.apply_force(handle, expected_force);
    }

    #[test]
    fn bodies_in_area_returns_body_in_area() {
        let rotation_translator = rotation_translator_for_adding_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = movable_body();
        let handle = world.add_body(expected_body);

        world.step();

        assert_eq!(
            vec![handle],
            world.bodies_in_area(Aabb::new((-100.0, -100.0), (100.0, 100.0)))
        );
    }

    #[test]
    fn bodies_in_area_does_not_return_out_of_range_bodies() {
        let rotation_translator = rotation_translator_for_adding_body();
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);

        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
        );
        let expected_body = movable_body();
        let _handle = world.add_body(expected_body.clone());

        world.step();

        assert_eq!(
            Vec::<BodyHandle>::new(),
            world.bodies_in_area(Aabb::new((20.0, 20.0), (30.0, 40.0)))
        );
    }

    fn rotation_translator_for_adding_body() -> NphysicsRotationTranslatorMock {
        let mut rotation_translator = NphysicsRotationTranslatorMock::new();
        rotation_translator
            .expect_to_nphysics_rotation(partial_eq(Radians::try_new(FRAC_PI_2).unwrap()))
            .returns(FRAC_PI_2);
        rotation_translator
    }

    fn rotation_translator_for_adding_and_reading_body() -> NphysicsRotationTranslatorMock {
        let mut rotation_translator = rotation_translator_for_adding_body();
        rotation_translator
            .expect_to_radians(partial_eq(FRAC_PI_2))
            .returns(Ok(Radians::try_new(FRAC_PI_2).unwrap()));
        rotation_translator
    }

    fn movable_body() -> PhysicalBody {
        PhysicalBody {
            location: Point { x: 5.0, y: 5.0 },
            rotation: Radians::try_new(FRAC_PI_2).unwrap(),
            mobility: Mobility::Movable(Vector { x: 1.0, y: 1.0 }),
            shape: PolygonBuilder::default()
                .vertex(-5.0, -5.0)
                .vertex(-5.0, 5.0)
                .vertex(5.0, 5.0)
                .vertex(5.0, -5.0)
                .build()
                .unwrap(),
            passable: false,
        }
    }

    fn immovable_body() -> PhysicalBody {
        PhysicalBody {
            shape: PolygonBuilder::default()
                .vertex(-100.0, -100.0)
                .vertex(100.0, -100.0)
                .vertex(100.0, 100.0)
                .vertex(-100.0, 100.0)
                .build()
                .unwrap(),
            mobility: Mobility::Immovable,
            location: Point { x: 300.0, y: 200.0 },
            rotation: Radians::try_new(FRAC_PI_2).unwrap(),
            passable: false,
        }
    }

    #[derive(Default)]
    struct SingleTimeForceApplierMock {
        expect_register_force: Option<(Force,)>,
        expect_apply_and_return: Option<(bool,)>,

        register_force_was_called: RwLock<bool>,
        apply_was_called: RwLock<bool>,
    }

    impl fmt::Debug for SingleTimeForceApplierMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct(name_of_type!(SingleTimeForceApplierMock))
                .finish()
        }
    }

    impl SingleTimeForceApplierMock {
        fn expect_register_force(&mut self, force: Force) {
            self.expect_register_force = Some((force,))
        }

        fn expect_apply_and_return(&mut self, return_value: bool) {
            self.expect_apply_and_return = Some((return_value,))
        }
    }

    impl SingleTimeForceApplier for SingleTimeForceApplierMock {
        fn register_force(&mut self, _handle: NphysicsBodyHandle, force: Force) {
            *self
                .register_force_was_called
                .write()
                .expect("RwLock was poisoned") = true;

            let expected_force = self
                .expect_register_force
                .clone()
                .expect("register_force() was called unexpectedly")
                .0;

            assert_eq!(
                expected_force, force,
                "register_force() was called with {:?}, expected {:?}",
                force, expected_force
            );
        }
    }

    impl ForceGenerator<f64> for SingleTimeForceApplierMock {
        fn apply(&mut self, _: &IntegrationParameters<f64>, _: &mut BodySet<f64>) -> bool {
            *self.apply_was_called.write().expect("RwLock was poisoned") = true;

            if let Some((return_value,)) = self.expect_apply_and_return {
                // IntegrationParameters and BodySet have no comparison functionality
                return_value
            } else {
                panic!("apply() was called unexpectedly")
            }
        }
    }

    impl Drop for SingleTimeForceApplierMock {
        fn drop(&mut self) {
            if !panicking() {
                if self.expect_register_force.is_some() {
                    assert!(
                        *self
                            .register_force_was_called
                            .read()
                            .expect("RwLock was poisoned"),
                        "expect_register_force() was not called, but was expected"
                    )
                }
                if self.expect_apply_and_return.is_some() {
                    assert!(
                        *self.apply_was_called.read().expect("RwLock was poisoned"),
                        "expect_apply_and_return() was not called, but was expected"
                    )
                }
            }
        }
    }
}
