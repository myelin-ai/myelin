use crate::object::{Object, ObjectId};
use std::fmt::Debug;

pub trait CollisionChecker {
    fn get_collisions(&self, id: ObjectId) -> Vec<&Object>;
}

pub trait ObjectContainer: Debug + CollisionChecker {
    fn add(&mut self, object: Object) -> ObjectId;
    fn remove(&mut self, id: ObjectId) -> Option<Object>;
    fn update(&mut self, id: ObjectId, object: Object) -> Option<Object>;
    fn get(&self, id: ObjectId) -> Option<&Object>;
}
