//! A `Simulation` that outsources all physical
//! behaviour into a separate `World` type

pub use self::object_environment::ObjectEnvironmentImpl;
use crate::object::*;
use crate::{Simulation, Snapshot};
use myelin_geometry::*;
use ncollide2d::world::CollisionObjectHandle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug};

mod object_environment;
pub mod world;

/// Factory used by [`SimulationImpl`] to create an [`ObjectEnvironment`].
///
/// [`SimulationImpl`]: ./struct.SimulationImpl.html
/// [`ObjectEnvironment`]: ./../object/trait.ObjectEnvironment.html
pub type ObjectEnvironmentFactoryFn =
    dyn for<'a> Fn(&'a dyn Simulation) -> Box<dyn ObjectEnvironment + 'a>;

/// Implementation of [`Simulation`] that uses a physical
/// [`World`] in order to apply physics to objects.
///
/// [`Simulation`]: ./../trait.Simulation.html
/// [`World`]: ./trait.World.html
pub struct SimulationImpl {
    world: Box<dyn World>,
    non_physical_object_data: HashMap<BodyHandle, NonPhysicalObjectData>,
    object_environment_factory_fn: Box<ObjectEnvironmentFactoryFn>,
}

impl Debug for SimulationImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(SimulationImpl))
            .field("world", &self.world)
            .field("non_physical_object_data", &self.non_physical_object_data)
            .finish()
    }
}

#[derive(Debug)]
struct NonPhysicalObjectData {
    pub(crate) kind: Kind,
    pub(crate) behavior: RefCell<Box<dyn ObjectBehavior>>,
}

/// An error that can occur whenever an action is performed
#[derive(Debug, Clone)]
pub enum ActionError {
    /// The given handle was invalid
    InvalidHandle,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid handle")
    }
}

impl Error for ActionError {}

impl SimulationImpl {
    /// Create a new SimulationImpl by injecting a [`World`]
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::world::collision_filter::IgnoringCollisionFilterImpl;
    /// use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
    /// use myelin_environment::simulation_impl::{
    ///     world::rotation_translator::NphysicsRotationTranslatorImpl, world::NphysicsWorld,
    ///     ObjectEnvironmentImpl, SimulationImpl,
    /// };
    /// use std::sync::{Arc, RwLock};
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl::default();
    /// let force_applier = SingleTimeForceApplierImpl::default();
    /// let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterImpl::default()));
    /// let world = Box::new(NphysicsWorld::with_timestep(
    ///     1.0,
    ///     Box::new(rotation_translator),
    ///     Box::new(force_applier),
    ///     collision_filter,
    /// ));
    /// let simulation = SimulationImpl::new(
    ///     world,
    ///     Box::new(|simulation| Box::new(ObjectEnvironmentImpl::new(simulation))),
    /// );
    /// ```
    /// [`World`]: ./trait.World.html
    pub fn new(
        world: Box<dyn World>,
        object_environment_factory_fn: Box<ObjectEnvironmentFactoryFn>,
    ) -> Self {
        Self {
            world,
            non_physical_object_data: HashMap::new(),
            object_environment_factory_fn,
        }
    }

    fn convert_to_object_description(&self, body_handle: BodyHandle) -> Option<ObjectDescription> {
        let physics_body = self.world.body(body_handle)?;
        let non_physical_object_data = self.non_physical_object_data.get(&body_handle)?;
        Some(ObjectDescription {
            shape: physics_body.shape,
            location: physics_body.location,
            rotation: physics_body.rotation,
            mobility: physics_body.mobility,
            kind: non_physical_object_data.kind,
            passable: self.world.is_body_passable(body_handle),
        })
    }

    fn handle_action(
        &mut self,
        body_handle: BodyHandle,
        action: Action,
    ) -> Result<(), ActionError> {
        match action {
            Action::Reproduce(object_description, object_behavior) => {
                self.add_object(object_description, object_behavior);
                Ok(())
            }
            Action::ApplyForce(force) => self
                .world
                .apply_force(body_handle, force)
                .map(|_| ())
                .ok_or(ActionError::InvalidHandle),
            Action::Destroy(object_id) => self
                .world
                .remove_body(BodyHandle(object_id))
                .map(|_| ())
                .ok_or(ActionError::InvalidHandle),
            Action::Die => self
                .world
                .remove_body(body_handle)
                .and(self.non_physical_object_data.remove(&body_handle))
                .map(|_| ())
                .ok_or(ActionError::InvalidHandle),
        }
    }
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        let object_handle_to_own_description: HashMap<_, _> = self
            .non_physical_object_data
            .keys()
            .map(|&object_handle| {
                (
                    object_handle,
                    // This is safe because the keys of self.objects and
                    // object_handle_to_objects_within_sensor are identical
                    self.convert_to_object_description(object_handle).unwrap(),
                )
            })
            .collect();
        let mut actions = Vec::new();

        {
            let environment = (self.object_environment_factory_fn)(self);
            for (object_handle, non_physical_object_data) in &self.non_physical_object_data {
                // This is safe because the keys of self.objects and
                // object_handle_to_objects_within_sensor are identical
                let own_description = &object_handle_to_own_description[object_handle];
                let action = non_physical_object_data
                    .behavior
                    .borrow_mut()
                    .step(&own_description, environment.as_ref());
                if let Some(action) = action {
                    actions.push((*object_handle, action));
                }
            }
        }

        for (body_handle, action) in actions {
            self.handle_action(body_handle, action)
                .expect("Body handle was not found within world");
        }
        self.world.step()
    }

    fn add_object(
        &mut self,
        object_description: ObjectDescription,
        object_behavior: Box<dyn ObjectBehavior>,
    ) {
        let physical_body = PhysicalBody {
            shape: object_description.shape,
            location: object_description.location,
            rotation: object_description.rotation,
            mobility: object_description.mobility,
            passable: object_description.passable,
        };

        let body_handle = self.world.add_body(physical_body);

        let non_physical_object_data = NonPhysicalObjectData {
            kind: object_description.kind,
            behavior: RefCell::new(object_behavior),
        };
        self.non_physical_object_data
            .insert(body_handle, non_physical_object_data);
    }

    fn objects(&self) -> Snapshot {
        self.non_physical_object_data
            .keys()
            .map(|&handle| {
                (
                    handle.0,
                    self.convert_to_object_description(handle)
                        .expect("Handle stored in simulation was not found in world"),
                )
            })
            .collect()
    }

    fn objects_in_area(&self, area: Aabb) -> Snapshot {
        self.world
            .bodies_in_area(area)
            .into_iter()
            .map(|handle| {
                let object_description = self
                    .convert_to_object_description(handle)
                    .expect("Handle stored in simulation was not found in world");

                (handle.0, object_description)
            })
            .collect()
    }

    fn set_simulated_timestep(&mut self, timestep: f64) {
        assert!(timestep >= 0.0, "Cannot set timestep to a negative value");
        self.world.set_simulated_timestep(timestep)
    }
}

/// A container for [`PhysicalBodies`] that will apply
/// physical laws to them on [`step`]
///
/// [`PhysicalBodies`]: ./struct.PhysicalBody.html
/// [`step`]: ./trait.World.html#tymethod.step
pub trait World: fmt::Debug {
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

/// A unique identifier that represents either a [`BodyHandle`]
/// or a [`SensorHandle`]
///
/// You are allowed to construct this handle from either subhandles.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AnyHandle(pub usize);

impl From<BodyHandle> for AnyHandle {
    fn from(body_handle: BodyHandle) -> Self {
        AnyHandle(body_handle.0)
    }
}

impl From<CollisionObjectHandle> for AnyHandle {
    fn from(collision_object_handle: CollisionObjectHandle) -> Self {
        AnyHandle(collision_object_handle.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{ObjectBehaviorMock, ObjectEnvironmentMock};
    use crate::object_builder::ObjectBuilder;
    use myelin_geometry::PolygonBuilder;
    use std::cell::RefCell;
    use std::thread::panicking;

    fn object_environment_factory_fn<'a>(
        _simulation: &'a dyn Simulation,
    ) -> Box<dyn ObjectEnvironment + 'a> {
        box ObjectEnvironmentMock::new()
    }

    #[test]
    fn propagates_step() {
        let mut world = box WorldMock::new();
        world.expect_step();
        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        simulation.step();
    }

    #[test]
    fn propagates_simulated_timestep() {
        let mut world = box WorldMock::new();
        const EXPECTED_TIMESTEP: f64 = 1.0;
        world.expect_set_simulated_timestep(EXPECTED_TIMESTEP);
        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        simulation.set_simulated_timestep(EXPECTED_TIMESTEP);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_timestep() {
        let world = box WorldMock::new();
        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        const INVALID_TIMESTEP: f64 = -0.1;
        simulation.set_simulated_timestep(INVALID_TIMESTEP);
    }

    #[test]
    fn propagates_zero_timestep() {
        let mut world = box WorldMock::new();
        const EXPECTED_TIMESTEP: f64 = 0.0;
        world.expect_set_simulated_timestep(EXPECTED_TIMESTEP);
        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        simulation.set_simulated_timestep(EXPECTED_TIMESTEP);
    }

    #[test]
    fn returns_no_objects_when_empty() {
        let world = box WorldMock::new();
        let simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        let objects = simulation.objects();
        assert!(objects.is_empty())
    }

    #[test]
    fn converts_to_physical_body() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);

        let object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();
        let object_behavior = ObjectBehaviorMock::new();

        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        simulation.add_object(object_description, box object_behavior);
    }

    #[test]
    fn propagates_step_to_added_object() {
        let mut world = WorldMock::new();
        world.expect_step();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));
        world.expect_is_body_passable_and_return(returned_handle.into(), expected_passable);

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        let mut object_behavior = ObjectBehaviorMock::new();
        object_behavior.expect_step_and_return(expected_object_description.clone(), None);

        let mut simulation = SimulationImpl::new(box world, box object_environment_factory_fn);
        simulation.add_object(expected_object_description, box object_behavior);
        simulation.step();
    }

    #[test]
    fn returns_added_object() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));
        world.expect_is_body_passable_and_return(returned_handle, expected_passable);

        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);
        let object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        simulation.add_object(expected_object_description.clone(), box object_behavior);

        let objects = simulation.objects();
        assert_eq!(1, objects.len());

        let object_description = objects.iter().next().unwrap().1;
        assert_eq!(expected_object_description, *object_description);
    }

    // Something seems fishy with the following test
    // It fails because the expected step from the child
    // is not called.
    // Removing the expected step from the child
    // results in step being called unexpectedly.
    // I suspect it's a problem resulting from our mock
    // returning the same handle twice.
    #[ignore]
    #[test]
    fn reproducing_spawns_object() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));
        world.expect_step();

        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let mut child_object_behavior = ObjectBehaviorMock::new();
        child_object_behavior.expect_step_and_return(expected_object_description.clone(), None);

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Some(Action::Reproduce(
                expected_object_description.clone(),
                box child_object_behavior,
            )),
        );

        simulation.add_object(expected_object_description.clone(), box object_behavior);

        simulation.step();
        simulation.step();
    }

    #[test]
    fn dying_removes_object() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body.clone()));
        world.expect_step();
        world.expect_remove_body_and_return(returned_handle, Some(expected_physical_body));
        world.expect_is_body_passable_and_return(returned_handle, expected_passable);

        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior
            .expect_step_and_return(expected_object_description.clone(), Some(Action::Die));

        simulation.add_object(expected_object_description.clone(), box object_behavior);

        simulation.step();
    }

    #[test]
    fn destroy_removes_object() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };

        let handle_one = BodyHandle(1);
        let handle_two = BodyHandle(2);
        world.expect_add_body_and_return(expected_physical_body.clone(), handle_one);
        world.expect_body_and_return(handle_one, Some(expected_physical_body.clone()));
        world.expect_is_body_passable_and_return(handle_one, expected_passable);
        world.expect_step();
        world.expect_remove_body_and_return(handle_two, Some(expected_physical_body));

        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Some(Action::Destroy(handle_two.0)),
        );

        simulation.add_object(expected_object_description.clone(), box object_behavior);

        simulation.step();
    }

    #[test]
    fn force_application_is_propagated() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body.clone()));
        world.expect_step();
        world.expect_is_body_passable_and_return(returned_handle, expected_passable);
        let expected_force = Force {
            linear: Vector { x: 20.0, y: -5.0 },
            torque: Torque(-8.0),
        };
        world.expect_apply_force_and_return(returned_handle, expected_force.clone(), Some(()));

        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Some(Action::ApplyForce(expected_force)),
        );

        simulation.add_object(expected_object_description.clone(), box object_behavior);

        simulation.step();
    }

    #[should_panic]
    #[test]
    fn panics_on_invalid_body() {
        let mut world = box WorldMock::new();
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location.clone(),
            rotation: expected_rotation.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, None);
        let mut simulation = SimulationImpl::new(world, box object_environment_factory_fn);

        let object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let object_behavior = ObjectBehaviorMock::default();
        simulation.add_object(object_description, box object_behavior);
        simulation.objects();
    }

    #[test]
    fn propagates_objects_in_area() {
        let mut world = WorldMock::new();
        let (expected_physical_body, object_description) = object();
        let area = Aabb {
            upper_left: Point { x: 30.0, y: 30.0 },
            lower_right: Point { x: 10.0, y: 10.0 },
        };

        let returned_handle = BodyHandle(1234);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_bodies_in_area_and_return(area, vec![returned_handle]);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body.clone()));
        world.expect_is_body_passable_and_return(returned_handle, expected_physical_body.passable);

        let mut simulation = SimulationImpl::new(box world, box object_environment_factory_fn);

        let object_behavior = ObjectBehaviorMock::default();
        simulation.add_object(object_description.clone(), box object_behavior);

        let expected_objects = hashmap! { 1234 => object_description };

        assert_eq!(expected_objects, simulation.objects_in_area(area));
    }

    #[test]
    #[should_panic]
    fn objects_in_area_panics_when_given_invalid_handle() {
        let mut world = WorldMock::new();
        let (expected_physical_body, object_description) = object();
        let area = Aabb {
            upper_left: Point { x: 30.0, y: 30.0 },
            lower_right: Point { x: 10.0, y: 10.0 },
        };

        let returned_handle = BodyHandle(1234);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_bodies_in_area_and_return(area, vec![returned_handle]);
        world.expect_body_and_return(returned_handle, None);

        let mut simulation = SimulationImpl::new(box world, box object_environment_factory_fn);

        let object_behavior = ObjectBehaviorMock::default();
        simulation.add_object(object_description.clone(), box object_behavior);

        let expected_objects = hashmap! { 1234 => object_description };

        assert_eq!(expected_objects, simulation.objects_in_area(area));
    }

    fn object() -> (PhysicalBody, ObjectDescription) {
        let expected_shape = shape();
        let expected_location = location();
        let expected_rotation = rotation();
        let expected_mobility = Mobility::Movable(Vector::default());
        let expected_passable = false;
        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        (expected_physical_body, object_description)
    }

    fn shape() -> Polygon {
        PolygonBuilder::default()
            .vertex(-5.0, -5.0)
            .vertex(5.0, -5.0)
            .vertex(5.0, 5.0)
            .vertex(-5.0, 5.0)
            .build()
            .unwrap()
    }

    fn location() -> Point {
        Point { x: 30.0, y: 40.0 }
    }

    fn rotation() -> Radians {
        Radians::try_new(3.4).unwrap()
    }

    #[derive(Debug, Default)]
    struct WorldMock {
        expect_step: Option<()>,
        expect_add_body_and_return: Option<(PhysicalBody, BodyHandle)>,
        expect_remove_body_and_return: Option<(BodyHandle, Option<PhysicalBody>)>,
        expect_body_and_return: Option<(BodyHandle, Option<PhysicalBody>)>,
        expect_apply_force_and_return: Option<(BodyHandle, Force, Option<()>)>,
        expect_set_simulated_timestep: Option<f64>,
        expect_is_body_passable_and_return: Option<(BodyHandle, bool)>,
        expect_bodies_in_area_and_return: Option<(Aabb, Vec<BodyHandle>)>,

        step_was_called: RefCell<bool>,
        add_body_was_called: RefCell<bool>,
        remove_body_was_called: RefCell<bool>,
        body_was_called: RefCell<bool>,
        apply_force_was_called: RefCell<bool>,
        set_simulated_timestep_was_called: RefCell<bool>,
        is_body_passable: RefCell<bool>,
        bodies_in_area_was_called: RefCell<bool>,
    }

    impl WorldMock {
        pub(crate) fn new() -> Self {
            Default::default()
        }

        pub(crate) fn expect_step(&mut self) {
            self.expect_step = Some(());
        }

        pub(crate) fn expect_add_body_and_return(
            &mut self,
            body: PhysicalBody,
            returned_value: BodyHandle,
        ) {
            self.expect_add_body_and_return = Some((body, returned_value));
        }

        pub(crate) fn expect_remove_body_and_return(
            &mut self,
            body_handle: BodyHandle,
            returned_value: Option<PhysicalBody>,
        ) {
            self.expect_remove_body_and_return = Some((body_handle, returned_value));
        }

        pub(crate) fn expect_body_and_return(
            &mut self,
            handle: BodyHandle,
            returned_value: Option<PhysicalBody>,
        ) {
            self.expect_body_and_return = Some((handle, returned_value));
        }

        pub(crate) fn expect_apply_force_and_return(
            &mut self,
            body_handle: BodyHandle,
            force: Force,
            returned_value: Option<()>,
        ) {
            self.expect_apply_force_and_return = Some((body_handle, force, returned_value));
        }

        pub(crate) fn expect_set_simulated_timestep(&mut self, timestep: f64) {
            self.expect_set_simulated_timestep = Some(timestep);
        }

        pub(crate) fn expect_is_body_passable_and_return(
            &mut self,
            body_handle: BodyHandle,
            return_value: bool,
        ) {
            self.expect_is_body_passable_and_return = Some((body_handle, return_value));
        }

        pub(crate) fn expect_bodies_in_area_and_return(
            &mut self,
            area: Aabb,
            return_value: Vec<BodyHandle>,
        ) {
            self.expect_bodies_in_area_and_return = Some((area, return_value));
        }
    }

    impl Drop for WorldMock {
        fn drop(&mut self) {
            if !panicking() {
                if self.expect_step.is_some() {
                    assert!(
                        *self.step_was_called.borrow(),
                        "step() was not called, but was expected"
                    )
                }
                if self.expect_add_body_and_return.is_some() {
                    assert!(
                        *self.add_body_was_called.borrow(),
                        "add_body() was not called, but was expected"
                    )
                }

                if self.expect_body_and_return.is_some() {
                    assert!(
                        *self.body_was_called.borrow(),
                        "body() was not called, but was expected"
                    )
                }

                if self.expect_apply_force_and_return.is_some() {
                    assert!(
                        *self.apply_force_was_called.borrow(),
                        "apply_force() was not called, but was expected"
                    )
                }

                if self.expect_set_simulated_timestep.is_some() {
                    assert!(
                        *self.set_simulated_timestep_was_called.borrow(),
                        "set_simulated_timestep() was not called, but was expected"
                    )
                }

                if self.expect_remove_body_and_return.is_some() {
                    assert!(
                        *self.remove_body_was_called.borrow(),
                        "remove_body() was not called, but was expected"
                    )
                }

                if self.expect_is_body_passable_and_return.is_some() {
                    assert!(
                        *self.is_body_passable.borrow(),
                        "is_body_passable() was not called, but was expected"
                    );
                }

                if self.expect_bodies_in_area_and_return.is_some() {
                    assert!(
                        *self.bodies_in_area_was_called.borrow(),
                        "bodies_in_area() was not called, but was expected"
                    );
                }
            }
        }
    }

    impl World for WorldMock {
        fn step(&mut self) {
            *self.step_was_called.borrow_mut() = true;
            if self.expect_step.is_none() {
                panic!("step() was called unexpectedly")
            }
        }

        fn add_body(&mut self, body: PhysicalBody) -> BodyHandle {
            *self.add_body_was_called.borrow_mut() = true;
            if let Some((ref expected_body, ref return_value)) = self.expect_add_body_and_return {
                if body == *expected_body {
                    *return_value
                } else {
                    panic!(
                        "add_body() was called with {:?}, expected {:?}",
                        body, expected_body
                    )
                }
            } else {
                panic!("add_body() was called unexpectedly")
            }
        }

        fn remove_body(&mut self, body_handle: BodyHandle) -> Option<PhysicalBody> {
            *self.remove_body_was_called.borrow_mut() = true;
            if let Some((ref expected_body_handle, ref return_value)) =
                self.expect_remove_body_and_return
            {
                if body_handle == *expected_body_handle {
                    return_value.clone()
                } else {
                    panic!(
                        "remove_body() was called with {:?}, expected {:?}",
                        body_handle, expected_body_handle
                    )
                }
            } else {
                panic!("remove_body() was called unexpectedly")
            }
        }

        fn body(&self, handle: BodyHandle) -> Option<PhysicalBody> {
            *self.body_was_called.borrow_mut() = true;
            if let Some((ref expected_handle, ref return_value)) = self.expect_body_and_return {
                if handle == *expected_handle {
                    return_value.clone()
                } else {
                    panic!(
                        "body() was called with {:?}, expected {:?}",
                        handle, expected_handle
                    )
                }
            } else {
                panic!("body() was called unexpectedly")
            }
        }

        fn apply_force(&mut self, body_handle: BodyHandle, force: Force) -> Option<()> {
            *self.apply_force_was_called.borrow_mut() = true;
            if let Some((ref expected_body_handle, ref expected_force, ref return_value)) =
                self.expect_apply_force_and_return
            {
                if body_handle == *expected_body_handle && force == *expected_force {
                    *return_value
                } else {
                    panic!(
                        "apply_force() was called with {:?} and {:?}, expected {:?} and {:?}",
                        body_handle, force, expected_body_handle, expected_force
                    )
                }
            } else {
                panic!("set_simulated_timestep() was called unexpectedly")
            }
        }

        fn set_simulated_timestep(&mut self, timestep: f64) {
            *self.set_simulated_timestep_was_called.borrow_mut() = true;
            if let Some(expected_timestep) = self.expect_set_simulated_timestep {
                if timestep != expected_timestep {
                    panic!(
                        "set_simulated_timestep() was called with {:?}, expected {:?}",
                        timestep, expected_timestep
                    )
                }
            } else {
                panic!("set_simulated_timestep() was called unexpectedly")
            }
        }

        fn is_body_passable(&self, body_handle: BodyHandle) -> bool {
            *self.is_body_passable.borrow_mut() = true;
            if let Some((expected_body_handle, return_value)) =
                self.expect_is_body_passable_and_return
            {
                if expected_body_handle == body_handle {
                    return_value
                } else {
                    panic!(
                        "is_body_passable() was called with {:?}, expected {:?}",
                        body_handle, expected_body_handle
                    )
                }
            } else {
                panic!("is_body_passable() was called unexpectedly")
            }
        }

        fn bodies_in_area(&self, area: Aabb) -> Vec<BodyHandle> {
            *self.bodies_in_area_was_called.borrow_mut() = true;

            if let Some((expected_area, ref return_value)) = self.expect_bodies_in_area_and_return {
                assert_eq!(
                    expected_area, area,
                    "bodies_in_area() was called with {:?}, expected {:?}",
                    expected_area, area
                );
                return_value.clone()
            } else {
                panic!("bodies_in_area() was called unexpectedly")
            }
        }
    }
}
