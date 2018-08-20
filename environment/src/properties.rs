pub trait Locatable {
    fn x(&self) -> u32;
    fn y(&self) -> u32;
}

pub trait Rectangle {
    fn length(&self) -> u32;
    fn width(&self) -> u32;
}

pub struct Vector {
    pub x: i32,
    pub y: i32,
}

pub enum SpawnEvent {
    Spawn(Box<dyn Collidable>),
    Despawn(usize),
    Move(usize, Vector),
}

pub trait Collidable: Locatable + Rectangle {
    fn id(&self) -> usize;
    fn on_collision(
        &self,
        other: &dyn Collidable,
        movement: Option<Vector>,
    ) -> Vec<SpawnEvent>;
}
