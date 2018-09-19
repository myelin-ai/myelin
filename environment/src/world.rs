//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.Body.html
use crate::object::*;
use crate::simulation::simulation_impl::{BodyHandle, PhysicalBody, World};
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::{BodyHandle as NphysicsBodyHandle, Collider, ColliderHandle, Material};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::HashMap;
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
}

impl NphysicsWorld {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::world::NphysicsWorld;
    /// let mut world = NphysicsWorld::with_timestep(1.0);
    /// ```
    pub fn with_timestep(timestep: f64) -> Self {
        let mut physics_world = PhysicsWorld::new();

        physics_world.set_timestep(timestep);

        Self {
            physics_world,
            collider_handles: HashMap::new(),
        }
    }

    fn convert_to_object(&self, collider_handle: ColliderHandle) -> PhysicalBody {
        let collider = self
            .physics_world
            .collider(collider_handle)
            .expect("Collider handle was invalid");

        let shape = self.get_shape(&collider);
        let position = self.get_position(&collider);
        let velocity = self.get_velocity(&collider);

        PhysicalBody {
            shape,
            position,
            velocity,
        }
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

    fn get_velocity(&self, collider: &Collider<PhysicsType>) -> Mobility {
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

fn get_isometry(body: &PhysicalBody) -> Isometry<PhysicsType> {
    Isometry::new(
        Vector::new(
            PhysicsType::from(body.position.location.x),
            PhysicsType::from(body.position.location.y),
        ),
        to_nphysics_rotation(body.position.rotation),
    )
}

fn get_shape(body: &PhysicalBody) -> ShapeHandle<PhysicsType> {
    let points: Vec<_> = body
        .shape
        .vertices
        .iter()
        .map(|vertex| Point::new(PhysicsType::from(vertex.x), PhysicsType::from(vertex.y)))
        .collect();

    ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"))
}

impl World for NphysicsWorld {
    fn step(&mut self) {
        self.physics_world.step();
    }

    fn add_rigid_body(&mut self, body: PhysicalBody) -> BodyHandle {
        let shape = get_shape(&body);
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let isometry = get_isometry(&body);
        let rigid_body_handle =
            self.physics_world
                .add_rigid_body(isometry, local_inertia, local_center_of_mass);

        let material = Material::default();
        let collider_handle = self.physics_world.add_collider(
            0.04,
            shape,
            rigid_body_handle,
            Isometry::identity(),
            material,
        );
        to_object_handle(collider_handle)
    }

    fn add_grounded_body(&mut self, body: PhysicalBody) -> BodyHandle {
        let shape = get_shape(&body);
        let material = Material::default();
        let isometry = get_isometry(&body);

        let collider_handle = self.physics_world.add_collider(
            0.04,
            shape,
            NphysicsBodyHandle::ground(),
            isometry,
            material,
        );
        to_object_handle(collider_handle)
    }

    fn body(&self, handle: BodyHandle) -> PhysicalBody {
        let collider_handle = to_collider_handle(handle);
        self.convert_to_object(collider_handle)
    }

    fn set_simulated_timestep(&mut self, timestep: f64) {
        self.physics_world.set_timestep(timestep);
    }
}

fn to_object_handle(collider_handle: ColliderHandle) -> BodyHandle {
    BodyHandle(collider_handle.uid())
}

fn to_collider_handle(object_handle: BodyHandle) -> ColliderHandle {
    CollisionObjectHandle(object_handle.0)
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
    use crate::object_builder::{ObjectBuilder, PolygonBuilder};

    const DEFAULT_TIMESTEP: f64 = 1.0;

    fn local_rigid_object(orientation: Radians) -> PhysicalBody {
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-10, -10)
                    .vertex(10, -10)
                    .vertex(10, 10)
                    .vertex(-10, 10)
                    .build()
                    .unwrap(),
            ).location(30, 40)
            .orientation(orientation)
            .build()
            .unwrap()
    }

    fn local_grounded_object(orientation: Radians) -> PhysicalBody {
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-100, -100)
                    .vertex(100, -100)
                    .vertex(100, 100)
                    .vertex(-100, 100)
                    .build()
                    .unwrap(),
            ).location(300, 400)
            .orientation(orientation)
            .build()
            .unwrap()
    }

    #[should_panic]
    #[test]
    fn panics_on_invalid_handle() {
        let world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let body = world.body(BodyHandle(1337));
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let local_rigid_object = local_rigid_object(Radians(3.0));

        let handle = world.add_rigid_body(local_rigid_object);
        let _body = world.body(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_grounded_object(Radians(3.0));

        let handle = world.add_grounded_body(object.clone());
        let _body = world.body(handle);
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let rigid_object = local_rigid_object(Radians(3.0));
        let grounded_object = local_grounded_object(Radians(3.0));

        let rigid_handle = world.add_rigid_body(rigid_object);
        let grounded_handle = world.add_grounded_body(grounded_object);

        let _rigid_body = world.body(rigid_handle);
        let _grounded_body = world.body(grounded_handle);
    }

    #[test]
    fn converting_to_global_object_works_with_orientation() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_rigid_object(Radians(3.0));
        let handle = world.add_rigid_body(object);

        let expected_global_object = PhysicalBody {
            shape: Polygon {
                vertices: vec![
                    Vertex {
                        x: 20 - 1,
                        y: 30 + 2,
                    },
                    Vertex {
                        x: 40 - 2,
                        y: 30 - 1,
                    },
                    Vertex {
                        x: 40 + 1,
                        y: 50 - 2,
                    },
                    Vertex {
                        x: 20 + 2,
                        y: 50 + 1,
                    },
                ],
            },
            orientation: Radians(3.0),
            velocity: Velocity::default(),
        };

        let body = world.body(handle);
        assert_eq!(expected_global_object, object)
    }

    #[test]
    fn converting_to_global_rigid_object_works_without_orientation() {
        let object = local_rigid_object(Default::default());
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let handle = world.add_rigid_body(object);

        let expected_global_object = Body {
            orientation: Default::default(),
            shape: Polygon {
                vertices: vec![
                    Vertex { x: 40, y: 50 },
                    Vertex { x: 20, y: 50 },
                    Vertex { x: 20, y: 30 },
                    Vertex { x: 40, y: 30 },
                ],
            },
            velocity: Velocity::default(),
        };

        let body = world.body(handle);
        assert_eq!(expected_global_object, object)
    }

    #[test]
    fn converting_to_global_grounded_object_works_without_orientation() {
        let object = local_grounded_object(Default::default());
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let handle = world.add_rigid_body(object);

        let expected_global_object = PhysicalBody {
            orientation: Default::default(),
            shape: Polygon {
                vertices: vec![
                    Vertex { x: 400, y: 500 },
                    Vertex { x: 200, y: 500 },
                    Vertex { x: 200, y: 300 },
                    Vertex { x: 400, y: 300 },
                ],
            },
            velocity: Velocity::default(),
        };

        let body = world.body(handle);
        assert_eq!(expected_global_object, object)
    }

    #[test]
    fn converting_to_global_object_works_with_pi_orientation() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let orientation = Radians(1.5 * PI);
        let object = local_rigid_object(orientation);
        let handle = world.add_rigid_body(object);

        let expected_global_object = PhysicalBody {
            orientation,
            shape: Polygon {
                vertices: vec![
                    Vertex { x: 40, y: 30 },
                    Vertex { x: 40, y: 50 },
                    Vertex { x: 20, y: 50 },
                    Vertex { x: 20, y: 30 },
                ],
            },
            velocity: Velocity::default(),
        };

        let body = world.body(handle);
        assert_eq!(expected_global_object, object)
    }

    #[ignore]
    #[test]
    fn timestep_is_respected() {
        let mut world = NphysicsWorld::with_timestep(1.0);

        let local_object = ObjectBuilder::new()
            .location(5, 5)
            .shape(
                PolygonBuilder::new()
                    .vertex(-5, -5)
                    .vertex(-5, 5)
                    .vertex(5, 5)
                    .vertex(5, -5)
                    .build()
                    .unwrap(),
            ).build()
            .unwrap();
        let handle = world.add_rigid_body(local_object);

        world.step();
        world.step();

        let body = world.body(handle);
        assert_eq!(
            vec![
                Vertex { x: 11, y: 11 },
                Vertex { x: 11, y: 1 },
                Vertex { x: 1, y: 1 },
                Vertex { x: 1, y: 11 },
            ],
            body.shape.vertices
        );
    }

    /// Can be reactivated when https://github.com/myelin-ai/myelin/issues/92
    /// has been resolved
    #[ignore]
    #[test]
    fn timestep_can_be_changed() {
        let mut world = NphysicsWorld::with_timestep(0.0);

        world.set_simulated_timestep(1.0);

        let local_object = ObjectBuilder::new()
            .location(5, 5)
            .shape(
                PolygonBuilder::new()
                    .vertex(-5, -5)
                    .vertex(-5, 5)
                    .vertex(5, 5)
                    .vertex(5, -5)
                    .build()
                    .unwrap(),
            ).build()
            .unwrap();
        let handle = world.add_rigid_body(local_object);

        world.step();
        world.step();

        let body = world.body(handle);
        assert_eq!(
            vec![
                Vertex { x: 11, y: 11 },
                Vertex { x: 11, y: 1 },
                Vertex { x: 1, y: 1 },
                Vertex { x: 1, y: 11 },
            ],
            body.shape.vertices
        );
    }

    /// Can be reactivated when https://github.com/myelin-ai/myelin/issues/92
    /// has been resolved
    #[ignore]
    #[test]
    fn step_is_ignored_for_grounded_objects() {
        use std::f64::consts::FRAC_PI_2;

        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_grounded_object(Radians(FRAC_PI_2));
        let handle = world.add_grounded_body(object);
        world.step();

        let expected_global_object = PhysicalBody {
            shape: Polygon {
                vertices: vec![
                    Vertex { x: 200, y: 500 },
                    Vertex { x: 200, y: 300 },
                    Vertex { x: 400, y: 300 },
                    Vertex { x: 400, y: 500 },
                ],
            },
            orientation: Radians(FRAC_PI_2),
            velocity: Velocity { x: 0, y: 0 },
        };

        let body = world.body(handle);
        assert_eq!(expected_global_object, object)
    }
}
