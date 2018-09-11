use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::ColliderHandle;
use nphysics2d::object::Material;
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::fmt;

use nalgebra as na;

use crate::object::{GlobalObject, LocalObject};

pub trait World: fmt::Debug {
    fn step(&mut self);
    fn add_object(&mut self, object: LocalObject);
    fn objects(&self) -> Vec<GlobalObject>;
}

#[derive(Default)]
pub struct WorldImpl {
    physics_world: PhysicsWorld<f64>,
    collider_handles: Vec<ColliderHandle>,
}

impl WorldImpl {
    pub fn new() -> Self {
        Self {
            physics_world: PhysicsWorld::new(),
            collider_handles: Vec::new(),
        }
    }
}

impl World for WorldImpl {
    fn step(&mut self) {
        self.physics_world.step();

        for collider_handle in &self.collider_handles {
            let collider = self
                .physics_world
                .collider(*collider_handle)
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
            .map(|vertex| Point::new(f64::from(vertex.x), f64::from(vertex.y)))
            .collect();

        let shape =
            ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"));
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let rigid_body_handle = self.physics_world.add_rigid_body(
            Isometry::new(
                Vector::new(f64::from(object.location.x), f64::from(object.location.y)),
                na::zero(),
            ),
            local_inertia,
            local_center_of_mass,
        );

        let material = Material::default();
        let collider_handle = self.physics_world.add_collider(
            0.04,
            shape,
            rigid_body_handle,
            Isometry::identity(),
            material,
        );

        self.collider_handles.push(collider_handle);
    }

    fn objects(&self) -> Vec<GlobalObject> {
        unimplemented!()
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

pub struct DebugPhysicsWorld<'a>(&'a PhysicsWorld<f64>);

impl<'a> fmt::Debug for DebugPhysicsWorld<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicsWorld").finish()
    }
}
