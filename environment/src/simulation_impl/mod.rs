//! A `Simulation` that outsources all physical
//! behaviour into a separate `World` type

use crate::object::{
    Mobility, Object, ObjectBehavior, ObjectDescription, Polygon, Position, Sensor, Velocity,
};
use crate::Simulation;
use std::collections::HashMap;
use std::fmt;

pub mod world;

/// Implementation of [`Simulation`] that uses a physical
/// [`World`] in order to apply physics to objects.
///
/// [`Simulation`]: ./../trait.Simulation.html
/// [`World`]: ./trait.World.html
#[derive(Debug)]
pub struct SimulationImpl {
    world: Box<dyn World>,
    objects: HashMap<BodyHandle, ObjectBehavior>,
}

impl SimulationImpl {
    /// Create a new SimulationImpl by injecting a [`World`]
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::{SimulationImpl, world::NphysicsWorld};
    ///
    /// let world = Box::new(NphysicsWorld::with_timestep(1.0));
    /// let simulation = SimulationImpl::new(world);
    /// ```
    /// [`World`]: ./trait.World.html
    pub fn new(world: Box<dyn World>) -> Self {
        Self {
            world,
            objects: HashMap::new(),
        }
    }

    fn convert_to_object_description(
        &self,
        body_handle: BodyHandle,
        object: &ObjectBehavior,
    ) -> ObjectDescription {
        let physics_body = self
            .world
            .body(body_handle)
            .expect("Internal error: Stored body handle was invalid");
        ObjectDescription {
            shape: physics_body.shape,
            position: physics_body.position,
            mobility: physics_body.mobility,
            kind: object.kind(),
        }
    }
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        for object in self.objects.values_mut() {
            match object {
                ObjectBehavior::Movable(object) => {
                    object.step();
                }
                ObjectBehavior::Immovable(object) => {
                    object.step();
                }
            }
        }
        self.world.step()
    }

    fn add_object(&mut self, object: Object) {
        let mobility = match object.object_behavior {
            ObjectBehavior::Immovable(_) => Mobility::Immovable,
            ObjectBehavior::Movable(_) => Mobility::Movable(Velocity::default()),
        };
        let physical_body = PhysicalBody {
            shape: object.shape,
            position: object.position,
            mobility,
        };
        let body_handle = self.world.add_body(physical_body);
        self.objects.insert(body_handle, object.object_behavior);
    }

    fn objects(&self) -> Vec<ObjectDescription> {
        self.objects
            .iter()
            .map(|(&handle, object)| self.convert_to_object_description(handle, object))
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

    /// Attaches a sensor to the body identified by `body_handle`.
    /// # Errors
    /// Returns `None` if `body_handle` did not match any bodies
    fn attach_sensor(&mut self, body_handle: BodyHandle, sensor: Sensor) -> Option<SensorHandle>;

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

    /// Retrieves the handles of all bodies that are within the range of
    /// a sensor, excluding the parent body it is attached to.
    /// # Errors
    /// Returns `None` if `sensor_handle` did not match any sensors
    fn bodies_within_sensor(&self, sensor_handle: SensorHandle) -> Option<Vec<BodyHandle>>;

    /// Sets how much time in seconds is simulated for each step.
    /// # Examples
    /// If you want to run a simulation with 60 steps per second, you
    /// can run `set_simulated_timestep(1.0/60.0)`. Note that this method
    /// does not block the thread if called faster than expected.
    fn set_simulated_timestep(&mut self, timestep: f64);
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
    /// The current position of the body
    pub position: Position,
    /// The current mobility of the object. If present,
    /// this is defined as a two dimensional vector relative to the
    /// objects center
    pub mobility: Mobility,
}

/// A unique identifier that can be used to retrieve a [`PhysicalBody`] from a
/// [`World`].
///
/// Don't construct any of these by yourself, only use the
/// instances that [`World`] provides you
///
/// [`PhysicalBody`]: ./struct.PhysicalBody.html
/// [`World`]: ./trait.World.html
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BodyHandle(pub usize);

/// A unique identifier that can be used to retrieve [`PhysicalBodies`] that
/// are within the range of a [`Sensor`].
///
/// Don't construct any of these by yourself, only use the
/// instances that [`World`] provides you
///
/// [`PhysicalBodies`]: ./struct.PhysicalBody.html
/// [`Sensor`]: ../object/struct.Sensor.html
/// [`World`]: ./trait.World.html
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SensorHandle(pub usize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::*;
    use crate::object_builder::PolygonBuilder;
    use std::cell::RefCell;

    #[test]
    fn propagates_step() {
        let mut world = Box::new(WorldMock::new());
        world.expect_step();
        let mut simulation = SimulationImpl::new(world);
        simulation.step();
    }

    #[test]
    fn propagates_simulated_timestep() {
        let mut world = Box::new(WorldMock::new());
        const EXPECTED_TIMESTEP: f64 = 1.0;
        world.expect_set_simulated_timestep(EXPECTED_TIMESTEP);
        let mut simulation = SimulationImpl::new(world);
        simulation.set_simulated_timestep(EXPECTED_TIMESTEP);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_timestep() {
        let world = Box::new(WorldMock::new());
        let mut simulation = SimulationImpl::new(world);
        const INVALID_TIMESTEP: f64 = -0.1;
        simulation.set_simulated_timestep(INVALID_TIMESTEP);
    }

    #[test]
    fn propagates_zero_timestep() {
        let mut world = Box::new(WorldMock::new());
        const EXPECTED_TIMESTEP: f64 = 0.0;
        world.expect_set_simulated_timestep(EXPECTED_TIMESTEP);
        let mut simulation = SimulationImpl::new(world);
        simulation.set_simulated_timestep(EXPECTED_TIMESTEP);
    }

    #[test]
    fn returns_no_objects_when_empty() {
        let world = Box::new(WorldMock::new());
        let simulation = SimulationImpl::new(world);
        let objects = simulation.objects();
        assert!(objects.is_empty())
    }

    #[test]
    fn converts_to_physical_body() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: Mobility::Movable(Velocity { x: 0, y: 0 }),
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);
        let mut simulation = SimulationImpl::new(world);

        let object = Object {
            object_behavior: ObjectBehavior::Movable(Box::new(ObjectMock::new())),
            position: expected_position,
            shape: expected_shape,
        };
        simulation.add_object(object);
    }

    #[test]
    fn propagates_step_to_added_object() {
        let mut world = Box::new(WorldMock::new());
        world.expect_step();
        let expected_shape = shape();
        let expected_position = position();
        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: Mobility::Movable(Velocity { x: 0, y: 0 }),
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);
        let mut simulation = SimulationImpl::new(world);

        let mut object = ObjectMock::new();
        object.expect_step();
        let object = Object {
            object_behavior: ObjectBehavior::Movable(Box::new(object)),
            position: expected_position,
            shape: expected_shape,
        };
        simulation.add_object(object);
        simulation.step();
    }

    #[test]
    fn returns_added_object() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: Mobility::Movable(Velocity::default()),
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));
        let mut simulation = SimulationImpl::new(world);

        let object = Box::new(ObjectMock::default());
        let expected_kind = object.kind();
        let new_object = Object {
            object_behavior: ObjectBehavior::Movable(object),
            position: expected_position.clone(),
            shape: expected_shape.clone(),
        };
        simulation.add_object(new_object);

        let objects = simulation.objects();
        assert_eq!(1, objects.len());

        let expected_object_description = ObjectDescription {
            position: expected_position,
            shape: expected_shape,
            kind: expected_kind,
            mobility: Mobility::Movable(Velocity::default()),
        };
        let object_description = &objects[0];
        assert_eq!(expected_object_description, *object_description);
    }

    #[should_panic]
    #[test]
    fn panics_on_invalid_body() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: Mobility::Movable(Velocity::default()),
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, None);
        let mut simulation = SimulationImpl::new(world);

        let object = Box::new(ObjectMock::default());
        let new_object = Object {
            object_behavior: ObjectBehavior::Movable(object),
            position: expected_position.clone(),
            shape: expected_shape.clone(),
        };
        simulation.add_object(new_object);
        simulation.objects();
    }

    fn shape() -> Polygon {
        PolygonBuilder::new()
            .vertex(-5, -5)
            .vertex(5, -5)
            .vertex(5, 5)
            .vertex(-5, 5)
            .build()
            .unwrap()
    }

    fn position() -> Position {
        Position {
            location: Location { x: 30, y: 40 },
            rotation: Radians(3.4),
        }
    }

    #[derive(Debug, Default)]
    struct WorldMock {
        expect_step: Option<()>,
        expect_add_body_and_return: Option<(PhysicalBody, BodyHandle)>,
        expect_body_and_return: Option<(BodyHandle, Option<PhysicalBody>)>,
        expect_attach_sensor_and_return: Option<(BodyHandle, Sensor, Option<SensorHandle>)>,
        expect_set_simulated_timestep: Option<f64>,

        step_was_called: RefCell<bool>,
        add_body_was_called: RefCell<bool>,
        attach_sensor_was_called: RefCell<bool>,
        body_was_called: RefCell<bool>,
        set_simulated_timestep_was_called: RefCell<bool>,
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

        pub(crate) fn expect_body_and_return(
            &mut self,
            handle: BodyHandle,
            returned_value: Option<PhysicalBody>,
        ) {
            self.expect_body_and_return = Some((handle, returned_value));
        }

        pub(crate) fn expect_set_simulated_timestep(&mut self, timestep: f64) {
            self.expect_set_simulated_timestep = Some(timestep);
        }
    }

    impl Drop for WorldMock {
        fn drop(&mut self) {
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
            if self.expect_set_simulated_timestep.is_some() {
                assert!(
                    *self.set_simulated_timestep_was_called.borrow(),
                    "set_simulated_timestep() was not called, but was expected"
                )
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
                    return_value.clone()
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
        fn attach_sensor(
            &mut self,
            body_handle: BodyHandle,
            sensor: Sensor,
        ) -> Option<SensorHandle> {
            *self.attach_sensor_was_called.borrow_mut() = true;
            if let Some((ref expected_handle, ref expected_sensor, ref return_value)) =
                self.expect_attach_sensor_and_return
            {
                if body_handle == *expected_handle && sensor == *expected_sensor {
                    return_value.clone()
                } else {
                    panic!(
                        "attach_sensor() was called with {:?} and {:?}, expected {:?} and {:?}",
                        body_handle, sensor, expected_handle, expected_sensor
                    )
                }
            } else {
                panic!("attach_sensor() was called unexpectedly")
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
    }

    #[derive(Debug, Default)]
    struct ObjectMock {
        expects_step: bool,
        step_was_called: RefCell<bool>,
    }

    impl ObjectMock {
        fn new() -> ObjectMock {
            Default::default()
        }

        fn expect_step(&mut self) {
            self.expects_step = true
        }
    }

    impl MovableObject for ObjectMock {
        fn step(&mut self) -> Vec<MovableAction> {
            if self.expects_step {
                *self.step_was_called.borrow_mut() = true;
                // Actions are currently ignored
                Vec::new()
            } else {
                panic!("step() was called unexpectedly")
            }
        }
        fn kind(&self) -> Kind {
            Kind::Organism
        }
    }
    impl Drop for ObjectMock {
        fn drop(&mut self) {
            if self.expects_step {
                assert!(
                    *self.step_was_called.borrow(),
                    "step() was not called, but was expected"
                )
            }
        }
    }
}
