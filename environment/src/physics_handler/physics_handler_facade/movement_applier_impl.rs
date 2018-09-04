use super::MovementApplier;
use crate::object::{Location, Object, Rectangle};

#[derive(Debug, Default)]
pub struct MovementApplierImpl;

impl MovementApplierImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl MovementApplier for MovementApplierImpl {
    fn apply_movement(&self, object: &Object) -> Object {
        let new_x = object.location.x as i32 + object.movement.x;
        let new_y = object.location.y as i32 + object.movement.y;

        if !is_valid_movement(new_x, new_y, &object.rectangle) {
            panic!(format!("Object moving out of bounds: {:#?}", object));
        }
        let mut new_position = object.clone();
        new_position.location = Location {
            x: new_x as u32,
            y: new_y as u32,
        };
        new_position
    }
}

fn is_valid_movement(new_x: i32, new_y: i32, rectangle: &Rectangle) -> bool {
    let left_edge = new_x - (rectangle.width as f32 / 2.0).round() as i32;
    let top_edge = new_y - (rectangle.length as f32 / 2.0).round() as i32;
    left_edge >= 0 && top_edge >= 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::*;

    #[test]
    fn applies_movement() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 6,
                length: 5,
            },
            location: Location { x: 10, y: 20 },
            movement: MovementVector { x: 6, y: 7 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        let new_object = physics_handler.apply_movement(&original_object);

        let expected_object = Object {
            location: Location { x: 16, y: 27 },
            ..original_object
        };
        assert_eq!(expected_object, new_object);
    }

    #[test]
    #[should_panic]
    fn panics_when_moving_out_of_bounds() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 4,
                length: 5,
            },
            location: Location { x: 10, y: 20 },
            movement: MovementVector { x: -30, y: -20 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        physics_handler.apply_movement(&original_object);
    }

    #[test]
    #[should_panic]
    fn panics_when_rectangle_clips_into_negative_position() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 6,
                length: 4,
            },
            location: Location { x: 4, y: 2 },
            movement: MovementVector { x: -2, y: 0 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        physics_handler.apply_movement(&original_object);
    }

    #[test]
    #[should_panic]
    fn panics_when_rectangle_clips_into_negative_position_without_movement() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 4,
                length: 6,
            },
            location: Location { x: 3, y: 2 },
            movement: MovementVector { x: 0, y: 0 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        physics_handler.apply_movement(&original_object);
    }

    #[test]
    #[should_panic]
    fn panics_when_rectangle_clips_into_negative_position_by_rounding_up() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 7,
                length: 4,
            },
            location: Location { x: 4, y: 2 },
            movement: MovementVector { x: -1, y: 0 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        physics_handler.apply_movement(&original_object);
    }

    #[test]
    fn allows_object_on_exact_origin() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 6,
                length: 4,
            },
            location: Location { x: 4, y: 2 },
            movement: MovementVector { x: -1, y: 0 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        let actual_object = physics_handler.apply_movement(&original_object);

        let expected_object = Object {
            location: Location { x: 3, y: 2 },
            ..original_object
        };
        assert_eq!(expected_object, actual_object);
    }

    #[test]
    fn allows_object_on_exact_origin_by_rounding_down() {
        let original_object = Object {
            rectangle: Rectangle {
                width: 5,
                length: 4,
            },
            location: Location { x: 4, y: 2 },
            movement: MovementVector { x: -1, y: 0 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        let actual_object = physics_handler.apply_movement(&original_object);

        let expected_object = Object {
            location: Location { x: 3, y: 2 },
            ..original_object
        };

        assert_eq!(expected_object, actual_object);
    }

    #[test]
    fn ignores_zeroed_movement_vector() {
        let expected_object = Object {
            rectangle: Rectangle {
                width: 4,
                length: 6,
            },
            location: Location { x: 40, y: 22 },
            movement: MovementVector { x: 0, y: 0 },
            kind: Kind::Undefined,
        };

        let physics_handler = MovementApplierImpl::new();
        let actual_object = physics_handler.apply_movement(&expected_object);

        assert_eq!(expected_object, actual_object);
    }
}
