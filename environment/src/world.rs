use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::math::{Isometry, Vector};
use nphysics2d::object::BodyHandle;
use nphysics2d::object::Material;
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;

use nalgebra as na;

#[allow(missing_debug_implementations)]
pub struct World {
    physics_world: PhysicsWorld<f64>,
    bodies: Vec<BodyHandle>,
}

impl World {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new();
        let mut bodies = Vec::new();
        let cuboid = ShapeHandle::new(Cuboid::new(Vector::new(1.0, 2.0)));
        let local_inertia = cuboid.inertia(0.1);
        let local_center_of_mass = cuboid.center_of_mass();
        let rigid_body_handle = physics_world.add_rigid_body(
            Isometry::new(Vector::new(2.0, 10.0), na::zero()),
            local_inertia,
            local_center_of_mass,
        );
        bodies.push(rigid_body_handle);

        let material = Material::default();
        let _collider_handle = physics_world.add_collider(
            0.04,
            cuboid,
            rigid_body_handle,
            Isometry::identity(),
            material,
        );
        Self {
            physics_world,
            bodies,
        }
    }

    pub fn step(&mut self) {
        self.physics_world.step();
        for body in &self.bodies {
            let rigid_body = self
                .physics_world
                .rigid_body(*body)
                .expect("Attempted to access invalid rigid body handle");
            print!("{}", rigid_body.position());
        }
    }
}
