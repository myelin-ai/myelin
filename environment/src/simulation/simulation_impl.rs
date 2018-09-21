use super::{NewObject, ObjectDescription, Simulation};
use crate::object::{Mobility, Object, Polygon, Position};
use std::fmt;

#[derive(Debug)]
pub struct SimulationImpl {
    world: Box<dyn World>,
}

impl SimulationImpl {
    pub fn new(world: Box<dyn World>) -> Self {
        Self { world }
    }
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        unimplemented!()
    }
    fn add_object(&mut self, object: NewObject) {
        unimplemented!()
    }
    fn objects(&self) -> Vec<ObjectDescription> {
        unimplemented!()
    }
    fn set_simulated_timestep(&mut self, timestep: f64) {
        unimplemented!()
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
    use std::borrow::BorrowMut;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    struct WorldMock {
        expect_step: Option<()>,
        expect_add_body_and_return: Option<(PhysicalBody, BodyHandle)>,
        expect_body_and_return: Option<(BodyHandle, PhysicalBody)>,
        expect_set_simulated_timestep: Option<f64>,

        step_was_called_with: RefCell<Option<()>>,
        add_body_was_called_with: RefCell<Option<(PhysicalBody, BodyHandle)>>,
        body_was_called_with: RefCell<Option<(BodyHandle, PhysicalBody)>>,
        set_simulated_timestep_was_called_with: RefCell<Option<f64>>,
    }
    impl WorldMock {
        fn new() -> Self {
            Default::default()
        }

        fn expect_step(&mut self) {
            self.expect_step = Some(());
        }

        fn expect_add_body_and_return(&mut self, body: PhysicalBody, returned_value: BodyHandle) {
            self.expect_add_body_and_return = Some((body, returned_value));
        }

        fn expect_body_and_return(&mut self, handle: BodyHandle, returned_value: PhysicalBody) {
            self.expect_body_and_return = Some((handle, returned_value));
        }

        fn expect_set_simulated_timestep(&mut self, timestep: f64) {
            self.expect_set_simulated_timestep = Some(timestep);
        }
    }

    impl Drop for WorldMock {
        fn drop(&mut self) {
            assert_eq!(self.expect_step, *self.step_was_called_with.borrow());
            assert_eq!(
                self.expect_add_body_and_return,
                *self.add_body_was_called_with.borrow()
            );
            assert_eq!(
                self.expect_body_and_return,
                *self.body_was_called_with.borrow()
            );
            assert_eq!(
                self.expect_set_simulated_timestep,
                *self.set_simulated_timestep_was_called_with.borrow()
            );
        }
    }

    impl World for WorldMock {
        fn step(&mut self) {}
        fn add_body(&mut self, body: PhysicalBody) -> BodyHandle {
            if let Some((expected_body, return_value)) = self.expect_add_body_and_return {
            } else {
                panic!("add_body was called unexpectedly")
            }
        }
        fn body(&self, handle: BodyHandle) -> PhysicalBody {}
        fn set_simulated_timestep(&mut self, timestep: f64) {}
    }
}
