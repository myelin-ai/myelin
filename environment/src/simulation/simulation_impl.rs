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
    use std::cell::RefCell;

    #[derive(Debug)]
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

    impl World for WorldMock {
        fn step(&mut self) {}
        fn add_body(&mut self, body: PhysicalBody) -> BodyHandle {}
        fn body(&self, handle: BodyHandle) -> PhysicalBody {}
        fn set_simulated_timestep(&mut self, timestep: f64) {}
    }
}
