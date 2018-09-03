use crate::object::{Id, Object};
use std::fmt::Debug;

pub trait CollisionChecker {
    fn get_collisions(&self, id: Id) -> Vec<&Object>;
}

pub trait ObjectContainer: Debug + CollisionChecker {
    fn add(&mut self, object: Object) -> Id;
    fn remove(&mut self, id: Id) -> Option<Object>;
    fn update(&mut self, id: Id, object: Object) -> Option<Object>;
    fn get(&self, id: Id) -> Option<&Object>;
}
