use crate::object::{ObjectBehavior, ObjectDescription, Polygon, Position};

pub mod simulation_impl;

/// A Simulation that can be filled with [`Object`] on
/// which it will apply physical rules when calling [`step`].
/// This trait represents our API.
///
/// [`Object`]: ./struct.Object.html
/// [`step`]: ./trait.World.html#structfield.location#tymethod.step
pub trait Simulation {
    /// Advance the simulation by one tick. This will apply
    /// forces to the objects, handle collisions and allow them to
    /// take action.
    fn step(&mut self);
    /// Add a new object to the world.
    fn add_object(&mut self, object: Object);
    /// Returns a read-only description of all objects currently inhabiting the simulation.
    fn objects(&self) -> Vec<ObjectDescription>;
    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
}

/// A new object that is going to be placed in the simulation
#[derive(Debug)]
pub struct Object {
    /// The object's behavior, which determines its kind and what the object is going to do every step
    pub object_behavior: ObjectBehavior,
    /// The object's initial position
    pub position: Position,
    /// The object's shape
    pub shape: Polygon,
}
