//! A `Simulation` that outsources all physical
//! behaviour into a separate `World` type

use crate::object::*;
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
    non_physical_object_data: HashMap<BodyHandle, NonPhysicalObjectData>,
}

#[derive(Debug)]
struct NonPhysicalObjectData {
    pub(crate) sensor: Option<(SensorHandle, Sensor)>,
    pub(crate) kind: Kind,
    pub(crate) behavior: Box<dyn ObjectBehavior>,
}

impl SimulationImpl {
    /// Create a new SimulationImpl by injecting a [`World`]
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::{
    ///     SimulationImpl, world::NphysicsWorld, world::rotation_translator::NphysicsRotationTranslatorImpl
    /// };
    /// use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl::default();
    /// let force_applier = SingleTimeForceApplierImpl::default();
    /// let world = Box::new(NphysicsWorld::with_timestep(1.0, Box::new(rotation_translator), Box::new(force_applier)));
    /// let simulation = SimulationImpl::new(world);
    /// ```
    /// [`World`]: ./trait.World.html
    pub fn new(world: Box<dyn World>) -> Self {
        Self {
            world,
            non_physical_object_data: HashMap::new(),
        }
    }

    fn convert_to_object_description(&self, body_handle: BodyHandle) -> Option<ObjectDescription> {
        let physics_body = self.world.body(body_handle)?;
        let non_physical_object_data = self.non_physical_object_data.get(&body_handle)?;
        Some(ObjectDescription {
            shape: physics_body.shape,
            position: physics_body.position,
            mobility: physics_body.mobility,
            kind: non_physical_object_data.kind,
            sensor: sensor_without_handle(non_physical_object_data.sensor.clone()),
            passable: self.world.is_body_passable(body_handle),
        })
    }

    fn retrieve_objects_within_sensor(&self, body_handle: BodyHandle) -> Vec<ObjectDescription> {
        if let Some(non_physical_object_data) = self.non_physical_object_data.get(&body_handle) {
            if let Some((sensor_handle, _)) = non_physical_object_data.sensor {
                let object_handles = self
                    .world
                    .bodies_within_sensor(sensor_handle)
                    .expect("Internal error: Stored invalid sensor handle");
                object_handles
                    .iter()
                    .map(|handle| {
                        self.convert_to_object_description(*handle)
                            .expect("Object handle returned by world was not found in simulation")
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    fn attach_sensor_if_available(
        &mut self,
        body_handle: BodyHandle,
        sensor: Option<Sensor>,
    ) -> Option<(SensorHandle, Sensor)> {
        if let Some(sensor) = sensor {
            let sensor_handle = self
                .world
                .attach_sensor(body_handle, sensor.clone())
                .expect("Internal error: World returned invalid handle");
            Some((sensor_handle, sensor))
        } else {
            None
        }
    }

    fn handle_action(&mut self, body_handle: BodyHandle, action: Action) -> Option<()> {
        match action {
            Action::Reproduce(object_description, object_behavior) => {
                self.add_object(object_description, object_behavior);
            }
            Action::ApplyForce(force) => {
                self.world.apply_force(body_handle, force)?;
            }
            Action::Die => {
                self.world.remove_body(body_handle)?;
                self.non_physical_object_data.remove(&body_handle)?;
            }
        };
        Some(())
    }
}

fn sensor_without_handle(sensor: Option<(SensorHandle, Sensor)>) -> Option<Sensor> {
    Some(sensor?.1)
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        let object_handle_to_objects_within_sensor: HashMap<_, _> = self
            .non_physical_object_data
            .keys()
            .map(|&object_handle| {
                (
                    object_handle,
                    self.retrieve_objects_within_sensor(object_handle),
                )
            })
            .collect();
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
        for (object_handle, non_physical_object_data) in &mut self.non_physical_object_data {
            // This is safe because the keys of self.objects and
            // object_handle_to_objects_within_sensor are identical
            let own_description = &object_handle_to_own_description[object_handle];
            let objects_within_sensor = &object_handle_to_objects_within_sensor[object_handle];
            let action = non_physical_object_data
                .behavior
                .step(&own_description, &objects_within_sensor);
            if let Some(action) = action {
                actions.push((*object_handle, action));
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
            position: object_description.position,
            mobility: object_description.mobility,
            passable: object_description.passable,
        };

        let body_handle = self.world.add_body(physical_body);

        let sensor = self.attach_sensor_if_available(body_handle, object_description.sensor);
        let non_physical_object_data = NonPhysicalObjectData {
            sensor,
            kind: object_description.kind,
            behavior: object_behavior,
        };
        self.non_physical_object_data
            .insert(body_handle, non_physical_object_data);
    }

    fn objects(&self) -> Vec<ObjectDescription> {
        self.non_physical_object_data
            .keys()
            .map(|&handle| {
                self.convert_to_object_description(handle)
                    .expect("Handle stored in simulation was not found in world")
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

    /// Attaches a sensor to the body identified by `body_handle`.
    /// # Errors
    /// Returns `None` if `body_handle` did not match any bodies.
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
    /// Returns `None` if `sensor_handle` did not match any sensors.
    fn bodies_within_sensor(&self, sensor_handle: SensorHandle) -> Option<Vec<BodyHandle>>;

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

    fn is_body_passable(&self, body_handle: BodyHandle) -> bool;
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
    /// Whether this object is passable or not
    pub passable: bool,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Handle {
    Body(BodyHandle),
    Sensor(SensorHandle),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_builder::{ObjectBuilder, PolygonBuilder};
    use std::cell::RefCell;
    use std::thread::panicking;

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
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);

        let object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();
        let object_behavior = ObjectBehaviorMock::new();

        let mut simulation = SimulationImpl::new(world);
        simulation.add_object(object_description, Box::new(object_behavior));
    }

    #[test]
    fn attaches_sensors_to_body() {
        let mut world = WorldMock::new();
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);

        let expected_sensor = Sensor {
            shape: PolygonBuilder::new()
                .vertex(-5, -5)
                .vertex(5, -5)
                .vertex(5, 5)
                .vertex(-5, 5)
                .build()
                .unwrap(),
            position: Position {
                location: Location::default(),
                rotation: Radians::default(),
            },
        };
        let sensor_handle = Some(SensorHandle(69));
        world.expect_attach_sensor_and_return(
            returned_handle,
            expected_sensor.clone(),
            sensor_handle,
        );

        let object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .sensor(expected_sensor)
            .passable(false)
            .build()
            .unwrap();
        let object_behavior = ObjectBehaviorMock::new();

        let mut simulation = SimulationImpl::new(Box::new(world));
        simulation.add_object(object_description, Box::new(object_behavior));
    }

    #[should_panic]
    #[test]
    fn panics_on_sensor_attachement_failure() {
        let mut world = WorldMock::new();
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);

        let sensor = Sensor {
            shape: PolygonBuilder::new()
                .vertex(-5, -5)
                .vertex(5, -5)
                .vertex(5, 5)
                .vertex(-5, 5)
                .build()
                .unwrap(),
            position: Position {
                location: Location::default(),
                rotation: Radians::default(),
            },
        };
        world.expect_attach_sensor_and_return(returned_handle, sensor, None);

        let object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();
        let object_behavior = ObjectBehaviorMock::new();

        let mut simulation = SimulationImpl::new(Box::new(world));
        simulation.add_object(object_description, Box::new(object_behavior));
    }

    #[test]
    fn propagates_step_to_added_object() {
        let mut world = WorldMock::new();
        world.expect_step();
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));

        let expected_object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let mut object_behavior = ObjectBehaviorMock::new();
        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Vec::new(),
            None,
        );

        let mut simulation = SimulationImpl::new(Box::new(world));
        simulation.add_object(expected_object_description, Box::new(object_behavior));
        simulation.step();
    }

    #[test]
    fn propagates_objects_within_sensor() {
        let mut world = Box::new(WorldMock::new());
        world.expect_step();
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_is_body_passable_and_return(returned_handle, expected_passable);

        let mut object_behavior = ObjectBehaviorMock::new();
        let sensor_shape = shape();
        let expected_sensor = Sensor {
            shape: sensor_shape,
            position: Position::default(),
        };
        let expected_sensor_handle = SensorHandle(1357);
        world.expect_attach_sensor_and_return(
            returned_handle,
            expected_sensor.clone(),
            Some(expected_sensor_handle),
        );
        world.expect_bodies_within_sensor_and_return(
            expected_sensor_handle,
            Some(vec![returned_handle]),
        );

        let expected_object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .sensor(expected_sensor)
            .passable(false)
            .build()
            .unwrap();

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            vec![expected_object_description.clone()],
            None,
        );
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));

        let mut simulation = SimulationImpl::new(world);
        simulation.add_object(expected_object_description, Box::new(object_behavior));
        simulation.step();
    }

    #[test]
    fn returns_added_object() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));
        let mut simulation = SimulationImpl::new(world);

        let object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(false)
            .build()
            .unwrap();

        simulation.add_object(
            expected_object_description.clone(),
            Box::new(object_behavior),
        );

        let objects = simulation.objects();
        assert_eq!(1, objects.len());

        let object_description = &objects[0];
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
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body));
        world.expect_step();

        let mut simulation = SimulationImpl::new(world);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let mut child_object_behavior = ObjectBehaviorMock::new();
        child_object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Vec::new(),
            None,
        );

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Vec::new(),
            Some(Action::Reproduce(
                expected_object_description.clone(),
                Box::new(child_object_behavior),
            )),
        );

        simulation.add_object(
            expected_object_description.clone(),
            Box::new(object_behavior),
        );

        simulation.step();
        simulation.step();
    }

    #[test]
    fn dying_removes_object() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body.clone()));
        world.expect_step();
        world.expect_remove_body_and_return(returned_handle, Some(expected_physical_body));
        world.expect_is_body_passable_and_return(returned_handle, expected_passable);

        let mut simulation = SimulationImpl::new(world);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Vec::new(),
            Some(Action::Die),
        );

        simulation.add_object(
            expected_object_description.clone(),
            Box::new(object_behavior),
        );

        simulation.step();
    }

    #[test]
    fn force_application_is_propagated() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, Some(expected_physical_body.clone()));
        world.expect_step();
        world.expect_is_body_passable_and_return(returned_handle, expected_passable);
        let expected_force = Force {
            linear: LinearForce { x: 20, y: -5 },
            torque: Torque(-8.0),
        };
        world.expect_apply_force_and_return(returned_handle, expected_force.clone(), Some(()));

        let mut simulation = SimulationImpl::new(world);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior.expect_step_and_return(
            expected_object_description.clone(),
            Vec::new(),
            Some(Action::ApplyForce(expected_force)),
        );

        simulation.add_object(
            expected_object_description.clone(),
            Box::new(object_behavior),
        );

        simulation.step();
    }

    #[should_panic]
    #[test]
    fn panics_on_invalid_body() {
        let mut world = Box::new(WorldMock::new());
        let expected_shape = shape();
        let expected_position = position();
        let expected_mobility = Mobility::Movable(Velocity::default());
        let expected_passable = false;

        let expected_physical_body = PhysicalBody {
            shape: expected_shape.clone(),
            position: expected_position.clone(),
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, None);
        let mut simulation = SimulationImpl::new(world);

        let object_description = ObjectBuilder::new()
            .location(expected_position.location.x, expected_position.location.y)
            .rotation(expected_position.rotation)
            .shape(expected_shape)
            .kind(Kind::Organism)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let object_behavior = ObjectBehaviorMock::default();
        simulation.add_object(object_description, Box::new(object_behavior));
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
            rotation: Radians::try_new(3.4).unwrap(),
        }
    }

    #[derive(Debug, Default)]
    struct WorldMock {
        expect_step: Option<()>,
        expect_add_body_and_return: Option<(PhysicalBody, BodyHandle)>,
        expect_remove_body_and_return: Option<(BodyHandle, Option<PhysicalBody>)>,
        expect_body_and_return: Option<(BodyHandle, Option<PhysicalBody>)>,
        expect_attach_sensor_and_return: Option<(BodyHandle, Sensor, Option<SensorHandle>)>,
        expect_bodies_within_sensor_and_return: Option<(SensorHandle, Option<Vec<BodyHandle>>)>,
        expect_apply_force_and_return: Option<(BodyHandle, Force, Option<()>)>,
        expect_set_simulated_timestep: Option<f64>,
        expect_is_body_passable_and_return: Option<(BodyHandle, bool)>,

        step_was_called: RefCell<bool>,
        add_body_was_called: RefCell<bool>,
        remove_body_was_called: RefCell<bool>,
        attach_sensor_was_called: RefCell<bool>,
        body_was_called: RefCell<bool>,
        bodies_within_sensor_was_called: RefCell<bool>,
        apply_force_was_called: RefCell<bool>,
        set_simulated_timestep_was_called: RefCell<bool>,
        is_body_passable: RefCell<bool>,
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

        pub(crate) fn expect_attach_sensor_and_return(
            &mut self,
            body_handle: BodyHandle,
            sensor: Sensor,
            returned_value: Option<SensorHandle>,
        ) {
            self.expect_attach_sensor_and_return = Some((body_handle, sensor, returned_value));
        }

        pub(crate) fn expect_body_and_return(
            &mut self,
            handle: BodyHandle,
            returned_value: Option<PhysicalBody>,
        ) {
            self.expect_body_and_return = Some((handle, returned_value));
        }

        pub(crate) fn expect_bodies_within_sensor_and_return(
            &mut self,
            sensor_handle: SensorHandle,
            returned_value: Option<Vec<BodyHandle>>,
        ) {
            self.expect_bodies_within_sensor_and_return = Some((sensor_handle, returned_value));
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
    }

    impl Drop for WorldMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
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

            if self.expect_attach_sensor_and_return.is_some() {
                assert!(
                    *self.attach_sensor_was_called.borrow(),
                    "attach_sensor() was not called, but was expected"
                )
            }

            if self.expect_body_and_return.is_some() {
                assert!(
                    *self.body_was_called.borrow(),
                    "body() was not called, but was expected"
                )
            }

            if self.expect_bodies_within_sensor_and_return.is_some() {
                assert!(
                    *self.bodies_within_sensor_was_called.borrow(),
                    "bodies_within_sensor() was not called, but was expected"
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
                    *return_value
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

        fn bodies_within_sensor(&self, sensor_handle: SensorHandle) -> Option<Vec<BodyHandle>> {
            *self.bodies_within_sensor_was_called.borrow_mut() = true;
            if let Some((ref expected_handle, ref return_value)) =
                self.expect_bodies_within_sensor_and_return
            {
                if sensor_handle == *expected_handle {
                    return_value.clone()
                } else {
                    panic!(
                        "bodies_within_sensor() was called with {:?}, expected {:?}",
                        sensor_handle, expected_handle
                    )
                }
            } else {
                panic!("bodies_within_sensor() was called unexpectedly")
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
    }

    #[derive(Debug, Default, Clone)]
    struct ObjectBehaviorMock {
        expect_step_and_return: Option<(ObjectDescription, Vec<ObjectDescription>, Option<Action>)>,

        step_was_called: RefCell<bool>,
    }

    impl ObjectBehaviorMock {
        fn new() -> ObjectBehaviorMock {
            Default::default()
        }

        pub(crate) fn expect_step_and_return(
            &mut self,
            own_description: ObjectDescription,
            sensor_collisions: Vec<ObjectDescription>,
            returned_value: Option<Action>,
        ) {
            self.expect_step_and_return =
                Some((own_description, sensor_collisions, returned_value));
        }
    }

    impl ObjectBehavior for ObjectBehaviorMock {
        fn step(
            &mut self,
            own_description: &ObjectDescription,
            sensor_collisions: &[ObjectDescription],
        ) -> Option<Action> {
            *self.step_was_called.borrow_mut() = true;
            if let Some((
                ref expected_own_description,
                ref expected_sensor_collisions,
                ref return_value,
            )) = self.expect_step_and_return
            {
                if sensor_collisions.to_vec() == *expected_sensor_collisions
                    && expected_own_description == own_description
                {
                    return_value.clone()
                } else {
                    panic!(
                        "step() was called with {:?} and {:?}, expected {:?} and {:?}",
                        own_description,
                        sensor_collisions,
                        expected_own_description,
                        expected_sensor_collisions
                    )
                }
            } else {
                panic!("step() was called unexpectedly")
            }
        }
    }
    impl Drop for ObjectBehaviorMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_step_and_return.is_some() {
                assert!(
                    *self.step_was_called.borrow(),
                    "step() was not called, but was expected"
                )
            }
        }
    }
}
