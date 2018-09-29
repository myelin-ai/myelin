//! This module contains a simulated [`World`] and its implementations,
//! in which one can place [`Objects`] in order for them to be influenced
//! by physics.
//!
//! [`World`]: ./trait.World.html
//! [`Objects`]: ../object/struct.Body.html
use super::{BodyHandle, PhysicalBody, World};
use crate::object::*;
use nalgebra::base::{Scalar, Vector2};
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use ncollide2d::world::CollisionObjectHandle;
use nphysics2d::math::{Isometry, Point, Vector};
use nphysics2d::object::{
    BodyHandle as NphysicsBodyHandle, Collider, ColliderHandle, Material, RigidBody,
};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World as PhysicsWorld;
use std::collections::HashMap;
use std::fmt;

type PhysicsType = f64;

pub mod rotation_translator;

/// An implementation of [`World`] that uses nphysics
/// in the background.
///
/// [`World`]: ./trait.World.html
pub struct NphysicsWorld {
    physics_world: PhysicsWorld<PhysicsType>,
    collider_handles: HashMap<ColliderHandle, Kind>,
    rotation_translator: Box<dyn NphysicsRotationTranslator>,
}

impl NphysicsWorld {
    /// Instantiates a new empty world
    /// # Examples
    /// ```
    /// use myelin_environment::simulation_impl::world::NphysicsWorld;
    /// use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
    ///
    /// let rotation_translator = NphysicsRotationTranslatorImpl {};
    /// let mut world = NphysicsWorld::with_timestep(1.0, Box::new(rotation_translator));
    /// ```
    pub fn with_timestep(
        timestep: f64,
        rotation_translator: Box<dyn NphysicsRotationTranslator>,
    ) -> Self {
        let mut physics_world = PhysicsWorld::new();

        physics_world.set_timestep(timestep);

        Self {
            physics_world,
            collider_handles: HashMap::new(),
            rotation_translator,
        }
    }

    fn get_body_from_handle(&self, collider_handle: ColliderHandle) -> Option<PhysicalBody> {
        let collider = self.physics_world.collider(collider_handle)?;

        let shape = self.get_shape(&collider);
        let position = self.get_position(&collider);
        let mobility = self.get_mobility(&collider);

        Some(PhysicalBody {
            shape,
            position,
            mobility,
        })
    }

    fn get_shape(&self, collider: &Collider<PhysicsType>) -> Polygon {
        let convex_polygon: &ConvexPolygon<_> = collider
            .shape()
            .as_shape()
            .expect("Failed to cast shape to a ConvexPolygon");
        let vertices: Vec<_> = convex_polygon
            .points()
            .iter()
            .map(|vertex| Vertex {
                x: vertex.x.round() as i32,
                y: vertex.y.round() as i32,
            })
            .collect();
        Polygon { vertices }
    }

    fn get_mobility(&self, collider: &Collider<PhysicsType>) -> Mobility {
        let body_handle = collider.data().body();
        if body_handle.is_ground() {
            Mobility::Immovable
        } else {
            let rigid_body = self
                .physics_world
                .rigid_body(body_handle)
                .expect("Body handle did not correspond to any rigid body");

            let linear_velocity = rigid_body.velocity().linear;
            let (x, y) = elements(&linear_velocity);
            Mobility::Movable(Velocity {
                x: x as i32,
                y: y as i32,
            })
        }
    }

    fn get_position(&self, collider: &Collider<PhysicsType>) -> Position {
        let position = collider.position();
        let (x, y) = elements(&position.translation.vector);
        let rotation = position.rotation.angle();

        Position {
            location: Location {
                x: x as u32,
                y: y as u32,
            },
            rotation: self.rotation_translator.to_radians(rotation),
        }
    }
}

/// This trait translates the rotation from [`Radians`] to the range (-π; π] defined by nphysics
///
/// [`Radians`]: ../../object/struct.Radians.html
pub trait NphysicsRotationTranslator: fmt::Debug {
    fn to_nphysics_rotation(&self, orientation: Radians) -> f64;
    fn to_radians(&self, nphysics_rotation: f64) -> Radians;
}

fn elements<N>(vector: &Vector2<N>) -> (N, N)
where
    N: Scalar,
{
    let mut iter = vector.iter();

    (*iter.next().unwrap(), *iter.next().unwrap())
}

fn get_isometry(
    body: &PhysicalBody,
    rotation_translator: &dyn NphysicsRotationTranslator,
) -> Isometry<PhysicsType> {
    Isometry::new(
        Vector::new(
            PhysicsType::from(body.position.location.x),
            PhysicsType::from(body.position.location.y),
        ),
        rotation_translator.to_nphysics_rotation(body.position.rotation),
    )
}

fn get_shape(body: &PhysicalBody) -> ShapeHandle<PhysicsType> {
    let points: Vec<_> = body
        .shape
        .vertices
        .iter()
        .map(|vertex| Point::new(PhysicsType::from(vertex.x), PhysicsType::from(vertex.y)))
        .collect();

    ShapeHandle::new(ConvexPolygon::try_new(points).expect("Polygon was not convex"))
}

impl World for NphysicsWorld {
    fn step(&mut self) {
        self.physics_world.step();
    }

    fn add_body(&mut self, body: PhysicalBody) -> BodyHandle {
        let shape = get_shape(&body);
        let local_inertia = shape.inertia(0.1);
        let local_center_of_mass = shape.center_of_mass();
        let isometry = get_isometry(&body, &*self.rotation_translator);
        let material = Material::default();

        let handle = match body.mobility {
            Mobility::Immovable => self.physics_world.add_collider(
                0.04,
                shape,
                NphysicsBodyHandle::ground(),
                isometry,
                material,
            ),
            Mobility::Movable(velocity) => {
                let rigid_body_handle = self.physics_world.add_rigid_body(
                    isometry,
                    local_inertia,
                    local_center_of_mass,
                );
                let mut rigid_body = self
                    .physics_world
                    .rigid_body_mut(rigid_body_handle)
                    .expect("Invalid body handle");
                set_velocity(&mut rigid_body, &velocity);
                self.physics_world.add_collider(
                    0.04,
                    shape,
                    rigid_body_handle,
                    Isometry::identity(),
                    material,
                )
            }
        };

        to_object_handle(handle)
    }

    fn body(&self, handle: BodyHandle) -> Option<PhysicalBody> {
        let collider_handle = to_collider_handle(handle);
        self.get_body_from_handle(collider_handle)
    }

    fn set_simulated_timestep(&mut self, timestep: f64) {
        self.physics_world.set_timestep(timestep);
    }
}

fn set_velocity(rigid_body: &mut RigidBody<PhysicsType>, velocity: &Velocity) {
    let nphysics_velocity = nphysics2d::algebra::Velocity2::linear(
        PhysicsType::from(velocity.x),
        PhysicsType::from(velocity.y),
    );
    rigid_body.set_velocity(nphysics_velocity);
}

fn to_object_handle(collider_handle: ColliderHandle) -> BodyHandle {
    BodyHandle(collider_handle.uid())
}

fn to_collider_handle(object_handle: BodyHandle) -> ColliderHandle {
    CollisionObjectHandle(object_handle.0)
}

impl fmt::Debug for NphysicsWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NphysicsWorld")
            .field("collider_handles", &self.collider_handles)
            .field("physics", &DebugPhysicsWorld(&self.physics_world))
            .finish()
    }
}

/// A helper struct used to implement [`std::fmt::Debug`]
/// for [`NphysicsWorld`]
///
/// [`std::fmt::Debug`]: https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html
/// [`NphysicsWorld`]: ./struct.NphysicsWorld.html
struct DebugPhysicsWorld<'a>(&'a PhysicsWorld<PhysicsType>);

impl<'a> fmt::Debug for DebugPhysicsWorld<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicsWorld").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_builder::PolygonBuilder;
    use std::cell::RefCell;

    const DEFAULT_TIMESTEP: f64 = 1.0;

    fn movable_body(orientation: Radians) -> PhysicalBody {
        PhysicalBody {
            position: Position {
                location: Location { x: 5, y: 5 },
                rotation: orientation,
            },
            mobility: Mobility::Movable(Velocity { x: 1, y: 1 }),
            shape: PolygonBuilder::new()
                .vertex(-5, -5)
                .vertex(-5, 5)
                .vertex(5, 5)
                .vertex(5, -5)
                .build()
                .unwrap(),
        }
    }

    fn immovable_body(orientation: Radians) -> PhysicalBody {
        PhysicalBody {
            shape: PolygonBuilder::new()
                .vertex(-100, -100)
                .vertex(100, -100)
                .vertex(100, 100)
                .vertex(-100, 100)
                .build()
                .unwrap(),
            mobility: Mobility::Immovable,
            position: Position {
                location: Location { x: 300, y: 200 },
                rotation: orientation,
            },
        }
    }

    #[test]
    fn panics_on_invalid_handle() {
        let rotation_translator = NphysicsRotationTranslatorMock::default();
        let world = NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let body = world.body(BodyHandle(1337));
        assert!(body.is_none())
    }

    #[test]
    fn can_return_rigid_object_with_valid_handle() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let movable_body = movable_body(Radians(3.0));

        let handle = world.add_body(movable_body);
        world.body(handle);
    }

    #[test]
    fn can_return_grounded_object_with_valid_handle() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let body = immovable_body(Radians(3.0));

        let handle = world.add_body(body);
        world.body(handle);
    }

    #[test]
    fn can_return_mixed_objects_with_valid_handles() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let rigid_object = movable_body(Radians(3.0));
        let grounded_object = immovable_body(Radians(3.0));

        let rigid_handle = world.add_body(rigid_object);
        let grounded_handle = world.add_body(grounded_object);

        let _rigid_body = world.body(rigid_handle);
        let _grounded_body = world.body(grounded_handle);
    }

    #[test]
    fn returns_correct_rigid_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let expected_body = movable_body(Radians(3.0));
        let handle = world.add_body(expected_body.clone());
        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn returns_correct_grounded_body() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(3.0), 3.0);
        rotation_translator.expect_to_radians_and_return(3.0, Radians(3.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let expected_body = immovable_body(Radians(3.0));
        let handle = world.add_body(expected_body.clone());
        let actual_body = world.body(handle);

        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn timestep_is_respected() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(0.0), 0.0);
        rotation_translator.expect_to_radians_and_return(0.0, Radians(0.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));

        let local_object = movable_body(Radians::default());
        let handle = world.add_body(local_object.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            position: Position {
                location: Location { x: 6, y: 6 },
                rotation: Radians::default(),
            },
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn timestep_can_be_changed() {
        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(0.0), 0.0);
        rotation_translator.expect_to_radians_and_return(0.0, Radians(0.0));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        world.set_simulated_timestep(2.0);

        let local_object = movable_body(Radians::default());
        let handle = world.add_body(local_object.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);

        let expected_body = PhysicalBody {
            position: Position {
                location: Location { x: 7, y: 7 },
                rotation: Radians::default(),
            },
            ..local_object
        };
        assert_eq!(Some(expected_body), actual_body);
    }

    #[test]
    fn step_is_ignored_for_rigid_objects_with_no_movement() {
        use std::f64::consts::FRAC_PI_2;

        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(FRAC_PI_2), FRAC_PI_2);
        rotation_translator.expect_to_radians_and_return(FRAC_PI_2, Radians(FRAC_PI_2));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let expected_body = immovable_body(Radians(FRAC_PI_2));
        let handle = world.add_body(expected_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(expected_body), actual_body)
    }

    #[test]
    fn step_is_ignored_for_grounded_objects() {
        use std::f64::consts::FRAC_PI_2;

        let mut rotation_translator = NphysicsRotationTranslatorMock::default();
        rotation_translator.expect_to_nphysics_rotation_and_return(Radians(FRAC_PI_2), FRAC_PI_2);
        rotation_translator.expect_to_radians_and_return(FRAC_PI_2, Radians(FRAC_PI_2));
        let mut world =
            NphysicsWorld::with_timestep(DEFAULT_TIMESTEP, Box::new(rotation_translator));
        let body = immovable_body(Radians(FRAC_PI_2));
        let still_body = PhysicalBody {
            mobility: Mobility::Movable(Velocity { x: 0, y: 0 }),
            ..body
        };
        let handle = world.add_body(still_body.clone());

        world.step();
        world.step();

        let actual_body = world.body(handle);
        assert_eq!(Some(still_body), actual_body)
    }

    #[derive(Debug, Default)]
    struct NphysicsRotationTranslatorMock {
        expect_to_nphysics_rotation_and_return: Option<(Radians, f64)>,
        expect_to_radians_and_return: Option<(f64, Radians)>,

        to_nphysics_rotation_was_called: RefCell<bool>,
        to_radians_was_called: RefCell<bool>,
    }

    impl NphysicsRotationTranslatorMock {
        fn expect_to_nphysics_rotation_and_return(
            &mut self,
            input_value: Radians,
            return_value: f64,
        ) {
            self.expect_to_nphysics_rotation_and_return = Some((input_value, return_value))
        }

        fn expect_to_radians_and_return(&mut self, input_value: f64, return_value: Radians) {
            self.expect_to_radians_and_return = Some((input_value, return_value))
        }
    }

    impl NphysicsRotationTranslator for NphysicsRotationTranslatorMock {
        fn to_nphysics_rotation(&self, orientation: Radians) -> f64 {
            *self.to_nphysics_rotation_was_called.borrow_mut() = true;

            if let Some((expected_input, expected_output)) =
                self.expect_to_nphysics_rotation_and_return
            {
                if orientation != expected_input {
                    panic!("to_nphysics_rotation() was called with an unexpected input value: {:?}")
                }

                expected_output
            } else {
                panic!("to_nphysics_rotation() was called unexpectedly")
            }
        }

        fn to_radians(&self, nphysics_rotation: f64) -> Radians {
            *self.to_radians_was_called.borrow_mut() = true;

            if let Some((expected_input, expected_output)) = self.expect_to_radians_and_return {
                if nphysics_rotation != expected_input {
                    panic!("to_radians() was called with an unexpected input value: {:?}")
                }

                expected_output
            } else {
                panic!("to_radians() was called unexpectedly")
            }
        }
    }

    impl Drop for NphysicsRotationTranslatorMock {
        fn drop(&mut self) {
            if self.expect_to_nphysics_rotation_and_return.is_some() {
                assert!(
                    *self.to_nphysics_rotation_was_called.borrow(),
                    "to_nphysics_rotation() was not called, but was expected"
                )
            }
            if self.expect_to_radians_and_return.is_some() {
                assert!(
                    *self.to_radians_was_called.borrow(),
                    "to_radians() was not called, but was expected"
                )
            }
        }
    }

}
