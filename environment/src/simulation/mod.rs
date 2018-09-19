use crate::object::{Object, ObjectDescription, Position};

pub mod simulation_impl;

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
    fn add_object_at(&mut self, object: Object, position: Position);
    /// Returns all objects currently inhabiting the simulation.
    fn objects(&self) -> ObjectDescription;
    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
}
