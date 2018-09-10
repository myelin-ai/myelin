use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::BodyHandle;
use nphysics2d::object::Material;
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;

use nalgebra as na;

use crate::object::Object;

pub trait World {
    fn step(&mut self);
    fn add_object(&mut self, object: Object);
    fn objects(&self) -> Vec<Object>;
}

#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct WorldImpl {
    physics_world: PhysicsWorld<f64>,
    bodies: Vec<BodyHandle>,
}

impl WorldImpl {
    pub fn new() -> Self {
        Self {
            physics_world: PhysicsWorld::new(),
            bodies: Vec::new(),
        }
    }
}

impl World for WorldImpl {
    fn step(&mut self) {
        self.physics_world.step();
        for body in &self.bodies {
            let rigid_body = self
                .physics_world
                .rigid_body(*body)
                .expect("Attempted to access invalid rigid body handle");
            print!("{}", rigid_body.position());
        }
    }

    fn add_object(&mut self, object: Object) {
        let points: Vec<_> = object
            .body
            .vertices
            .iter()
            .map(|vertex| Point::new(vertex.x as f64, vertex.y as f64))
            .collect();

        let shape =
            ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"));
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let rigid_body_handle = self.physics_world.add_rigid_body(
            Isometry::new(
                Vector::new(object.location.x as f64, object.location.y as f64),
                na::zero(),
            ),
            local_inertia,
            local_center_of_mass,
        );

        let material = Material::default();
        let _collider_handle = self.physics_world.add_collider(
            0.04,
            shape,
            rigid_body_handle,
            Isometry::identity(),
            material,
        );
    }

    fn objects(&self) -> Vec<Object> {
        vec![]
    }
}
