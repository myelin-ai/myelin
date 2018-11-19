//! Objects that can be placed in a world and their components.
//! You can construct a [`ObjectDescription`] by using an [`ObjectBuilder`].
//!
//! [`ObjectBuilder`]: crate::object_builder::ObjectBuilder

pub use crate::object_builder::*;
use crate::{Id, Snapshot};
use myelin_geometry::*;
use std::fmt::Debug;

/// Behavior of an object
pub trait ObjectBehavior: Debug + ObjectBehaviorClone {
    /// Returns all actions performed by the object
    /// in the current simulation tick
    fn step(
        &mut self,
        own_description: &ObjectDescription,
        environment: &dyn ObjectEnvironment,
    ) -> Option<Action>;
}

/// Provides information to an [`ObjectBehavior`] about
/// the world it is placed in.
///
/// [`ObjectBehavior`]: ./trait.ObjectBehavior.html
pub trait ObjectEnvironment: Debug {
    /// Scans for objects in the area defined by an [`Aabb`].
    ///
    /// Returns all objects either completely contained or intersecting
    /// with the area.
    ///
    /// [`Aabb`]: ./struct.Aabb.html
    fn find_objects_in_area(&self, area: Aabb) -> Snapshot;
}

/// Possible actions performed by an [`Object`]
/// during a simulation step
///
/// [`Object`]: ./trait.Object.html
#[derive(Debug)]
pub enum Action {
    /// Apply the specified force to the object
    ApplyForce(Force),
    /// Create a new object at the specified location
    Reproduce(ObjectDescription, Box<dyn ObjectBehavior>),
    /// Destroys another object
    Destroy(Id),
    /// Destroy the object
    Die,
}

impl Clone for Action {
    fn clone(&self) -> Action {
        match self {
            Action::ApplyForce(force) => Action::ApplyForce(force.clone()),
            Action::Reproduce(object_description, object_behavior) => {
                Action::Reproduce(object_description.clone(), object_behavior.clone_box())
            }
            Action::Destroy(body_handle) => Action::Destroy(*body_handle),
            Action::Die => Action::Die,
        }
    }
}

/// The behaviourless description of an object that has
/// been placed inside a [`Simulation`].
///
/// [`Simulation`]: ../simulation/trait.Simulation.html
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectDescription {
    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Polygon,

    /// The global location of the center of the object
    pub location: Point,

    /// The object's rotation
    pub rotation: Radians,

    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Mobility,

    /// The object's kind
    pub kind: Kind,

    /// Whether the object is passable or not
    pub passable: bool,
}

/// An object's mobility and, if present, its
/// current velocity as a vector
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Mobility {
    /// The object cannot have any velocity as
    /// it cannot be moved. Corresponds to [`ImmovableObject`]
    ///
    /// [`ImmovableObject`]: ./trait.ImmovableObject.html
    Immovable,
    /// A movable object's current velocity. Corresponds to [`MovableObject`]
    ///
    /// [`MovableObject`]: ./trait.MovableObject.html
    Movable(Vector),
}

/// The part of an object that is responsible for custom
/// behavior and interactions
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Kind {
    /// An intelligent organism featuring a neural network
    Organism,
    /// A self-spreading plant, ready for consumption
    Plant,
    /// A stationary body of water
    Water,
    /// Impassable terrain
    Terrain,
}

/// Combination of a linear force and its torque,
/// resulting in a rotated force applied to an object
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Force {
    /// The linear component of the [`Force`]
    pub linear: Vector,
    /// The torque (rotation) component of the [`Force`]
    pub torque: Torque,
}

/// Force of rotation
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Torque(pub f64);

/// Supertrait used to make sure that all implementors
/// of [`ObjectBehavior`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`ObjectBehavior`]: ./trait.ObjectBehavior.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait ObjectBehaviorClone {
    fn clone_box(&self) -> Box<dyn ObjectBehavior>;
}

impl<T> ObjectBehaviorClone for T
where
    T: ObjectBehavior + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn ObjectBehavior> {
        box self.clone()
    }
}

#[cfg(any(test, feature = "use-mocks"))]
pub use self::mock::*;

#[cfg(any(test, feature = "use-mocks"))]
mod mock {
    use super::*;
    use std::cell::RefCell;
    use std::thread::panicking;

    /// Mock [`ObjectBehavior`]
    ///
    /// [`ObjectBehavior`]: ./trait.ObjectBehavior.html
    #[derive(Debug, Default, Clone)]
    pub struct ObjectBehaviorMock {
        expect_step_and_return: Option<(ObjectDescription, Option<Action>)>,
        step_was_called: RefCell<bool>,
    }

    impl ObjectBehaviorMock {
        /// Creates a new [`ObjectBehaviorMock`] without any expectations set.
        ///
        /// [`ObjectBehaviorMock`]: ./struct.ObjectBehaviorMock.html
        pub fn new() -> ObjectBehaviorMock {
            Default::default()
        }

        /// Marks the method [`ObjectBehavior::step`] as expected
        pub fn expect_step_and_return(
            &mut self,
            own_description: ObjectDescription,
            returned_value: Option<Action>,
        ) {
            self.expect_step_and_return = Some((own_description, returned_value));
        }
    }

    impl ObjectBehavior for ObjectBehaviorMock {
        fn step(
            &mut self,
            own_description: &ObjectDescription,
            _environment: &dyn ObjectEnvironment,
        ) -> Option<Action> {
            *self.step_was_called.borrow_mut() = true;
            if let Some((ref expected_own_description, ref return_value)) =
                self.expect_step_and_return
            {
                if expected_own_description == own_description {
                    return_value.clone()
                } else {
                    panic!(
                        "step() was called with {:?}, expected {:?}",
                        own_description, expected_own_description
                    )
                }
            } else {
                panic!("step() was called unexpectedly")
            }
        }
    }

    impl Drop for ObjectBehaviorMock {
        fn drop(&mut self) {
            if !panicking() && self.expect_step_and_return.is_some() {
                assert!(
                    *self.step_was_called.borrow(),
                    "step() was not called, but was expected"
                )
            }
        }
    }

    /// Mock for [`ObjectEnvironment`]
    ///
    /// [`ObjectEnvironment`]: ./trait.ObjectEnvironment.html
    #[derive(Debug, Default, Clone)]
    pub struct ObjectEnvironmentMock {
        expect_find_objects_in_area_and_return: Vec<(Aabb, Snapshot)>,
        find_objects_in_area_was_called: RefCell<bool>,
    }

    impl ObjectEnvironmentMock {
        /// Creates a new [`ObjectEnvironmentMock`] without any expectations set.
        ///
        /// [`ObjectEnvironmentMock`]: ./struct.ObjectEnvironmentMock.html
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds an expected method call for  [`ObjectEnvironment::find_objects_in_area`].
        /// This can be called multiple times to expect multiple calls.
        /// Note that for each `area` there may be only one `return_value`.
        pub fn expect_find_objects_in_area(&mut self, area: Aabb, return_value: Snapshot) {
            self.expect_find_objects_in_area_and_return
                .push((area, return_value));
        }
    }

    impl ObjectEnvironment for ObjectEnvironmentMock {
        fn find_objects_in_area(&self, area: Aabb) -> Snapshot {
            if let Some((_, ref return_value)) = self
                .expect_find_objects_in_area_and_return
                .iter()
                .find(|(expected_area, _)| *expected_area == area)
            {
                *self.find_objects_in_area_was_called.borrow_mut() = true;

                return_value.clone()
            } else {
                panic!(
                    "find_objects_in_area() was called with unexpected area: {:?}",
                    area
                );
            }
        }
    }

    impl Drop for ObjectEnvironmentMock {
        fn drop(&mut self) {
            if !panicking() && !self.expect_find_objects_in_area_and_return.is_empty() {
                assert!(
                    *self.find_objects_in_area_was_called.borrow(),
                    "find_objects_in_area() was expected, but never called"
                );
            }
        }
    }
}
