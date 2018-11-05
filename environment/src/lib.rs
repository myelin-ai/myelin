//! This crate containes the physical environment of
//! the simulation, as well as the objects that reside
//! within it.

#![feature(specialization, non_exhaustive, box_syntax)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown,
    clippy::unimplemented
)]
#![cfg_attr(test, allow(clippy::float_cmp))]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate nameof;

pub mod object;
pub mod object_builder;
pub mod simulation_impl;

use crate::object::{ObjectBehavior, ObjectDescription};
use std::fmt;

/// A Simulation that can be filled with [`Object`] on
/// which it will apply physical rules when calling [`step`].
/// This trait represents our API.
///
/// [`Object`]: ./object/struct.Object.html
/// [`step`]: ./trait.Simulation.html#tymethod.step
pub trait Simulation: fmt::Debug {
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
    fn objects(&self) -> Vec<ObjectDescription>;
    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
}
