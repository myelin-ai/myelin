use crate::object::*;

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

/// An objects that can be placed in the world.
/// Its coordinates are relative to the center of
/// the object, which is determined by the [`location`]
///
/// [`location`]: ./struct.Body.html#structfield.location
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectDescription {
    /// The vertices defining the shape of the object
    /// in relation to its [`location`]
    ///
    /// [`location`]: ./struct.Body.html#structfield.location
    pub shape: Polygon,
    pub position: Position,
    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub velocity: Mobility,
    pub kind: Kind,
}
