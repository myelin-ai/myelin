//! A `Simulation` that outsources all physical
//! behaviour into a separate `World` type

pub mod world;

use self::world::{BodyHandle, PhysicalBody, World};
use crate::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug};

/// Factory used by [`SimulationImpl`] to create an [`WorldInteractor`].
///
/// [`SimulationImpl`]: ./struct.SimulationImpl.html
/// [`WorldInteractor`]: ./../object/trait.WorldInteractor.html
pub type WorldInteractorFactoryFn =
    dyn for<'a> Fn(&'a dyn Simulation) -> Box<dyn WorldInteractor + 'a>;

/// Implementation of [`Simulation`] that uses a physical
/// [`World`] in order to apply physics to objects.
///
/// [`Simulation`]: ./../trait.Simulation.html
/// [`World`]: ./trait.World.html
pub struct SimulationImpl {
    world: Box<dyn World>,
    non_physical_object_data: HashMap<BodyHandle, NonPhysicalObjectData>,
    world_interactor_factory_fn: Box<WorldInteractorFactoryFn>,
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
    pub(crate) behavior: RefCell<Box<dyn ObjectBehavior>>,
    pub(crate) associated_data: Vec<u8>,
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

type ActionResult = Result<(), ActionError>;

impl SimulationImpl {
    /// Create a new SimulationImpl by injecting a [`World`]
    /// # Examples
    /// ```
    /// use myelin_environment::prelude::*;
    /// use myelin_environment::simulation::world::{
    ///     rotation_translator::NphysicsRotationTranslatorImpl, NphysicsWorld,
    ///     SingleTimeForceApplierImpl,
    /// };
    /// use myelin_environment::simulation::SimulationImpl;
    /// use myelin_environment::world_interactor::WorldInteractorImpl;
    /// use std::sync::{Arc, RwLock};
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl::default();
    /// let force_applier = SingleTimeForceApplierImpl::default();
    /// let world = Box::new(NphysicsWorld::with_timestep(
    ///     1.0,
    ///     Box::new(rotation_translator),
    ///     Box::new(force_applier),
    /// ));
    /// let simulation = SimulationImpl::new(
    ///     world,
    ///     Box::new(|simulation| Box::new(WorldInteractorImpl::new(simulation))),
    /// );
    /// ```
    /// [`World`]: ./trait.World.html
    pub fn new(
        world: Box<dyn World>,
        world_interactor_factory_fn: Box<WorldInteractorFactoryFn>,
    ) -> Self {
        Self {
            world,
            non_physical_object_data: HashMap::new(),
            world_interactor_factory_fn,
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
            passable: self.world.is_body_passable(body_handle),
            associated_data: non_physical_object_data.associated_data.clone(),
        })
    }

    fn handle_action(&mut self, body_handle: BodyHandle, action: Action) -> ActionResult {
        match action {
            Action::Spawn(object_description, object_behavior) => {
                self.spawn(object_description, object_behavior)
            }
            Action::ApplyForce(force) => self.apply_force(body_handle, force),
            Action::Destroy(object_id) => self.destroy(object_id),
            Action::DestroySelf => self.destroy_self(body_handle),
        }
    }

    fn spawn(
        &mut self,
        object_description: ObjectDescription,
        object_behavior: Box<dyn ObjectBehavior>,
    ) -> ActionResult {
        self.add_object(object_description, object_behavior);
        Ok(())
    }

    fn apply_force(&mut self, body_handle: BodyHandle, force: Force) -> ActionResult {
        self.world
            .apply_force(body_handle, force)
            .to_action_result()
    }

    fn destroy(&mut self, object_id: Id) -> ActionResult {
        self.world
            .remove_body(BodyHandle(object_id))
            .to_action_result()
    }

    fn destroy_self(&mut self, body_handle: BodyHandle) -> ActionResult {
        self.world
            .remove_body(body_handle)
            .and(self.non_physical_object_data.remove(&body_handle))
            .to_action_result()
    }
}

trait HandleOption {
    fn to_action_result(self) -> ActionResult;
}

impl<T> HandleOption for Option<T> {
    fn to_action_result(self) -> ActionResult {
        self.map(|_| ()).ok_or(ActionError::InvalidHandle)
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
            let world_interactor = (self.world_interactor_factory_fn)(self);
            for (object_handle, non_physical_object_data) in &self.non_physical_object_data {
                // This is safe because the keys of self.objects and
                // object_handle_to_objects_within_sensor are identical
                let own_description = &object_handle_to_own_description[object_handle];
                let action = non_physical_object_data
                    .behavior
                    .borrow_mut()
                    .step(&own_description, world_interactor.as_ref());
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
            behavior: RefCell::new(object_behavior),
            associated_data: object_description.associated_data.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::ObjectBehaviorMock;
    use crate::object_builder::ObjectBuilder;
    use crate::simulation::simulation_impl::world::WorldMock;
    use crate::world_interactor::{WorldInteractor, WorldInteractorMock};
    use mockiato::{any, partial_eq, partial_eq_owned};
    use myelin_geometry::PolygonBuilder;

    fn world_interactor_factory_fn<'a>(
        _simulation: &'a dyn Simulation,
    ) -> Box<dyn WorldInteractor + 'a> {
        box WorldInteractorMock::new()
    }

    #[test]
    fn propagates_step() {
        let mut world = box WorldMock::new();
        world.expect_step();
        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
        simulation.step();
    }

    #[test]
    fn propagates_simulated_timestep() {
        let mut world = box WorldMock::new();
        const EXPECTED_TIMESTEP: f64 = 1.0;
        world.expect_set_simulated_timestep(partial_eq(EXPECTED_TIMESTEP));
        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
        simulation.set_simulated_timestep(EXPECTED_TIMESTEP);
    }

    #[should_panic]
    #[test]
    fn panics_on_negative_timestep() {
        let world = box WorldMock::new();
        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
        const INVALID_TIMESTEP: f64 = -0.1;
        simulation.set_simulated_timestep(INVALID_TIMESTEP);
    }

    #[test]
    fn propagates_zero_timestep() {
        let mut world = box WorldMock::new();
        const EXPECTED_TIMESTEP: f64 = 0.0;
        world.expect_set_simulated_timestep(partial_eq(EXPECTED_TIMESTEP));
        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
        simulation.set_simulated_timestep(EXPECTED_TIMESTEP);
    }

    #[test]
    fn returns_no_objects_when_empty() {
        let world = box WorldMock::new();
        let simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
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
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world
            .expect_add_body(partial_eq(expected_physical_body))
            .returns(returned_handle);

        let object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();
        let object_behavior = ObjectBehaviorMock::new();

        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
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
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1337);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body));
        world
            .expect_is_body_passable(partial_eq(returned_handle))
            .returns(expected_passable);

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        let mut object_behavior = ObjectBehaviorMock::new();
        object_behavior
            .expect_step(partial_eq_owned(expected_object_description.clone()), any())
            .returns(None);

        let mut simulation = SimulationImpl::new(box world, box world_interactor_factory_fn);
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
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body));
        world
            .expect_is_body_passable(partial_eq(returned_handle))
            .returns(expected_passable);

        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);
        let object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
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
    fn spawns_object() {
        let mut world = box WorldMock::new();
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
        let returned_handle = BodyHandle(1984);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body));
        world.expect_step();

        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let mut child_object_behavior = ObjectBehaviorMock::new();
        child_object_behavior
            .expect_step(partial_eq_owned(expected_object_description.clone()), any())
            .returns(None);

        object_behavior
            .expect_step(partial_eq_owned(expected_object_description.clone()), any())
            .returns(Some(Action::Spawn(
                expected_object_description.clone(),
                box child_object_behavior,
            )));

        simulation.add_object(expected_object_description.clone(), box object_behavior);

        simulation.step();
        simulation.step();
    }

    #[test]
    fn destroying_self_removes_object() {
        let mut world = box WorldMock::new();
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
        let returned_handle = BodyHandle(1984);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body.clone()));
        world.expect_step();
        world
            .expect_remove_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body));
        world
            .expect_is_body_passable(partial_eq(returned_handle))
            .returns(expected_passable);

        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior
            .expect_step(partial_eq_owned(expected_object_description.clone()), any())
            .returns(Some(Action::DestroySelf));

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
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };

        let handle_one = BodyHandle(1);
        let handle_two = BodyHandle(2);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(handle_one);
        world
            .expect_body(partial_eq(handle_one))
            .returns(Some(expected_physical_body.clone()));
        world
            .expect_is_body_passable(partial_eq(handle_one))
            .returns(expected_passable);
        world.expect_step();
        world
            .expect_remove_body(partial_eq(handle_two))
            .returns(Some(expected_physical_body));

        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior
            .expect_step(partial_eq_owned(expected_object_description.clone()), any())
            .returns(Some(Action::Destroy(handle_two.0)));

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
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body.clone()));
        world.expect_step();
        world
            .expect_is_body_passable(partial_eq(returned_handle))
            .returns(expected_passable);
        let expected_force = Force {
            linear: Vector { x: 20.0, y: -5.0 },
            torque: Torque(-8.0),
        };
        world
            .expect_apply_force(
                partial_eq(returned_handle),
                partial_eq(expected_force.clone()),
            )
            .returns(Some(()));

        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);

        let mut object_behavior = ObjectBehaviorMock::new();

        let expected_object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .passable(expected_passable)
            .build()
            .unwrap();

        object_behavior
            .expect_step(partial_eq_owned(expected_object_description.clone()), any())
            .returns(Some(Action::ApplyForce(expected_force)));

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
            location: expected_location,
            rotation: expected_rotation,
            mobility: expected_mobility.clone(),
            passable: expected_passable,
        };
        let returned_handle = BodyHandle(1984);
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world.expect_body(partial_eq(returned_handle)).returns(None);
        let mut simulation = SimulationImpl::new(world, box world_interactor_factory_fn);

        let object_description = ObjectBuilder::default()
            .location(expected_location.x, expected_location.y)
            .rotation(expected_rotation)
            .shape(expected_shape)
            .mobility(expected_mobility)
            .build()
            .unwrap();

        let object_behavior = ObjectBehaviorMock::new();
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
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_bodies_in_area(partial_eq(area))
            .returns(vec![returned_handle]);
        world
            .expect_body(partial_eq(returned_handle))
            .returns(Some(expected_physical_body.clone()));
        world
            .expect_is_body_passable(partial_eq(returned_handle))
            .returns(expected_physical_body.passable);

        let mut simulation = SimulationImpl::new(box world, box world_interactor_factory_fn);

        let object_behavior = ObjectBehaviorMock::new();
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
        world
            .expect_add_body(partial_eq(expected_physical_body.clone()))
            .returns(returned_handle);
        world
            .expect_bodies_in_area(partial_eq(area))
            .returns(vec![returned_handle]);
        world.expect_body(partial_eq(returned_handle)).returns(None);

        let mut simulation = SimulationImpl::new(box world, box world_interactor_factory_fn);

        let object_behavior = ObjectBehaviorMock::new();
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
}
