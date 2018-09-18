//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.LocalBody.html
use crate::object::*;
use crate::simulation::{ObjectHandle, World};
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::{BodyHandle, Collider, ColliderHandle, Material};
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

    fn convert_to_object(&self, collider_handle: ColliderHandle) -> GlobalBody {
        let collider = self
            .physics_world
            .collider(collider_handle)
            .expect("Collider handle was invalid");
        let convex_polygon: &ConvexPolygon<_> = collider
            .shape()
            .as_shape()
            .expect("Failed to cast shape to a ConvexPolygon");
        let position_isometry = collider.position();
        let global_vertices: Vec<_> = convex_polygon
            .points()
            .iter()
            .map(|vertex| position_isometry * vertex)
            .map(|vertex| GlobalVertex {
                x: vertex.x.round() as u32,
                y: vertex.y.round() as u32,
            }).collect();

        let velocity = self.get_velocity(&collider);

        GlobalBody {
            shape: GlobalPolygon {
                vertices: global_vertices,
            },
            orientation: to_orientation(position_isometry.rotation.angle()),
            velocity,
        }
    }

    fn get_velocity(&self, collider: &Collider<PhysicsType>) -> Velocity {
        let body_handle = collider.data().body();
        let (x, y) = if body_handle.is_ground() {
            (0.0, 0.0)
        } else {
            let rigid_body = self
                .physics_world
                .rigid_body(body_handle)
                .expect("Body handle did not correspond to any rigid body");

            let linear_velocity = rigid_body.velocity().linear;
            elements(&linear_velocity)
        };
        Velocity {
            x: x as i32,
            y: y as i32,
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

fn to_orientation(nphysics_rotation: f64) -> Radians {
    Radians(nphysics_rotation + NPHYSICS_ROTATION_OFFSET)
}

fn elements<N>(vector: &Vector2<N>) -> (N, N)
where
    N: Scalar,
{
    let mut iter = vector.iter();

    (*iter.next().unwrap(), *iter.next().unwrap())
}

fn get_isometry(object: &LocalBody) -> Isometry<PhysicsType> {
    Isometry::new(
        Vector::new(
            PhysicsType::from(object.location.x),
            PhysicsType::from(object.location.y),
        ),
        to_nphysics_rotation(object.orientation),
    )
}

fn get_shape(object: &LocalBody) -> ShapeHandle<PhysicsType> {
    let points: Vec<_> = object
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

    fn add_rigid_object(&mut self, object: LocalBody) -> ObjectHandle {
        let shape = get_shape(&object);
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let isometry = get_isometry(&object);
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

    fn add_grounded_object(&mut self, object: LocalBody) -> ObjectHandle {
        let shape = get_shape(&object);
        let material = Material::default();
        let isometry = get_isometry(&object);

        let collider_handle =
            self.physics_world
                .add_collider(0.04, shape, BodyHandle::ground(), isometry, material);
        to_object_handle(collider_handle)
    }

    fn object(&self, handle: ObjectHandle) -> GlobalBody {
        let collider_handle = to_collider_handle(handle);
        self.convert_to_object(collider_handle)
    }

    fn set_simulated_timestep(&mut self, timestep: f64) {
        self.physics_world.set_timestep(timestep);
    }
}

fn to_object_handle(collider_handle: ColliderHandle) -> ObjectHandle {
    ObjectHandle(collider_handle.uid())
}

fn to_collider_handle(object_handle: ObjectHandle) -> ColliderHandle {
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

    fn local_rigid_object(orientation: Radians) -> LocalBody {
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

    fn local_grounded_object(orientation: Radians) -> LocalBody {
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
        let object = world.object(ObjectHandle(1337));
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let local_rigid_object = local_rigid_object(Radians(3.0));

        let handle = world.add_rigid_object(local_rigid_object);
        let _object = world.object(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_grounded_object(Radians(3.0));

        let handle = world.add_grounded_object(object.clone());
        let _object = world.object(handle);
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let rigid_object = local_rigid_object(Radians(3.0));
        let grounded_object = local_grounded_object(Radians(3.0));

        let rigid_handle = world.add_rigid_object(rigid_object);
        let grounded_handle = world.add_grounded_object(grounded_object);

        let _rigid_object = world.object(rigid_handle);
        let _grounded_object = world.object(grounded_handle);
    }

    #[test]
    fn converting_to_global_object_works_with_orientation() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_rigid_object(Radians(3.0));
        let handle = world.add_rigid_object(object);

        let expected_global_object = GlobalBody {
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex {
                        x: 20 - 1,
                        y: 30 + 2,
                    },
                    GlobalVertex {
                        x: 40 - 2,
                        y: 30 - 1,
                    },
                    GlobalVertex {
                        x: 40 + 1,
                        y: 50 - 2,
                    },
                    GlobalVertex {
                        x: 20 + 2,
                        y: 50 + 1,
                    },
                ],
            },
            orientation: Radians(3.0),
            velocity: Velocity::default(),
        };

        let object = world.object(handle);
        assert_eq!(expected_global_object, object)
    }

    #[test]
    fn converting_to_global_rigid_object_works_without_orientation() {
        let object = local_rigid_object(Default::default());
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let handle = world.add_rigid_object(object);

        let expected_global_object = GlobalBody {
            orientation: Default::default(),
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 40, y: 50 },
                    GlobalVertex { x: 20, y: 50 },
                    GlobalVertex { x: 20, y: 30 },
                    GlobalVertex { x: 40, y: 30 },
                ],
            },
            velocity: Velocity::default(),
        };

        let object = world.object(handle);
        assert_eq!(expected_global_object, object)
    }

    #[test]
    fn converting_to_global_grounded_object_works_without_orientation() {
        let object = local_grounded_object(Default::default());
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let handle = world.add_rigid_object(object);

        let expected_global_object = GlobalBody {
            orientation: Default::default(),
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 400, y: 500 },
                    GlobalVertex { x: 200, y: 500 },
                    GlobalVertex { x: 200, y: 300 },
                    GlobalVertex { x: 400, y: 300 },
                ],
            },
            velocity: Velocity::default(),
        };

        let object = world.object(handle);
        assert_eq!(expected_global_object, object)
    }

    #[test]
    fn converting_to_global_object_works_with_pi_orientation() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let orientation = Radians(1.5 * PI);
        let object = local_rigid_object(orientation);
        let handle = world.add_rigid_object(object);

        let expected_global_object = GlobalBody {
            orientation,
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 40, y: 30 },
                    GlobalVertex { x: 40, y: 50 },
                    GlobalVertex { x: 20, y: 50 },
                    GlobalVertex { x: 20, y: 30 },
                ],
            },
            velocity: Velocity::default(),
        };

        let object = world.object(handle);
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
        let handle = world.add_rigid_object(local_object);

        world.step();
        world.step();

        let object = world.object(handle);
        assert_eq!(
            vec![
                GlobalVertex { x: 11, y: 11 },
                GlobalVertex { x: 11, y: 1 },
                GlobalVertex { x: 1, y: 1 },
                GlobalVertex { x: 1, y: 11 },
            ],
            object.shape.vertices
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
        let handle = world.add_rigid_object(local_object);

        world.step();
        world.step();

        let object = world.object(handle);
        assert_eq!(
            vec![
                GlobalVertex { x: 11, y: 11 },
                GlobalVertex { x: 11, y: 1 },
                GlobalVertex { x: 1, y: 1 },
                GlobalVertex { x: 1, y: 11 },
            ],
            object.shape.vertices
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
        let handle = world.add_grounded_object(object);
        world.step();

        let expected_global_object = GlobalBody {
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 200, y: 500 },
                    GlobalVertex { x: 200, y: 300 },
                    GlobalVertex { x: 400, y: 300 },
                    GlobalVertex { x: 400, y: 500 },
                ],
            },
            orientation: Radians(FRAC_PI_2),
            velocity: Velocity { x: 0, y: 0 },
        };

        let object = world.object(handle);
        assert_eq!(expected_global_object, object)
    }
}
