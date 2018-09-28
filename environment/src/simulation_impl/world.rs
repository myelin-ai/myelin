//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.Body.html
use super::{BodyHandle, PhysicalBody, SensorHandle, World};
use crate::object::*;
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::events::ContactEvent;
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::{
    BodyHandle as NphysicsBodyHandle, Collider, ColliderHandle, Material, RigidBody,
    SensorHandle as NphysicsSensorHandle,
};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::fmt;

type PhysicsType = f64;

/// An implementation of [`World`] that uses nphysics
/// in the background.
///
/// [`World`]: ./trait.World.html
#[derive(Default)]
pub struct NphysicsWorld {
    physics_world: PhysicsWorld<PhysicsType>,
    collider_handles: HashMap<ColliderHandle, Kind>,
    sensor_collisions: HashMap<SensorHandle, HashSet<ColliderHandle>>,
}

impl NphysicsWorld {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::world::NphysicsWorld;
    /// let mut world = NphysicsWorld::with_timestep(1.0);
    /// ```
    pub fn with_timestep(timestep: f64) -> Self {
        let mut physics_world = PhysicsWorld::new();

        physics_world.set_timestep(timestep);

        Self {
            physics_world,
            collider_handles: HashMap::new(),
            sensor_collisions: HashMap::new(),
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
            }).collect();
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
            rotation: Radians(rotation + NPHYSICS_ROTATION_OFFSET),
        }
    }
}
/// The offset needed because we define orientation as [0; 2π)
/// and nphysics defines rotation as (-π; π]
/// See http://nalgebra.org/rustdoc/nalgebra/geometry/type.UnitComplex.html#method.angle
const NPHYSICS_ROTATION_OFFSET: f64 = PI;
fn to_nphysics_rotation(orientation: Radians) -> f64 {
    orientation.0 - NPHYSICS_ROTATION_OFFSET
}

fn elements<N>(vector: &Vector2<N>) -> (N, N)
where
    N: Scalar,
{
    let mut iter = vector.iter();

    (*iter.next().unwrap(), *iter.next().unwrap())
}

fn translate_position(position: &Position) -> Isometry<PhysicsType> {
    Isometry::new(
        Vector::new(
            PhysicsType::from(position.location.x),
            PhysicsType::from(position.location.y),
        ),
        to_nphysics_rotation(position.rotation),
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
    match sensor_handle {
        _ if sensor_handle == first_handle => Some(second_handle),
        _ if sensor_handle == second_handle => Some(first_handle),
        _ => None,
    }
}

impl World for NphysicsWorld {
    fn step(&mut self) {
        for (&sensor_handle, collisions) in &mut self.sensor_collisions {
            for contact in self.physics_world.contact_events() {
                match *contact {
                    ContactEvent::Started(first_handle, second_handle) => {
                        if let Some(collision) =
                            collision_with_sensor(sensor_handle, first_handle, second_handle)
                        {
                            collisions.insert(collision);
                        }
                    }
                    ContactEvent::Stopped(first_handle, second_handle) => {
                        if let Some(collision) =
                            collision_with_sensor(sensor_handle, first_handle, second_handle)
                        {
                            collisions.remove(&collision);
                        }
                    }
                }
            }
        }
        self.physics_world.step();
    }

    fn add_body(&mut self, body: PhysicalBody) -> BodyHandle {
        let shape = translate_shape(&body.shape);
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let isometry = translate_position(&body.position);
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
        let position = translate_position(&sensor.position);
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
    use super::*;
    use crate::object_builder::PolygonBuilder;

    const DEFAULT_TIMESTEP: f64 = 1.0;

    #[test]
    fn returns_none_when_calling_body_with_invalid_handle() {
        let world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let invalid_handle = BodyHandle(1337);
        let body = world.body(invalid_handle);
        assert!(body.is_none())
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let movable_body = movable_body(Radians(3.0));

        let handle = world.add_body(movable_body);
        world.body(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let body = immovable_body(Radians(3.0));

        let handle = world.add_body(body);
        world.body(handle);
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let rigid_object = movable_body(Radians(3.0));
        let grounded_object = immovable_body(Radians(3.0));

        let rigid_handle = world.add_body(rigid_object);
        let grounded_handle = world.add_body(grounded_object);

        let _rigid_body = world.body(rigid_handle);
        let _grounded_body = world.body(grounded_handle);
    }

    #[test]
    fn returns_correct_rigid_body() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let expected_body = movable_body(Radians(3.0));
        let handle = world.add_body(expected_body.clone());
        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_correct_grounded_body() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let expected_body = immovable_body(Radians(3.0));
        let handle = world.add_body(expected_body.clone());
        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_sensor_handle_when_attachment_is_valid() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let body = immovable_body(Radians::default());
        let handle = world.add_body(body);
        let sensor_handle = world.attach_sensor(handle, sensor());
        assert!(sensor_handle.is_some())
    }

    #[test]
    fn sensors_do_not_work_without_step() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let body = immovable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 6, y: 6 },
                rotation: Radians::default(),
            },
            ..immovable_body(Radians::default())
        };
        world.add_body(close_body);

        let bodies = world
            .bodies_within_sensor(sensor_handle)
            .expect("sensor handle was invalid");

        assert!(bodies.is_empty());
    }

    #[test]
    fn sensor_detects_close_bodies() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let body = immovable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 6, y: 6 },
                rotation: Radians::default(),
            },
            ..immovable_body(Radians::default())
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
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let body = immovable_body(Radians::default());
        let handle_one = world.add_body(body);

        let sensor_handle = world
            .attach_sensor(handle_one, sensor())
            .expect("body handle was invalid");

        let close_body = PhysicalBody {
            position: Position {
                location: Location { x: 60, y: 60 },
                rotation: Radians::default(),
            },
            ..immovable_body(Radians::default())
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
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let invalid_handle = BodyHandle(132144);
        let sensor_handle = world.attach_sensor(invalid_handle, sensor());
        assert!(sensor_handle.is_none())
    }

    #[test]
    fn returns_none_when_calling_bodies_within_sensor_with_invalid_handle() {
        let world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let invalid_handle = SensorHandle(112358);
        let body_handles = world.bodies_within_sensor(invalid_handle);

        assert!(body_handles.is_none())
    }

    #[test]
    fn timestep_is_respected() {
        let mut world = NphysicsWorld::with_timestep(1.0);

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
        let mut world = NphysicsWorld::with_timestep(0.0);
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
        use std::f64::consts::FRAC_PI_2;

        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let expected_body = immovable_body(Radians(FRAC_PI_2));
        let handle = world.add_body(expected_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn step_is_ignored_for_grounded_objects() {
        use std::f64::consts::FRAC_PI_2;

        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
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
                .vertex(-5, -5)
                .vertex(5, -5)
                .vertex(5, 5)
                .vertex(-5, 5)
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
}
