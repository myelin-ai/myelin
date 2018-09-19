use super::{ObjectDescription, Simulation};
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
    fn add_object_at(&mut self, object: Object, position: Position) {
        unimplemented!()
    }
    fn objects(&self) -> ObjectDescription {
        unimplemented!()
    }
    fn set_simulated_timestep(&mut self, timestep: f64) {
        unimplemented!()
    }
}

pub trait World: fmt::Debug {
    fn step(&mut self);
    fn add_rigid_body(&mut self, body: PhysicalBody) -> BodyHandle;
    fn add_grounded_body(&mut self, body: PhysicalBody) -> BodyHandle;
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
