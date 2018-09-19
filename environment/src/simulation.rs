use crate::object::{Body, Kind, Object};
use std::fmt;

/// A world running a simulation that can be filled with [`Objects`] on
/// which it will apply physical rules when calling [`step`].
/// This trait represents our API.
///
/// [`Objects`]: ../object/struct.Body.html
/// [`step`]: ./trait.World.html#structfield.location#tymethod.step
pub trait Simulation {
    /// Advance the simulation by one tick. This will apply
    /// forces to the objects, handle collisions and move them.
    fn step(&mut self);
    /// Add a new object to the world.
    fn add_object_with_body(&mut self, object: Box<dyn Object>, body: Body);
    /// Returns all objects currently inhabiting the simulation.
    fn kinds_and_their_bodies(&self) -> Vec<(Kind, Body)>;
    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
}

#[derive(Debug)]
pub struct SimulationImpl {
    world: Box<dyn World>,
}

impl SimulationImpl {
    pub fn new(world: Box<dyn World>) -> Self {
        Self { world }
    }
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        unimplemented!()
    }
    fn add_object_with_body(&mut self, object: Box<dyn Object>, body: Body) {
        unimplemented!()
    }
    fn kinds_and_their_bodies(&self) -> Vec<(Kind, Body)> {
        unimplemented!()
    }
    fn set_simulated_timestep(&mut self, timestep: f64) {
        unimplemented!()
    }
}

pub trait World: fmt::Debug {
    fn step(&mut self);
    fn add_rigid_object(&mut self, object: Body) -> BodyHandle;
    fn add_grounded_object(&mut self, object: Body) -> BodyHandle;
    fn object(&self, handle: BodyHandle) -> Body;
    fn set_simulated_timestep(&mut self, timestep: f64);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BodyHandle(pub usize);
