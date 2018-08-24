use crate::properties::{Id, Object};

pub trait ObjectContainer {
    fn objects(&self) -> Vec<Box<dyn Object>>;
    fn add_object(&mut self, object: Box<dyn Object>) -> Id;
    fn remove_object(&mut self, id: Id);
    fn update_object(&mut self, object: Box<dyn Object>);
}
