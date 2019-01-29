//! Definitions for the underlying [`World`] used internally by [`Simulation`]

mod nphysics_world;

pub use self::nphysics_world::*;
use crate::object::*;
use myelin_geometry::*;
use std::fmt::Debug;

/// A container for [`PhysicalBodies`] that will apply
/// physical laws to them on [`step`]
///
/// [`PhysicalBodies`]: ./struct.PhysicalBody.html
/// [`step`]: ./trait.World.html#tymethod.step
#[cfg_attr(test, mockiato::mockable)]
pub trait World: Debug {
    /// Advance the simulation by one tick. This will apply
    /// forces to the objects and handle collisions;
    fn step(&mut self);
    /// Place a [`PhysicalBody`] in the world. Returns a
    /// unique [`BodyHandle`] that can be passed to [`body()`]
    /// in order to retrieve the [`PhysicalBody`] again
    ///
    /// [`PhysicalBody`]: ./struct.PhysicalBody.html
    /// [`BodyHandle`]: ./struct.BodyHandle.html
    /// [`body()`]: ./trait.World.html#tymethod.body
    fn add_body(&mut self, body: PhysicalBody) -> BodyHandle;

    /// Removes a previously added [`PhysicalBody`] from the world.
    /// If `body_handle` was valid, this will return the removed physical body.
    ///
    /// [`PhysicalBody`]: ./struct.PhysicalBody.html
    fn remove_body(&mut self, body_handle: BodyHandle) -> Option<PhysicalBody>;

    /// Returns a [`PhysicalBody`] that has previously been
    /// placed with [`add_body()`] by its [`BodyHandle`].
    ///
    /// # Errors
    /// Returns `None` if the [`BodyHandle`] did not correspond
    /// to any [`PhysicalBody`]
    ///
    /// [`PhysicalBody`]: ./struct.PhysicalBody.html
    /// [`BodyHandle`]: ./struct.BodyHandle.html
    /// [`add_body()`]: ./trait.World.html#tymethod.add_body
    fn body(&self, handle: BodyHandle) -> Option<PhysicalBody>;

    /// Register a force that will be applied to a body on the next
    /// step.
    /// # Errors
    /// Returns `None` if `body_handle` did not match any sensors.
    fn apply_force(&mut self, body_handle: BodyHandle, force: Force) -> Option<()>;

    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);

    /// Checks if the given [`BodyHandle`] is marked passable
    ///
    /// [`BodyHandle`]: ./struct.BodyHandle.html
    fn is_body_passable(&self, body_handle: BodyHandle) -> bool;

    /// Returns all bodies either completely contained or intersecting
    /// with the area.
    ///
    /// [`Aabb`]: ./struct.Aabb.html
    fn bodies_in_area(&self, area: Aabb) -> Vec<BodyHandle>;
}

/// The pure physical representation of an object
/// that can be placed within a [`World`]
///
/// [`World`]: trait.World.html
#[derive(Debug, PartialEq, Clone)]
pub struct PhysicalBody {
    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.Body.html#structfield.position
    pub shape: Polygon,
    /// The current global position of the center of the body
    pub location: Point,
    /// The body's rotation
    pub rotation: Radians,
    /// The current mobility of the object. If present,
    /// this is defined as a two dimensional vector relative to the
    /// objects center
    pub mobility: Mobility,
    /// Whether this object is passable or not
    pub passable: bool,
}

/// A unique identifier that can be used to retrieve a [`PhysicalBody`] from a
/// [`World`].
///
/// Don't construct any of these by yourself, only use the
/// instances that [`World`] provides you
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BodyHandle(pub usize);
