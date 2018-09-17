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
use nphysics2d::object::{BodyHandle, Collider, ColliderHandle, Material};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt;

/// A world running a simulation that can be filled with [`Objects`] on
/// which it will apply physical rules when calling [`step`].
/// This trait represents our API.
///
/// [`Objects`]: ../object/struct.LocalObject.html
/// [`step`]: ./trait.World.html#structfield.location#tymethod.step
pub trait World: fmt::Debug {
    /// Advance the simulation by one tick. This will apply
    /// forces to the objects, handle collisions and move them.
    fn step(&mut self);
    /// Add a new object to the world.
    fn add_object(&mut self, object: LocalObject);
    /// Returns all objects currently inhabiting the simulation.
    fn objects(&self) -> Vec<GlobalObject>;
    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
}

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
                x: vertex.x.round() as u32,
                y: vertex.y.round() as u32,
            }).collect();

        let velocity = self.get_velocity(&collider, kind);

        GlobalObject {
            shape: GlobalPolygon {
                vertices: global_vertices,
            },
            orientation: to_orientation(position_isometry.rotation.angle()),
            velocity,
            kind: kind.clone(),
        }
    }

    fn get_velocity(&self, collider: &Collider<PhysicsType>, kind: &Kind) -> Velocity {
        let (x, y) = if is_grounded(kind) {
            (0.0, 0.0)
        } else {
            let body_handle = collider.data().body();
            let body = self
                .physics_world
                .rigid_body(body_handle)
                .expect("Body handle was invalid");

            let linear_velocity = body.velocity().linear;
            elements(&linear_velocity)
        };
        Velocity {
            x: x as i32,
            y: y as i32,
        }
    }

    fn add_grounded(&mut self, object: &LocalObject) -> ColliderHandle {
        let shape = get_shape(object);
        let material = Material::default();
        let isometry = get_isometry(&object);

        self.physics_world
            .add_collider(0.04, shape, BodyHandle::ground(), isometry, material)
    }

    fn add_rigid_body(&mut self, object: &LocalObject) -> ColliderHandle {
        let shape = get_shape(object);
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let isometry = get_isometry(&object);
        let rigid_body_handle =
            self.physics_world
                .add_rigid_body(isometry, local_inertia, local_center_of_mass);

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
        self.physics_world.add_collider(
            0.04,
            shape,
            rigid_body_handle,
            Isometry::identity(),
            material,
        )
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

fn get_isometry(object: &LocalObject) -> Isometry<PhysicsType> {
    Isometry::new(
        Vector::new(
            PhysicsType::from(object.location.x),
            PhysicsType::from(object.location.y),
        ),
        to_nphysics_rotation(object.orientation),
    )
}

fn get_shape(object: &LocalObject) -> ShapeHandle<PhysicsType> {
    let points: Vec<_> = object
        .shape
        .vertices
        .iter()
        .map(|vertex| Point::new(PhysicsType::from(vertex.x), PhysicsType::from(vertex.y)))
        .collect();

    ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"))
}

fn is_grounded(kind: &Kind) -> bool {
    match kind {
        Kind::Terrain => true,
        Kind::Water => true,
        Kind::Organism => false,
        Kind::Plant => false,
    }
}

impl World for NphysicsWorld {
    fn step(&mut self) {
        self.physics_world.step();
    }

    fn add_object(&mut self, object: LocalObject) {
        let collider_handle = if is_grounded(&object.kind) {
            self.add_grounded(&object)
        } else {
            self.add_rigid_body(&object)
        };
        self.collider_handles.insert(collider_handle, object.kind);
    }

    fn objects(&self) -> Vec<GlobalObject> {
        self.collider_handles
            .iter()
            .map(|(&handle, kind)| self.convert_to_object(handle, kind))
            .collect()
    }

    fn set_simulated_timestep(&mut self, timestep: f64) {
        self.physics_world.set_timestep(timestep);
    }
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

    fn local_rigid_object(orientation: Radians) -> LocalObject {
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
            .velocity(4, 5)
            .kind(Kind::Organism)
            .build()
            .unwrap()
    }

    fn local_grounded_object(orientation: Radians) -> LocalObject {
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
            .kind(Kind::Terrain)
            .build()
            .unwrap()
    }

    #[test]
    fn returns_empty_world() {
        let world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let objects = world.objects();
        assert!(objects.is_empty())
    }

    #[test]
    fn returns_empty_world_after_step() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        world.step();
        let objects = world.objects();
        assert!(objects.is_empty())
    }

    #[test]
    fn returns_correct_number_of_rigid_objects() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let local_rigid_object = local_rigid_object(Radians(3.0));
        world.add_object(local_rigid_object.clone());
        world.add_object(local_rigid_object);

        let objects = world.objects();
        assert_eq!(2, objects.len());
    }

    #[test]
    fn returns_correct_number_of_grounded_objects() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_grounded_object(Radians(3.0));
        world.add_object(object.clone());
        world.add_object(object);

        let objects = world.objects();
        assert_eq!(2, objects.len());
    }

    #[test]
    fn returns_correct_number_of_mixed_objects() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let rigid_object = local_rigid_object(Radians(3.0));
        let grounded_object = local_grounded_object(Radians(3.0));
        world.add_object(grounded_object.clone());
        world.add_object(grounded_object);
        world.add_object(rigid_object.clone());
        world.add_object(rigid_object);

        let objects = world.objects();
        assert_eq!(4, objects.len());
    }

    #[test]
    fn converts_to_global_object_works_with_orientation() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_rigid_object(Radians(3.0));
        world.add_object(object);
        let objects = world.objects();

        let expected_global_object = GlobalObject {
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
            velocity: Velocity { x: 4, y: 5 },
            kind: Kind::Organism,
        };
        assert_eq!(expected_global_object, objects[0])
    }

    #[test]
    fn converts_to_global_rigid_object_works_without_orientation() {
        let object = local_rigid_object(Default::default());
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        world.add_object(object);
        let objects = world.objects();

        let expected_global_object = GlobalObject {
            orientation: Default::default(),
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 40, y: 50 },
                    GlobalVertex { x: 20, y: 50 },
                    GlobalVertex { x: 20, y: 30 },
                    GlobalVertex { x: 40, y: 30 },
                ],
            },
            velocity: Velocity { x: 4, y: 5 },
            kind: Kind::Organism,
        };
        assert_eq!(expected_global_object, objects[0])
    }

    #[test]
    fn converts_to_global_grounded_object_works_without_orientation() {
        let object = local_grounded_object(Default::default());
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        world.add_object(object);
        let objects = world.objects();

        let expected_global_object = GlobalObject {
            orientation: Default::default(),
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 400, y: 500 },
                    GlobalVertex { x: 200, y: 500 },
                    GlobalVertex { x: 200, y: 300 },
                    GlobalVertex { x: 400, y: 300 },
                ],
            },
            velocity: Velocity { x: 0, y: 0 },
            kind: Kind::Terrain,
        };
        assert_eq!(expected_global_object, objects[0])
    }

    #[test]
    fn converts_to_global_object_works_with_pi_orientation() {
        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let orientation = Radians(1.5 * PI);
        let object = local_rigid_object(orientation);
        world.add_object(object);
        let objects = world.objects();

        let expected_global_object = GlobalObject {
            orientation,
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 40, y: 30 },
                    GlobalVertex { x: 40, y: 50 },
                    GlobalVertex { x: 20, y: 50 },
                    GlobalVertex { x: 20, y: 30 },
                ],
            },
            velocity: Velocity { x: 4, y: 5 },
            kind: Kind::Organism,
        };
        assert_eq!(expected_global_object, objects[0])
    }

    #[test]
    fn timestep_is_respected() {
        let mut world = NphysicsWorld::with_timestep(1.0);

        let local_object = ObjectBuilder::new()
            .kind(Kind::Organism)
            .velocity(1, 1)
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
        world.add_object(local_object);

        world.step();
        world.step();

        assert_eq!(
            vec![
                GlobalVertex { x: 11, y: 11 },
                GlobalVertex { x: 11, y: 1 },
                GlobalVertex { x: 1, y: 1 },
                GlobalVertex { x: 1, y: 11 },
            ],
            world.objects().first().unwrap().shape.vertices
        );
    }

    #[test]
    fn timestep_can_be_changed() {
        let mut world = NphysicsWorld::with_timestep(0.0);

        world.set_simulated_timestep(1.0);

        let local_object = ObjectBuilder::new()
            .kind(Kind::Organism)
            .velocity(1, 1)
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
        world.add_object(local_object);

        world.step();
        world.step();

        assert_eq!(
            vec![
                GlobalVertex { x: 11, y: 11 },
                GlobalVertex { x: 11, y: 1 },
                GlobalVertex { x: 1, y: 1 },
                GlobalVertex { x: 1, y: 11 },
            ],
            world.objects().first().unwrap().shape.vertices
        );
    }

    #[test]
    fn step_is_ignored_for_grounded_objects() {
        use std::f64::consts::FRAC_PI_2;

        let mut world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP);
        let object = local_grounded_object(Radians(FRAC_PI_2));
        world.add_object(object);
        world.step();
        let objects = world.objects();

        let expected_global_object = GlobalObject {
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
            kind: Kind::Terrain,
        };
        assert_eq!(expected_global_object, objects[0])
    }
}
