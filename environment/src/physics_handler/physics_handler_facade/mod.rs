use super::PhysicsHandler;
use crate::collision_detector::CollisionIter;
use crate::properties::Object;

use std::fmt::Debug;
mod collision_handler_impl;
mod movement_applier_impl;

#[derive(Debug)]
pub struct PhysicsHandlerFacade {
    movement_applier: Box<dyn MovementApplier>,
    collision_handler: Box<dyn CollisionHandler>,
}

impl PhysicsHandlerFacade {
    pub fn new(
        movement_applier: Box<dyn MovementApplier>,
        collision_handler: Box<dyn CollisionHandler>,
    ) -> Self {
        Self {
            movement_applier,
            collision_handler,
        }
    }
}

impl PhysicsHandler for PhysicsHandlerFacade {
    fn handle_collisions<'a>(&self, collisions: Box<CollisionIter<'a>>, container: &mut [Object]) {
        self.collision_handler
            .handle_collisions(collisions, container)
    }
    fn apply_movement(&self, container: &mut [Object]) {
        self.movement_applier.apply_movement(container)
    }
}

pub trait MovementApplier: Debug {
    fn apply_movement(&self, container: &mut [Object]);
}

pub trait CollisionHandler: Debug {
    fn handle_collisions<'a>(&self, _collisions: Box<CollisionIter<'a>>, _container: &mut [Object]);
}
