use super::{NewObject, ObjectDescription, Simulation};
use crate::object::{Mobility, Object, Polygon, Position, Velocity};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct SimulationImpl {
    world: Box<dyn World>,
    objects: HashMap<BodyHandle, Object>,
}

impl SimulationImpl {
    pub fn new(world: Box<dyn World>) -> Self {
        Self {
            world,
            objects: HashMap::new(),
        }
    }

    fn convert_to_object_description(
        &self,
        body_handle: BodyHandle,
        object: &Object,
    ) -> ObjectDescription {
        let physics_body = self.world.body(body_handle);
        let kind = match object {
            Object::Immovable(object) => object.kind(),
            Object::Movable(object) => object.kind(),
        };
        ObjectDescription {
            shape: physics_body.shape,
            position: physics_body.position,
            velocity: physics_body.velocity,
            kind,
        }
    }
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        for (_, object) in &mut self.objects {
            match object {
                Object::Movable(object) => {
                    object.step();
                }
                Object::Immovable(object) => {
                    object.step();
                }
            }
        }
        self.world.step()
    }

    fn add_object(&mut self, new_object: NewObject) {
        let velocity = match new_object.object {
            Object::Immovable(_) => Mobility::Immovable,
            Object::Movable(_) => Mobility::Movable(Velocity::default()),
        };
        let physical_body = PhysicalBody {
            shape: new_object.shape,
            position: new_object.position,
            velocity,
        };
        let body_handle = self.world.add_body(physical_body);
        self.objects.insert(body_handle, new_object.object);
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

pub trait World: fmt::Debug {
    fn step(&mut self);
    fn add_body(&mut self, body: PhysicalBody) -> BodyHandle;
    fn body(&self, handle: BodyHandle) -> PhysicalBody;
    fn set_simulated_timestep(&mut self, timestep: f64);
}

#[derive(Debug, PartialEq, Clone)]
pub struct PhysicalBody {
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BodyHandle(pub usize);

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
            velocity: Mobility::Movable(Velocity { x: 0, y: 0 }),
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);
        let mut simulation = SimulationImpl::new(world);

        let object = NewObject {
            object: Object::Movable(Box::new(ObjectMock::new())),
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
            velocity: Mobility::Movable(Velocity { x: 0, y: 0 }),
        };
        let returned_handle = BodyHandle(1337);
        world.expect_add_body_and_return(expected_physical_body, returned_handle);
        let mut simulation = SimulationImpl::new(world);

        let mut object = ObjectMock::new();
        object.expect_step();
        let object = NewObject {
            object: Object::Movable(Box::new(object)),
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
            velocity: Mobility::Movable(Velocity::default()),
        };
        let returned_handle = BodyHandle(1984);
        world.expect_add_body_and_return(expected_physical_body.clone(), returned_handle);
        world.expect_body_and_return(returned_handle, expected_physical_body);
        let mut simulation = SimulationImpl::new(world);

        let object = Box::new(ObjectMock::default());
        let expected_kind = object.kind();
        let new_object = NewObject {
            object: Object::Movable(object),
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
            velocity: Mobility::Movable(Velocity::default()),
        };
        let object_description = &objects[0];
        assert_eq!(expected_object_description, *object_description);
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
        expect_body_and_return: Option<(BodyHandle, PhysicalBody)>,
        expect_set_simulated_timestep: Option<f64>,

        step_was_called: RefCell<bool>,
        add_body_was_called: RefCell<bool>,
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
            returned_value: PhysicalBody,
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
        fn body(&self, handle: BodyHandle) -> PhysicalBody {
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
