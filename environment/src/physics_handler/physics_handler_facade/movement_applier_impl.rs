use super::MovementApplier;
use crate::properties::{Location, Object, Rectangle};

#[derive(Debug, Default)]
pub struct MovementApplierImpl;

impl MovementApplier for MovementApplierImpl {
    fn apply_movement(&self, container: &mut [Object]) {
        for object in container {
            let new_x = object.location.x as i32 + object.movement.x;
            let new_y = object.location.y as i32 + object.movement.y;

            if !is_valid_movement(new_x, new_y, &object.rectangle) {
                panic!(format!("Object moving out of bounds: {:#?}", object));
            }
            object.location = Location {
                x: new_x as u32,
                y: new_y as u32,
            };
        }
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
    use crate::properties::*;

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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());

        let expected_object = Object {
            location: Location { x: 16, y: 27 },
            ..original_object
        };
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
    }

    #[test]
    fn applies_movement_to_all_objects() {
        let original_object_one = Object {
            rectangle: Rectangle {
                width: 4,
                length: 5,
            },
            location: Location { x: 10, y: 20 },
            movement: MovementVector { x: 6, y: 7 },
            kind: Kind::Undefined,
        };

        let original_object_two = Object {
            rectangle: Rectangle {
                width: 2,
                length: 6,
            },
            location: Location { x: 120, y: 70 },
            movement: MovementVector { x: -52, y: 2 },
            kind: Kind::Undefined,
        };
        let mut container = vec![original_object_one.clone(), original_object_two.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);

        assert_eq!(2, container.len());

        let expected_object_one = Object {
            location: Location { x: 16, y: 27 },
            ..original_object_one
        };
        let actual_object_one = &container[0];
        assert_eq!(expected_object_one, *actual_object_one);

        let expected_object_two = Object {
            location: Location { x: 68, y: 72 },
            ..original_object_two
        };
        let actual_object_two = &container[1];
        assert_eq!(expected_object_two, *actual_object_two);
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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);
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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);
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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);
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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);
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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());

        let expected_object = Object {
            location: Location { x: 3, y: 2 },
            ..original_object
        };
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
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
        let mut container = vec![original_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());

        let expected_object = Object {
            location: Location { x: 3, y: 2 },
            ..original_object
        };
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
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
        let mut container = vec![expected_object.clone()];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);

        assert_eq!(1, container.len());
        let actual_object = &container[0];
        assert_eq!(expected_object, *actual_object);
    }

    #[test]
    fn ignores_empty_container() {
        let mut container = vec![];

        let physics_handler = MovementApplierImpl::default();
        physics_handler.apply_movement(&mut container);

        assert!(container.is_empty());
    }

}
