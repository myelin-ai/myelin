//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.Body.html
use super::{BodyHandle, PhysicalBody, SensorHandle, World};
use crate::object::*;
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::query::Proximity;
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::force_generator::{ForceGenerator, ForceGeneratorHandle};
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::{
    BodyHandle as NphysicsBodyHandle, Collider, ColliderHandle, Material, RigidBody,
    SensorHandle as NphysicsSensorHandle,
};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::{HashMap, HashSet};
use std::fmt;

type PhysicsType = f64;

pub mod force_applier;
pub mod rotation_translator;
use self::force_applier::GenericSingleTimeForceApplierWrapper;

/// An implementation of [`World`] that uses nphysics
/// in the background.
///
/// [`World`]: ./trait.World.html
pub struct NphysicsWorld {
    physics_world: PhysicsWorld<PhysicsType>,
    collider_handles: HashMap<ColliderHandle, Kind>,
    sensor_collisions: HashMap<SensorHandle, HashSet<ColliderHandle>>,
    rotation_translator: Box<dyn NphysicsRotationTranslator>,
    force_generator_handle: ForceGeneratorHandle,
}

impl NphysicsWorld {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::world::NphysicsWorld;
    /// use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl::default();
    /// let mut world = NphysicsWorld::with_timestep(1.0, Box::new(rotation_translator));
    /// ```
    pub fn with_timestep(
        timestep: f64,
        rotation_translator: Box<dyn NphysicsRotationTranslator>,
        force_applier: Box<dyn SingleTimeForceApplier>,
    ) -> Self {
        let mut physics_world = PhysicsWorld::new();

        physics_world.set_timestep(timestep);
        let generic_wrapper = GenericSingleTimeForceApplierWrapper::new(force_applier);
        let force_generator_handle = physics_world.add_force_generator(generic_wrapper);

        Self {
            physics_world,
            collider_handles: HashMap::new(),
            sensor_collisions: HashMap::new(),
            rotation_translator,
            force_generator_handle,
        }
    }

    fn get_body_from_handle(&self, collider_handle: ColliderHandle) -> Option<PhysicalBody> {
        let collider = self.physics_world.collider(collider_handle)?;

        let shape = self.get_shape(&collider);
        let position = self.get_position(&collider);
        let mobility = self.get_mobility(&collider);

        Some(PhysicalBody {
            shape,
            position,
            mobility,
        })
    }

    fn get_shape(&self, collider: &Collider<PhysicsType>) -> Polygon {
        let convex_polygon: &ConvexPolygon<_> = collider
            .shape()
            .as_shape()
            .expect("Failed to cast shape to a ConvexPolygon");
        let vertices: Vec<_> = convex_polygon
            .points()
            .iter()
            .map(|vertex| Vertex {
                x: vertex.x.round() as i32,
                y: vertex.y.round() as i32,
            })
            .collect();
        Polygon { vertices }
    }

    fn get_mobility(&self, collider: &Collider<PhysicsType>) -> Mobility {
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
            Mobility::Movable(Velocity {
                x: x as i32,
                y: y as i32,
            })
        }
    }

    fn get_position(&self, collider: &Collider<PhysicsType>) -> Position {
        let position = collider.position();
        let (x, y) = elements(&position.translation.vector);
        let rotation = position.rotation.angle();

        Position {
            location: Location {
                x: x as u32,
                y: y as u32,
            },
            rotation: self.rotation_translator.to_radians(rotation),
        }
    }
}

/// This trait translates the rotation from [`Radians`] to the range (-π; π] defined by nphysics
///
/// [`Radians`]: ../../object/struct.Radians.html
pub trait NphysicsRotationTranslator: fmt::Debug {
    fn to_nphysics_rotation(&self, orientation: Radians) -> f64;
    fn to_radians(&self, nphysics_rotation: f64) -> Radians;
}

pub trait SingleTimeForceApplier: fmt::Debug + ForceGenerator<PhysicsType> {
    fn register_force(&mut self, handle: NphysicsBodyHandle, force: Force);
}

fn elements<N>(vector: &Vector2<N>) -> (N, N)
where
    N: Scalar,
{
    let mut iter = vector.iter();

    (*iter.next().unwrap(), *iter.next().unwrap())
}

fn translate_position(
    position: &Position,
    rotation_translator: &dyn NphysicsRotationTranslator,
) -> Isometry<PhysicsType> {
    Isometry::new(
        Vector::new(
            PhysicsType::from(position.location.x),
            PhysicsType::from(position.location.y),
        ),
        rotation_translator.to_nphysics_rotation(position.rotation),
    )
}

fn translate_shape(shape: &Polygon) -> ShapeHandle<PhysicsType> {
    let points: Vec<_> = shape
        .vertices
        .iter()
        .map(|vertex| Point::new(PhysicsType::from(vertex.x), PhysicsType::from(vertex.y)))
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
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();

        let isometry = translate_position(&body.position, &*self.rotation_translator);
        let material = Material::default();

        let handle = match body.mobility {
            Mobility::Immovable => self.physics_world.add_collider(
                0.04,
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
                    0.04,
                    shape,
                    rigid_body_handle,
                    Isometry::identity(),
                    material,
                )
            }
        };

        to_body_handle(handle)
    }

    fn attach_sensor(&mut self, body_handle: BodyHandle, sensor: Sensor) -> Option<SensorHandle> {
        let collider_handle = to_collider_handle(body_handle);
        let parent_handle = self.physics_world.collider_body_handle(collider_handle)?;

        let shape = translate_shape(&sensor.shape);
        let position = translate_position(&sensor.position, &*self.rotation_translator);
        let sensor_handle = self
            .physics_world
            .add_sensor(shape, parent_handle, position);

        let sensor_handle = to_sensor_handle(sensor_handle);
        self.sensor_collisions.insert(sensor_handle, HashSet::new());
        Some(sensor_handle)
    }

    fn body(&self, handle: BodyHandle) -> Option<PhysicalBody> {
        let collider_handle = to_collider_handle(handle);
        self.get_body_from_handle(collider_handle)
    }

    fn bodies_within_sensor(&self, sensor_handle: SensorHandle) -> Option<Vec<BodyHandle>> {
        let collisions = self.sensor_collisions.get(&sensor_handle)?;
        let bodies_within_sensor = collisions
            .iter()
            .map(|&collider_handle| to_body_handle(collider_handle))
            .collect();
        Some(bodies_within_sensor)
    }

    fn apply_force(&mut self, body_handle: BodyHandle, force: Force) -> Option<()> {
        let collider_handle = to_collider_handle(body_handle);
        let nphysics_body_handle = self.physics_world.collider(collider_handle)?.data().body();
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
}

fn set_velocity(rigid_body: &mut RigidBody<PhysicsType>, velocity: &Velocity) {
    let nphysics_velocity = nphysics2d::algebra::Velocity2::linear(
        PhysicsType::from(velocity.x),
        PhysicsType::from(velocity.y),
    );
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
            .field("collider_handles", &self.collider_handles)
            .field("physics", &DebugPhysicsWorld(&self.physics_world))
            .finish()
    }
}

/// A helper struct used to implement [`std::fmt::Debug`]
/// for [`NphysicsWorld`]
///
/// [`std::fmt::Debug`]: https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html
/// [`NphysicsWorld`]: ./struct.NphysicsWorld.html
struct DebugPhysicsWorld<'a>(&'a PhysicsWorld<PhysicsType>);

impl<'a> fmt::Debug for DebugPhysicsWorld<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicsWorld").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::rotation_translator::mock::NphysicsRotationTranslatorMock;
    use super::*;
    use crate::object_builder::PolygonBuilder;
    use nphysics2d::object::BodySet;
    use nphysics2d::solver::IntegrationParameters;
    use std::f64::consts::FRAC_PI_2;
    use std::sync::RwLock;
    use std::thread::panicking;

    const DEFAULT_TIMESTEP: f64 = 1.0;

    #[test]
    fn returns_none_when_calling_body_with_invalid_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierMock::default();
        let world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let invalid_handle = BodyHandle(1337);
        let body = world.body(invalid_handle);
        assert!(body.is_none())
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let movable_body = movable_body(Radians(3.0));

        let handle = world.add_body(movable_body);
        world.body(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let body = immovable_body(Radians(3.0));

        let handle = world.add_body(body);
        world.body(handle);
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let rigid_object = movable_body(Radians(3.0));
        let grounded_object = immovable_body(Radians(3.0));

        let rigid_handle = world.add_body(rigid_object);
        let grounded_handle = world.add_body(grounded_object);

        let _rigid_body = world.body(rigid_handle);
        let _grounded_body = world.body(grounded_handle);
    }

    #[test]
    fn returns_correct_rigid_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let expected_body = movable_body(Radians(3.0));
        let handle = world.add_body(expected_body.clone());
        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_correct_grounded_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let expected_body = immovable_body(Radians(3.0));
        let handle = world.add_body(expected_body.clone());
        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_sensor_handle_when_attachment_is_valid() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians::default(), 0.0);
        let force_applier = SingleTimeForceApplierMock::default();
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
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
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );;
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 6, y: 6 },
                rotation: Radians::default(),
            },
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
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 6, y: 6 },
                rotation: Radians::default(),
            },
            ..movable_body(Radians::default())
        };
        let expected_handle = world.add_body(close_body);

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
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 12, y: 12 },
                rotation: Radians::default(),
            },
            ..movable_body(Radians::default())
        };
        let expected_handle = world.add_body(close_body);

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
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let body = movable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 60, y: 60 },
                rotation: Radians::default(),
            },
            ..movable_body(Radians::default())
        };
        world.add_body(close_body);

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
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let invalid_handle = BodyHandle(132144);
        let sensor_handle = world.attach_sensor(invalid_handle, sensor());
        assert!(sensor_handle.is_none())
    }

    #[test]
    fn returns_none_when_calling_bodies_within_sensor_with_invalid_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let force_applier = SingleTimeForceApplierMock::default();
        let world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let invalid_handle = SensorHandle(112358);
        let body_handles = world.bodies_within_sensor(invalid_handle);

        assert!(body_handles.is_none())
    }

    #[test]
    fn timestep_is_respected() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(0.0), 0.0);
        rotation_translator.expect_to_radians_and_return(0.0, Radians(0.0));
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );

        let local_object = movable_body(Radians::default());
        let handle = world.add_body(local_object.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            position: Position {
                location: Location { x: 6, y: 6 },
                rotation: Radians::default(),
            },
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn timestep_can_be_changed() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_radians_and_return(0.0, Radians(0.0));
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(0.0), 0.0);
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        world.set_simulated_timestep(2.0);

        let local_object = movable_body(Radians::default());
        let handle = world.add_body(local_object.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            position: Position {
                location: Location { x: 7, y: 7 },
                rotation: Radians::default(),
            },
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn step_is_ignored_for_rigid_objects_with_no_movement() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(FRAC_PI_2), FRAC_PI_2);
        rotation_translator.expect_to_radians_and_return(FRAC_PI_2, Radians(FRAC_PI_2));
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let expected_body = immovable_body(Radians(FRAC_PI_2));
        let handle = world.add_body(expected_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn step_is_ignored_for_grounded_objects() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(FRAC_PI_2), FRAC_PI_2);
        rotation_translator.expect_to_radians_and_return(FRAC_PI_2, Radians(FRAC_PI_2));
        let mut force_applier = SingleTimeForceApplierMock::default();
        force_applier.expect_apply_and_return(true);
        let mut world = NphysicsWorld::with_timestep(
            DEFAULT_TIMESTEP,
            Box::new(rotation_translator),
            Box::new(force_applier),
        );
        let body = immovable_body(Radians(FRAC_PI_2));
        let still_body = PhysicalBody {
            mobility: Mobility::Movable(Velocity { x: 0, y: 0 }),
            ..body
        };
        let handle = world.add_body(still_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(still_body), actual_body)
    }

    fn sensor() -> Sensor {
        Sensor {
            shape: PolygonBuilder::new()
                .vertex(-10, -10)
                .vertex(10, -10)
                .vertex(10, 10)
                .vertex(-10, 10)
                .build()
                .unwrap(),
            position: Position {
                location: Location { x: 0, y: 0 },
                rotation: Radians::default(),
            },
        }
    }

    fn movable_body(orientation: Radians) -> PhysicalBody {
        PhysicalBody {
            position: Position {
                location: Location { x: 5, y: 5 },
                rotation: orientation,
            },
            mobility: Mobility::Movable(Velocity { x: 1, y: 1 }),
            shape: PolygonBuilder::new()
                .vertex(-5, -5)
                .vertex(-5, 5)
                .vertex(5, 5)
                .vertex(5, -5)
                .build()
                .unwrap(),
        }
    }

    fn immovable_body(orientation: Radians) -> PhysicalBody {
        PhysicalBody {
            shape: PolygonBuilder::new()
                .vertex(-100, -100)
                .vertex(100, -100)
                .vertex(100, 100)
                .vertex(-100, 100)
                .build()
                .unwrap(),
            mobility: Mobility::Immovable,
            position: Position {
                location: Location { x: 300, y: 200 },
                rotation: orientation,
            },
        }
    }

    #[derive(Default)]
    struct SingleTimeForceApplierMock {
        expect_register_force: Option<(NphysicsBodyHandle, Force)>,
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
        fn expect_register_force(&mut self, body_handle: NphysicsBodyHandle, force: Force) {
            self.expect_register_force = Some((body_handle, force))
        }

        fn expect_apply_and_return(&mut self, return_value: bool) {
            self.expect_apply_and_return = Some((return_value,))
        }
    }

    impl SingleTimeForceApplier for SingleTimeForceApplierMock {
        fn register_force(&mut self, handle: NphysicsBodyHandle, force: Force) {
            *self
                .register_force_was_called
                .write()
                .expect("RwLock was poisoned") = true;

            if let Some((ref expected_handle, ref expected_force)) = self.expect_register_force {
                if handle != *expected_handle || force != *expected_force {
                    panic!(
                        "register_force() was called with {:?} and {:?}, expected {:?} and {:?}",
                        handle, force, expected_handle, expected_force
                    )
                }
            } else {
                panic!("register_force() was called unexpectedly")
            }
        }
    }

    impl ForceGenerator<PhysicsType> for SingleTimeForceApplierMock {
        fn apply(
            &mut self,
            _: &IntegrationParameters<PhysicsType>,
            _: &mut BodySet<PhysicsType>,
        ) -> bool {
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
            if panicking() {
                return;
            }
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
