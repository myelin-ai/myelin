use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::math::{Isometry, Vector};
use nphysics2d::object::BodyHandle;
use nphysics2d::object::Material;
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;

use nalgebra as na;

use crate::object::Object;

pub trait World {
    fn step(&mut self);
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

    fn objects(&self) -> Vec<Object> {
        vec![]
    }
}
