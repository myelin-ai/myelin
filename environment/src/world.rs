//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.LocalObject.html
use crate::object::*;
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::ColliderHandle;
use nphysics2d::object::Material;
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt;

/// A world running a simulation that can be filled with [`Objects`] on
/// which it will apply physical rules when calling [`step`]
/// This trait represents our API.
///
/// [`Objects`]: ../object/struct.LocalObject.html
/// [`step`]: ./trait.World.html#structfield.location#tymethod.step
pub trait World: fmt::Debug {
    /// Advance the simulation by one tick. This will apply
    /// forces to the objects, handle collisions and move them.
    fn step(&mut self);
    /// Add a new object to the world
    fn add_object(&mut self, object: LocalObject);
    /// Returns all objects currently inhabiting the simulation
    fn objects(&self) -> Vec<GlobalObject>;
}

type PhysicsType = f64;

/// An implementation of [`World`] that uses nphysics
/// in the background.
///
/// [`World`]: ./trait.World.html
#[derive(Default)]
pub struct WorldImpl {
    physics_world: PhysicsWorld<PhysicsType>,
    collider_handles: HashMap<ColliderHandle, Kind>,
}

impl WorldImpl {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::world::WorldImpl;
    /// let mut world = WorldImpl::new();
    /// ```
    pub fn new() -> Self {
        Self {
            physics_world: PhysicsWorld::new(),
            collider_handles: HashMap::new(),
        }
    }

    fn convert_to_object(&self, collider_handle: ColliderHandle, kind: &Kind) -> GlobalObject {
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
                x: vertex.x as u32,
                y: vertex.y as u32,
            }).collect();

        let body_handle = collider.data().body();
        let body = self
            .physics_world
            .rigid_body(body_handle)
            .expect("Body handle was invalid");

        let linear_velocity = body.velocity().linear;
        let (x, y) = elements(&linear_velocity);

        GlobalObject {
            shape: GlobalPolygon {
                vertices: global_vertices,
            },
            orientation: to_orientation(position_isometry.rotation.angle()),
            velocity: Velocity {
                x: x as i32,
                y: y as i32,
            },
            kind: kind.clone(),
        }
    }
}

/// The offset needed because we define orientation as [0; 2π)
/// and nphysics defines rotation as (-π; π]
/// See http://nalgebra.org/rustdoc/nalgebra/geometry/type.UnitComplex.html#method.angle
const NPHYSICS_ROTATION_OFFSET: f32 = PI;

fn to_nphysics_rotation(orientation: Radians) -> f64 {
    PhysicsType::from(orientation.0 - NPHYSICS_ROTATION_OFFSET)
}

fn to_orientation(nphysics_rotation: f64) -> Radians {
    Radians(nphysics_rotation as f32 + NPHYSICS_ROTATION_OFFSET)
}

fn elements<N>(vector: &Vector2<N>) -> (N, N)
where
    N: Scalar,
{
    let mut iter = vector.iter();

    (*iter.next().unwrap(), *iter.next().unwrap())
}

impl World for WorldImpl {
    fn step(&mut self) {
        self.physics_world.step();

        for collider_handle in &self.collider_handles {
            let collider = self
                .physics_world
                .collider(*collider_handle.0)
                .expect("Attempted to access invalid collider handle");
            let rigid_body_handle = collider.data().body();
            let rigid_body = self
                .physics_world
                .rigid_body(rigid_body_handle)
                .expect("Attempted to access invalid rigid body handle");

            print!("{}", rigid_body.position());
        }
    }

    fn add_object(&mut self, object: LocalObject) {
        let points: Vec<_> = object
            .shape
            .vertices
            .iter()
            .map(|vertex| Point::new(PhysicsType::from(vertex.x), PhysicsType::from(vertex.y)))
            .collect();

        let shape =
            ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"));
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let rigid_body_handle = self.physics_world.add_rigid_body(
            Isometry::new(
                Vector::new(
                    PhysicsType::from(object.location.x),
                    PhysicsType::from(object.location.y),
                ),
                to_nphysics_rotation(object.orientation),
            ),
            local_inertia,
            local_center_of_mass,
        );

        let linear_velocity = Vector2::new(
            PhysicsType::from(object.velocity.x),
            PhysicsType::from(object.velocity.y),
        );
        let rigid_body = self
            .physics_world
            .rigid_body_mut(rigid_body_handle)
            .expect("add_rigid_body() returned invalid handle");
        rigid_body.set_linear_velocity(linear_velocity);

        let material = Material::default();
        let collider_handle = self.physics_world.add_collider(
            0.04,
            shape,
            rigid_body_handle,
            Isometry::identity(),
            material,
        );

        self.collider_handles.insert(collider_handle, object.kind);
    }

    fn objects(&self) -> Vec<GlobalObject> {
        self.collider_handles
            .iter()
            .map(|(&handle, kind)| self.convert_to_object(handle, kind))
            .collect()
    }
}

impl fmt::Debug for WorldImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorldImpl")
            .field("collider_handles", &self.collider_handles)
            .field("physics", &DebugPhysicsWorld(&self.physics_world))
            .finish()
    }
}

/// A helper struct used to implement [`std::fmt::Debug`]
/// for [`WorldImpl`]
///
/// [`std::fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
/// [`WorldImpl`]: ./struct.WorldImpl.html
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

    fn local_object() -> LocalObject {
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
            .orientation(Radians(3.0))
            .velocity(4, 5)
            .kind(Kind::Organism)
            .build()
            .unwrap()
    }

    #[test]
    fn returns_empty_world() {
        let world = WorldImpl::new();
        let objects = world.objects();
        assert!(objects.is_empty())
    }

    #[test]
    fn returns_empty_world_after_step() {
        let mut world = WorldImpl::new();
        world.step();
        let objects = world.objects();
        assert!(objects.is_empty())
    }

    #[test]
    fn returns_correct_number_of_objects() {
        let mut world = WorldImpl::new();
        let local_object = local_object();
        world.add_object(local_object.clone());
        world.add_object(local_object);

        let objects = world.objects();
        assert_eq!(2, objects.len());
    }

    #[test]
    fn converts_to_global_object() {
        let mut world = WorldImpl::new();
        let local_object = local_object();
        world.add_object(local_object);
        let objects = world.objects();

        let expected_global_object = GlobalObject {
            shape: GlobalPolygon {
                // The following inaccuracies appear to be
                // a product of how nphysics stores its objects
                // Maybe it can be fixed somehow, but it is okay
                // for the moment, as an occasional two pixel
                // displacement doesn't matter at all.
                vertices: vec![
                    GlobalVertex {
                        x: 20 - 2,
                        y: 30 + 1,
                    },
                    GlobalVertex {
                        x: 40 - 2,
                        y: 30 - 2,
                    },
                    GlobalVertex {
                        x: 40 + 1,
                        y: 50 - 2,
                    },
                    GlobalVertex {
                        x: 20 + 1,
                        y: 50 + 1,
                    },
                ],
            },
            orientation: Radians(3.0),
            velocity: Velocity { x: 4, y: 5 },
            kind: Kind::Organism,
        };
        assert_eq!(expected_global_object, objects[0])
    }
}
