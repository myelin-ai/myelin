use crate::properties::Object;

pub mod physics_handler_facade;

pub trait PhysicsHandler {
    fn handle_collisions<'a>(&self, collisions: CollisionIterMut<'a>);
    fn apply_movement(&self, container: &mut [Object]);
}

#[derive(Debug)]
pub struct CollisionMut<'a> {
    pub first: &'a mut Object,
    pub second: &'a mut Object,
}

#[allow(missing_debug_implementations)]
pub struct CollisionIterMut<'a>(pub(crate) Box<dyn Iterator<Item = CollisionMut<'a>> + 'a>);

impl<'a> Iterator for CollisionIterMut<'a> {
    type Item = CollisionMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
