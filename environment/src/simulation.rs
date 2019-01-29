//! Entrypoint for the Simulaton API handling the world itself

mod simulation_impl;

pub use self::simulation_impl::*;
use crate::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

/// A Simulation that can be filled with [`Object`] on
/// which it will apply physical rules when calling [`step`].
/// This trait represents our API.
///
/// [`Object`]: ./object/struct.Object.html
/// [`step`]: ./trait.Simulation.html#tymethod.step
#[cfg_attr(any(test, feature = "use-mocks"), mockiato::mockable)]
pub trait Simulation: Debug {
    /// Advance the simulation by one tick. This will apply
    /// forces to the objects, handle collisions and allow them to
    /// take action.
    fn step(&mut self);
    /// Add a new object to the world.
    fn add_object(
        &mut self,
        object_description: ObjectDescription,
        object_behavior: Box<dyn ObjectBehavior>,
    );
    /// Returns a read-only description of all objects currently inhabiting the simulation.
    fn objects(&self) -> Snapshot;
    /// Returns read-only descriptions for all objects either completely
    /// contained or intersecting with the given area.
    fn objects_in_area(&self, area: Aabb) -> Snapshot;
    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
}

/// Unique identifier of an Object
pub type Id = usize;

/// A representation of the current state of the simulation
pub type Snapshot = HashMap<Id, ObjectDescription>;
