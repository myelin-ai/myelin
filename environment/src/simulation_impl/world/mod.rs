//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.Body.html
use super::{ActionError, BodyHandle, PhysicalBody, SensorHandle, World};
use crate::object::*;
use crate::simulation_impl::world::collision_filter::{
    IgnoringCollisionFilter, IgnoringCollisionFilterWrapper,
};
use myelin_geometry::*;
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::query::Proximity;
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::force_generator::{ForceGenerator, ForceGeneratorHandle};
use nphysics2d::math::{Isometry, Point as NPhysicsPoint, Vector as NPhysicsVector};
use nphysics2d::object::{
    BodyHandle as NphysicsBodyHandle, Collider, ColliderHandle, Material, RigidBody,
    SensorHandle as NphysicsSensorHandle,
};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};

pub mod collision_filter;
pub mod force_applier;
pub mod rotation_translator;
use self::force_applier::GenericSingleTimeForceApplierWrapper;

/// An implementation of [`World`] that uses nphysics
/// in the background.
///
/// [`World`]: ./trait.World.html
pub struct NphysicsWorld {
    physics_world: PhysicsWorld<f64>,
    sensor_collisions: HashMap<SensorHandle, HashSet<ColliderHandle>>,
    /// Only used to track active sensors so that we can remove them
    /// when removing an object
    body_sensors: HashMap<BodyHandle, SensorHandle>,
    rotation_translator: Box<dyn NphysicsRotationTranslator>,
    force_generator_handle: ForceGeneratorHandle,
    collision_filter: Arc<RwLock<dyn IgnoringCollisionFilter>>,
}

impl NphysicsWorld {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::world::NphysicsWorld;
    /// use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
    /// use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
    /// use myelin_environment::simulation_impl::world::collision_filter::IgnoringCollisionFilterImpl;
    /// use std::sync::{Arc, RwLock};
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl::default();
    /// let force_applier = SingleTimeForceApplierImpl::default();
    /// let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterImpl::default()));
    /// let mut world = NphysicsWorld::with_timestep(1.0, Box::new(rotation_translator), Box::new(force_applier), collision_filter);
    /// ```
    pub fn with_timestep(
        timestep: f64,
        rotation_translator: Box<dyn NphysicsRotationTranslator>,
        force_applier: Box<dyn SingleTimeForceApplier>,
        collision_filter: Arc<RwLock<dyn IgnoringCollisionFilter>>,
    ) -> Self {
        let mut physics_world = PhysicsWorld::new();

        physics_world.set_timestep(timestep);
        let generic_wrapper = GenericSingleTimeForceApplierWrapper::new(force_applier);
        let force_generator_handle = physics_world.add_force_generator(generic_wrapper);

        const IGNORING_COLLISION_FILTER_NAME: &str = "Collision Filter";

        physics_world
            .collision_world_mut()
            .register_broad_phase_pair_filter(
                IGNORING_COLLISION_FILTER_NAME,
                IgnoringCollisionFilterWrapper {
                    collision_filter: collision_filter.clone(),
                },
            );

        Self {
            physics_world,
            sensor_collisions: HashMap::new(),
            body_sensors: HashMap::new(),
            rotation_translator,
            force_generator_handle,
            collision_filter,
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
        Polygon { vertices }
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
        self.collision_filter
            .read()
            .expect("RwLock was poisoned")
            .is_handle_ignored(body_handle.into())
    }
}

/// This trait translates the rotation from [`Radians`] to the range (-π; π] defined by nphysics
///
/// [`Radians`]: ../../object/struct.Radians.html
pub trait NphysicsRotationTranslator: fmt::Debug {
    /// Converts an `orientation` into a representation that is suitable for nphysics
    fn to_nphysics_rotation(&self, orientation: Radians) -> f64;

    /// Converts a rotation that originates from nphysics into [`Radians`]
    ///
    /// [`Radians`]: ../../object/struct.Radians.html
    fn to_radians(
        &self,
        nphysics_rotation: f64,
    ) -> Result<Radians, NphysicsRotationTranslatorError>;
}

/// The reason why a rotation could not be translated
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NphysicsRotationTranslatorError {
    /// The given nphysics value was not in the range (-π; π]
    InvalidNphysicsValue,
}

impl fmt::Display for NphysicsRotationTranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Given nphysics value is not in the range (-pi; pi]")
    }
}

impl Error for NphysicsRotationTranslatorError {}

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
        .vertices
        .iter()
        .map(|vertex| NPhysicsPoint::new(vertex.x, vertex.y))
        .collect();

    ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"))
}

fn collision_with_sensor(
    sensor_handle: SensorHandle,
    first_handle: CollisionObjectHandle,
    second_handle: CollisionObjectHandle,
) -> Option<CollisionObjectHandle> {
    let sensor_handle = to_nphysics_sensor_handle(sensor_handle);
    if first_handle == sensor_handle {
        Some(second_handle)
    } else {
        None
    }
}

impl World for NphysicsWorld {
    fn step(&mut self) {
        self.physics_world.step();
        for (&sensor_handle, collisions) in &mut self.sensor_collisions {
            for contact in self.physics_world.proximity_events() {
                if let Some(collision) =
                    collision_with_sensor(sensor_handle, contact.collider1, contact.collider2)
                {
                    match contact.new_status {
                        Proximity::WithinMargin | Proximity::Intersecting => {
                            collisions.insert(collision);
                        }
                        Proximity::Disjoint => {
                            collisions.remove(&collision);
                        }
                    }
                }
            }
        }
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
            self.collision_filter
                .write()
                .expect("Lock was poisoned")
                .add_ignored_handle(body_handle.into());
        }

        body_handle
    }

    #[must_use]
    fn remove_body(&mut self, body_handle: BodyHandle) -> Option<PhysicalBody> {
        let physical_body = self.body(body_handle)?;
        let collider_handle = to_collider_handle(body_handle);
        let nphysics_body_handle = self.physics_world.collider_body_handle(collider_handle)?;
        if let Some(sensor_handle) = self.body_sensors.remove(&body_handle) {
            self.sensor_collisions.remove(&sensor_handle)?;
        }
        if physical_body.passable {
            self.collision_filter
                .write()
                .expect("RwLock was poisoned")
                .remove_ignored_handle(body_handle.into());
        }
        self.physics_world.remove_bodies(&[nphysics_body_handle]);
        Some(physical_body)
    }

    #[must_use]
    fn attach_sensor(&mut self, body_handle: BodyHandle, sensor: Sensor) -> Option<SensorHandle> {
        let collider_handle = to_collider_handle(body_handle);
        let parent_handle = self.physics_world.collider_body_handle(collider_handle)?;

        let shape = translate_shape(&sensor.shape);
        let position =
            to_nphysics_isometry(sensor.location, sensor.rotation, &*self.rotation_translator);
        let sensor_handle = self
            .physics_world
            .add_sensor(shape, parent_handle, position);

        let sensor_handle = to_sensor_handle(sensor_handle);
        self.sensor_collisions.insert(sensor_handle, HashSet::new());
        self.body_sensors.insert(body_handle, sensor_handle);
        Some(sensor_handle)
    }

    #[must_use]
    fn body(&self, handle: BodyHandle) -> Option<PhysicalBody> {
        let collider_handle = to_collider_handle(handle);
        self.get_body_from_handle(collider_handle)
    }

    #[must_use]
    fn bodies_within_sensor(
        &self,
        sensor_handle: SensorHandle,
    ) -> Result<Vec<BodyHandle>, ActionError> {
        let collisions = self
            .sensor_collisions
            .get(&sensor_handle)
            .ok_or(ActionError::InvalidHandle)?;

        let bodies_within_sensor = collisions
            .iter()
            .filter(|&&collider_handle| !self.is_sensor_handle(collider_handle))
            .map(|&collider_handle| to_body_handle(collider_handle))
            .collect();

        Ok(bodies_within_sensor)
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
        self.collision_filter
            .read()
            .expect("RwLock was poisoned")
            .is_handle_ignored(body_handle.into())
    }
}

impl NphysicsWorld {
    fn is_sensor_handle(&self, handle: NphysicsSensorHandle) -> bool {
        self.sensor_collisions
            .contains_key(&to_sensor_handle(handle))
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

fn to_sensor_handle(sensor_handle: NphysicsSensorHandle) -> SensorHandle {
    SensorHandle(sensor_handle.0)
}

fn to_nphysics_sensor_handle(sensor_handle: SensorHandle) -> NphysicsSensorHandle {
    CollisionObjectHandle(sensor_handle.0)
}

impl fmt::Debug for NphysicsWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NphysicsWorld")
            .field("physics", &DebugPhysicsWorld(&self.physics_world))
            .finish()
    }
}

/// A helper struct used to implement [`std::fmt::Debug`]
/// for [`NphysicsWorld`]
///
/// [`std::fmt::Debug`]: https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html
/// [`NphysicsWorld`]: ./struct.NphysicsWorld.html
struct DebugPhysicsWorld<'a>(&'a PhysicsWorld<f64>);

impl<'a> fmt::Debug for DebugPhysicsWorld<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicsWorld").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_impl::world::collision_filter::IgnoringCollisionFilterMock;
    use crate::simulation_impl::world::rotation_translator::mock::NphysicsRotationTranslatorMock;
    use nphysics2d::object::BodySet;
    use nphysics2d::solver::IntegrationParameters;
    use std::collections::VecDeque;
    use std::f64::consts::FRAC_PI_2;
    use std::sync::RwLock;
    use std::thread::panicking;
    use unordered_pair::UnorderedPair;

    const DEFAULT_TIMESTEP: f64 = 1.0;

    #[test]
    fn returns_none_when_calling_body_with_invalid_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let invalid_handle = BodyHandle(1337);
        let body = world.body(invalid_handle);
        assert!(body.is_none())
    }

    #[test]
    fn returns_none_when_removing_invalid_object() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let invalid_handle = BodyHandle(123);
        let physical_body = world.remove_body(invalid_handle);
        assert!(physical_body.is_none())
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let movable_body = movable_body(Radians::try_new(3.0).unwrap());

        let handle = world.add_body(movable_body);

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world.body(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = immovable_body(Radians::try_new(3.0).unwrap());

        let handle = world.add_body(body);

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world.body(handle);
    }

    #[test]
    fn removing_object_returns_physical_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let expected_body = movable_body(Radians::try_new(3.0).unwrap());

        let handle = world.add_body(expected_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        let physical_body = world.remove_body(handle).expect("Invalid handle");
        assert_eq!(expected_body, physical_body);
    }

    #[test]
    fn can_remove_object_with_sensor() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        rotation_translator.expect_to_radians_and_return(0.0, Ok(Radians::default()));
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let expected_body = movable_body(Radians::default());

        let handle = world.add_body(expected_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world
            .attach_sensor(handle, sensor())
            .expect("Invalid handle");
        let physical_body = world.remove_body(handle).expect("Invalid handle");
        assert_eq!(expected_body, physical_body);
    }

    #[test]
    fn removed_object_cannot_be_accessed() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let expected_body = movable_body(Radians::try_new(3.0).unwrap());

        let handle = world.add_body(expected_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        let _physical_body = world.remove_body(handle).expect("Invalid handle");
        let removed_body = world.body(handle);
        assert!(removed_body.is_none())
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let rigid_object = movable_body(Radians::try_new(3.0).unwrap());
        let grounded_object = immovable_body(Radians::try_new(3.0).unwrap());

        let rigid_handle = world.add_body(rigid_object);
        let grounded_handle = world.add_body(grounded_object);

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![
                (rigid_handle.into(), false),
                (grounded_handle.into(), false),
            ]));

        let _rigid_body = world.body(rigid_handle);
        let _grounded_body = world.body(grounded_handle);
    }

    #[test]
    fn returns_correct_rigid_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let expected_body = movable_body(Radians::try_new(3.0).unwrap());
        let handle = world.add_body(expected_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_correct_grounded_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(3.0).unwrap(), 3.0);
        rotation_translator.expect_to_radians_and_return(
            3.0,
            Radians::try_new(3.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let expected_body = immovable_body(Radians::try_new(3.0).unwrap());
        let handle = world.add_body(expected_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_sensor_handle_when_attachment_is_valid() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let body = immovable_body(Radians::default());
        let handle = world.add_body(body);
        let sensor_handle = world.attach_sensor(handle, sensor());
        assert!(sensor_handle.is_some())
    }

    #[test]
    fn sensors_do_not_work_without_step() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            location: Point { x: 6.0, y: 6.0 },
            rotation: Radians::default(),
            ..movable_body(Radians::default())
        };
        world.add_body(close_body);

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert!(bodies.is_empty());
    }

    #[test]
    fn sensor_detects_close_bodies() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            location: Point { x: 6.0, y: 6.0 },
            rotation: Radians::default(),
            ..movable_body(Radians::default())
        };
        let expected_handle = world.add_body(close_body);

        let expected_is_valid_pairs = hashmap! {
            UnorderedPair(handle_one.into(), sensor_handle.into()) => true,
            UnorderedPair(handle_one.into(), expected_handle.into()) => true,
            UnorderedPair(sensor_handle.into(), expected_handle.into()) => true,
        };

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_pair_valid_and_return(expected_is_valid_pairs);

        world.step();

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert_eq!(1, bodies.len());
        assert_eq!(expected_handle, bodies[0]);
    }

    #[test]
    fn sensor_detects_non_colliding_bodies() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            location: Point { x: 12.0, y: 12.0 },
            rotation: Radians::default(),
            ..movable_body(Radians::default())
        };
        let expected_handle = world.add_body(close_body);

        let expected_is_valid_pairs = hashmap! {
            UnorderedPair(handle_one.into(), sensor_handle.into()) => true,
            UnorderedPair(handle_one.into(), expected_handle.into()) => true,
            UnorderedPair(sensor_handle.into(), expected_handle.into()) => true,
        };

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_pair_valid_and_return(expected_is_valid_pairs);

        world.step();

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert_eq!(1, bodies.len());
        assert_eq!(expected_handle, bodies[0]);
    }

    #[test]
    fn sensor_does_not_detect_far_away_bodies() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            location: Point { x: 60.0, y: 60.0 },
            rotation: Radians::default(),
            ..movable_body(Radians::default())
        };
        let handle_two = world.add_body(close_body);

        let expected_is_valid_pairs = hashmap! {
            UnorderedPair(handle_one.into(), sensor_handle.into()) => true,
            UnorderedPair(handle_one.into(), handle_two.into()) => true,
            UnorderedPair(sensor_handle.into(), handle_two.into()) => true,
        };

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_pair_valid_and_return(expected_is_valid_pairs);

        world.step();

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert!(bodies.is_empty());
    }

    #[test]
    fn sensor_does_not_detect_touching_sensors() {
        let close_body_location = Point { x: 25.0, y: 0.0 };

        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            location: close_body_location,
            rotation: Radians::default(),
            ..movable_body(Radians::default())
        };
        let close_body_handle = world.add_body(close_body);
        let close_body_sensor_handle = world
            .attach_sensor(close_body_handle, sensor())
            .expect("body handle was invalid");

        let expected_is_valid_pairs = hashmap! {
            UnorderedPair(handle_one.into(), sensor_handle.into()) => true,
            UnorderedPair(handle_one.into(), close_body_handle.into()) => true,
            UnorderedPair(handle_one.into(), close_body_sensor_handle.into()) => true,
            UnorderedPair(close_body_handle.into(), sensor_handle.into()) => true,
            UnorderedPair(close_body_handle.into(), close_body_sensor_handle.into()) => true,
            UnorderedPair(close_body_sensor_handle.into(), sensor_handle.into()) => true,
        };

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_pair_valid_and_return(expected_is_valid_pairs);

        world.step();

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert!(bodies.is_empty());
    }

    #[test]
    fn sensor_does_not_detect_overlapping_sensors() {
        let close_body_location = Point {
            x: 25.0 - 2.0,
            y: 0.0,
        };

        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            location: close_body_location,
            rotation: Radians::default(),
            ..movable_body(Radians::default())
        };
        let close_body_handle = world.add_body(close_body);
        let close_body_sensor_handle = world
            .attach_sensor(close_body_handle, sensor())
            .expect("body handle was invalid");

        let expected_is_valid_pairs = hashmap! {
            UnorderedPair(handle_one.into(), sensor_handle.into()) => true,
            UnorderedPair(handle_one.into(), close_body_handle.into()) => true,
            UnorderedPair(handle_one.into(), close_body_sensor_handle.into()) => true,
            UnorderedPair(handle_one.into(), sensor_handle.into()) => true,
            UnorderedPair(close_body_handle.into(), sensor_handle.into()) => true,
            UnorderedPair(close_body_sensor_handle.into(), sensor_handle.into()) => true,
            UnorderedPair(close_body_handle.into(), close_body_sensor_handle.into()) => true,
        };

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_pair_valid_and_return(expected_is_valid_pairs);

        world.step();

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert!(bodies.is_empty());
    }

    #[test]
    fn returns_none_attaching_sensor_to_inhalid_body_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let invalid_handle = BodyHandle(132_144);
        let sensor_handle = world.attach_sensor(invalid_handle, sensor());
        assert!(sensor_handle.is_none())
    }

    #[test]
    fn returns_err_when_calling_bodies_within_sensor_with_invalid_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierMock::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let invalid_handle = SensorHandle(112_358);
        let body_handles = world.bodies_within_sensor(invalid_handle);

        assert!(body_handles.is_err())
    }

    #[test]
    fn timestep_is_respected() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(0.0).unwrap(), 0.0);
        rotation_translator.expect_to_radians_and_return(
            0.0,
            Radians::try_new(0.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );

        let local_object = movable_body(Radians::default());
        let handle = world.add_body(local_object.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            location: Point { x: 6.0, y: 6.0 },
            rotation: Radians::default(),
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn timestep_can_be_changed() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_radians_and_return(
            0.0,
            Radians::try_new(0.0)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        rotation_translator
            .expect_to_nphysics_rotation_and_return(Radians::try_new(0.0).unwrap(), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        world.set_simulated_timestep(2.0);

        let local_object = movable_body(Radians::default());
        let handle = world.add_body(local_object.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            location: Point { x: 7.0, y: 7.0 },
            rotation: Radians::default(),
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn step_is_ignored_for_rigid_objects_with_no_movement() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(
            Radians::try_new(FRAC_PI_2).unwrap(),
            FRAC_PI_2,
        );
        rotation_translator.expect_to_radians_and_return(
            FRAC_PI_2,
            Radians::try_new(FRAC_PI_2)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let expected_body = immovable_body(Radians::try_new(FRAC_PI_2).unwrap());
        let handle = world.add_body(expected_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn step_is_ignored_for_grounded_objects() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(
            Radians::try_new(FRAC_PI_2).unwrap(),
            FRAC_PI_2,
        );
        rotation_translator.expect_to_radians_and_return(
            FRAC_PI_2,
            Radians::try_new(FRAC_PI_2)
                .map_err(|_| NphysicsRotationTranslatorError::InvalidNphysicsValue),
        );
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter.clone(),
        );
        let body = immovable_body(Radians::try_new(FRAC_PI_2).unwrap());
        let still_body = PhysicalBody {
            mobility: Mobility::Movable(Vector { x: 0.0, y: 0.0 }),
            ..body
        };
        let handle = world.add_body(still_body.clone());

        collision_filter
            .write()
            .expect("RwLock was poisoned")
            .expect_is_handle_ignored_and_return(VecDeque::from(vec![(handle.into(), false)]));

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(still_body), actual_body)
    }

    #[test]
    fn applied_force_is_propagated() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(
            Radians::try_new(FRAC_PI_2).unwrap(),
            FRAC_PI_2,
        );
        let mut force_applier = SingleTimeForceApplierMock::default();
        let expected_force = Force {
            linear: Vector { x: 4.0, y: 10.0 },
            torque: Torque(2.0),
        };
        force_applier.expect_register_force(expected_force.clone());

        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterMock::default()));
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        let expected_body = movable_body(Radians::try_new(FRAC_PI_2).unwrap());
        let handle = world.add_body(expected_body.clone());

        world.apply_force(handle, expected_force);
    }

    fn sensor() -> Sensor {
        Sensor {
            shape: PolygonBuilder::default()
                .vertex(-10.0, -10.0)
                .vertex(10.0, -10.0)
                .vertex(10.0, 10.0)
                .vertex(-10.0, 10.0)
                .build()
                .unwrap(),
            location: Point { x: 0.0, y: 0.0 },
            rotation: Radians::default(),
        }
    }

    fn movable_body(orientation: Radians) -> PhysicalBody {
        PhysicalBody {
            location: Point { x: 5.0, y: 5.0 },
            rotation: orientation,
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

    fn immovable_body(orientation: Radians) -> PhysicalBody {
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
            rotation: orientation,
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
            f.debug_struct("SingleTimeForceApplierMock").finish()
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

            if let Some((ref expected_force,)) = self.expect_register_force {
                // handles cannot be compared
                if force != *expected_force {
                    panic!(
                        "register_force() was called with {:?}, expected {:?}",
                        force, expected_force
                    )
                }
            } else {
                panic!("register_force() was called unexpectedly")
            }
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
