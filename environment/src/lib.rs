//! This crate containes the physical environment of
//! the simulation, as well as the objects that reside
//! within it.

#![feature(specialization, non_exhaustive, box_syntax)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]
#![cfg_attr(test, allow(clippy::float_cmp))]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate nameof;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate maplit;

pub mod object;
mod object_builder;
pub mod simulation_impl;

use crate::object::{Aabb, ObjectBehavior, ObjectDescription};
use std::collections::HashMap;
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

#[cfg(any(test, feature = "use-mocks"))]
pub use self::mock::*;

#[cfg(any(test, feature = "use-mocks"))]
#[allow(clippy::float_cmp)]
mod mock {
    use super::*;
    use crate::object::Action;
    use std::cell::RefCell;
    use std::thread::panicking;

    /// Mock for [`Simulation`]
    #[derive(Debug, Default, Clone)]
    pub struct SimulationMock {
        expect_step: Option<()>,
        expect_add_object: Option<ObjectDescription>,
        expect_objects_and_return: Option<Snapshot>,
        expect_set_simulated_timestep: Option<f64>,
        expect_objects_in_area_and_return: Option<(Aabb, Snapshot)>,

        step_was_called: RefCell<bool>,
        add_object_was_called: RefCell<bool>,
        objects_was_called: RefCell<bool>,
        set_simulated_timestep_was_called: RefCell<bool>,
        objects_in_area_was_called: RefCell<bool>,
    }

    impl SimulationMock {
        /// Marks the method [`Simulation::step`] as expected.
        pub fn expect_step(&mut self) {
            self.expect_step = Some(());
        }

        /// Marks the method [`Simulation::add_object`] as expected.
        pub fn expect_add_object(&mut self, object_description: ObjectDescription) {
            self.expect_add_object = Some(object_description)
        }

        /// Marks the method [`Simulation::objects`] as expected.
        pub fn expect_objects_and_return(&mut self, return_value: Snapshot) {
            self.expect_objects_and_return = Some(return_value)
        }

        /// Marks the method [`Simulation::objects_in_area`] as expected.
        pub fn expect_objects_in_area_and_return(&mut self, area: Aabb, return_value: Snapshot) {
            self.expect_objects_in_area_and_return = Some((area, return_value));
        }

        /// Marks the method [`Simulation::set_simulated_timestep`] as expected.
        pub fn expect_set_simulated_timestep(&mut self, timestep: f64) {
            self.expect_set_simulated_timestep = Some(timestep)
        }
    }

    impl Simulation for SimulationMock {
        fn step(&mut self) {
            *self.step_was_called.borrow_mut() = true;

            if self.expect_step.is_none() {
                panic!("step() was called unexpectedly")
            }
        }
        fn add_object(
            &mut self,
            object_description: ObjectDescription,
            _object_behavior: Box<dyn ObjectBehavior>,
        ) {
            *self.add_object_was_called.borrow_mut() = true;

            if let Some(ref expected_object_description) = self.expect_add_object {
                if object_description != *expected_object_description {
                    panic!(
                        "add_object() was called with {:?}, expected {:?} ",
                        object_description, expected_object_description,
                    )
                }
            } else {
                panic!("add_object() was called unexpectedly")
            }
        }
        fn objects(&self) -> Snapshot {
            *self.objects_was_called.borrow_mut() = true;

            if let Some(ref return_value) = self.expect_objects_and_return {
                return_value.clone()
            } else {
                panic!("objects() was called unexpectedly")
            }
        }

        fn objects_in_area(&self, area: Aabb) -> Snapshot {
            *self.objects_in_area_was_called.borrow_mut() = true;

            if let Some((expected_area, ref return_value)) = self.expect_objects_in_area_and_return
            {
                assert_eq!(
                    expected_area, area,
                    "objects_in_area() was called with {:?}, expected {:?}",
                    expected_area, area
                );
                return_value.clone()
            } else {
                panic!("objects_in_area() was called unexpectedly");
            }
        }

        fn set_simulated_timestep(&mut self, timestep: f64) {
            *self.set_simulated_timestep_was_called.borrow_mut() = true;

            if let Some(ref expected_timestep) = self.expect_set_simulated_timestep {
                if timestep != *expected_timestep {
                    panic!(
                        "set_simulated_timestep() was called with {:?}, expected {:?} ",
                        timestep, expected_timestep,
                    )
                }
            } else {
                panic!("set_simulated_timestep() was called unexpectedly")
            }
        }
    }

    impl Drop for SimulationMock {
        fn drop(&mut self) {
            if !panicking() {
                if self.expect_step.is_some() {
                    assert!(
                        *self.step_was_called.borrow(),
                        "step() was not called, but expected"
                    )
                }
                if self.expect_add_object.is_some() {
                    assert!(
                        *self.add_object_was_called.borrow(),
                        "add_object() was not called, but expected"
                    )
                }
                if self.expect_objects_and_return.is_some() {
                    assert!(
                        *self.objects_was_called.borrow(),
                        "objects() was not called, but expected"
                    )
                }
                if self.expect_objects_in_area_and_return.is_some() {
                    assert!(
                        *self.objects_in_area_was_called.borrow(),
                        "objects() was not called, but expected"
                    )
                }
                if self.expect_set_simulated_timestep.is_some() {
                    assert!(
                        *self.set_simulated_timestep_was_called.borrow(),
                        "set_simulated_timestep() was not called, but expected"
                    )
                }
            }
        }
    }
}
