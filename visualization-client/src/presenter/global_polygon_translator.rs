use crate::view_model;
use myelin_environment::object as business_object;
use std::fmt::Debug;
use std::marker::PhantomData;

pub(crate) trait GlobalPolygonTranslator: Debug {
    fn to_global_polygon(
        &self,
        polygon: &business_object::Polygon,
        position: &business_object::Position,
    ) -> view_model::Polygon;
}

#[derive(Debug)]
pub(crate) struct GlobalPolygonTranslatorImpl(PhantomData<()>);

impl GlobalPolygonTranslatorImpl {
    pub(crate) fn new() -> Self {
        GlobalPolygonTranslatorImpl(PhantomData)
    }
}

impl GlobalPolygonTranslator for GlobalPolygonTranslatorImpl {
    fn to_global_polygon(
        &self,
        polygon: &business_object::Polygon,
        position: &business_object::Position,
    ) -> view_model::Polygon {
        view_model::Polygon {
            vertices: polygon
                .vertices
                .iter()
                .map(|vertex| to_global_rotated_vertex(vertex, position))
                .collect(),
        }
    }
}

fn to_global_rotated_vertex(
    vertex: &business_object::Vertex,
    position: &business_object::Position,
) -> view_model::Vertex {
    // See https://en.wikipedia.org/wiki/Rotation_matrix
    let center_x = f64::from(position.location.x);
    let center_y = f64::from(position.location.y);
    let rotation = position.rotation.value();
    let global_x = center_x + f64::from(vertex.x);
    let global_y = center_y + f64::from(vertex.y);
    let rotated_global_x =
        rotation.cos() * (global_x - center_x) + rotation.sin() * (global_y - center_y) + center_x;
    let rotated_global_y =
        -rotation.sin() * (global_x - center_x) + rotation.cos() * (global_y - center_y) + center_y;

    view_model::Vertex {
        x: rotated_global_x.round() as u32,
        y: rotated_global_y.round() as u32,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::PolygonBuilder;
    use std::f64::consts::PI;

    fn polygon() -> Polygon {
        PolygonBuilder::new()
            .vertex(-10, -10)
            .vertex(10, -10)
            .vertex(10, 10)
            .vertex(-10, 10)
            .build()
            .unwrap()
    }

    fn position(rotation: Radians) -> Position {
        Position {
            location: Location { x: 30, y: 40 },
            rotation,
        }
    }

    #[test]
    fn converts_to_global_object_with_no_orientation() {
        let translator = GlobalPolygonTranslatorImpl::new();

        assert_eq!(
            view_model::Polygon {
                vertices: vec![
                    view_model::Vertex { x: 20, y: 30 },
                    view_model::Vertex { x: 40, y: 30 },
                    view_model::Vertex { x: 40, y: 50 },
                    view_model::Vertex { x: 20, y: 50 },
                ],
            },
            translator.to_global_polygon(&polygon(), &position(Radians::default()))
        );
    }

    #[test]
    fn converts_to_global_object_with_pi_orientation() {
        let translator = GlobalPolygonTranslatorImpl::new();

        assert_eq!(
            view_model::Polygon {
                vertices: vec![
                    view_model::Vertex { x: 40, y: 50 },
                    view_model::Vertex { x: 20, y: 50 },
                    view_model::Vertex { x: 20, y: 30 },
                    view_model::Vertex { x: 40, y: 30 },
                ],
            },
            translator.to_global_polygon(&polygon(), &position(Radians::try_new(PI).unwrap()))
        );
    }

    #[test]
    fn converts_to_global_object_with_arbitrary_orientation() {
        let translator = GlobalPolygonTranslatorImpl::new();

        assert_eq!(
            view_model::Polygon {
                vertices: vec![
                    view_model::Vertex {
                        x: 40 - 2,
                        y: 50 + 1,
                    },
                    view_model::Vertex {
                        x: 20 - 1,
                        y: 50 - 2,
                    },
                    view_model::Vertex {
                        x: 20 + 2,
                        y: 30 - 1,
                    },
                    view_model::Vertex {
                        x: 40 + 1,
                        y: 30 + 2,
                    },
                ],
            },
            translator.to_global_polygon(&polygon(), &position(Radians::try_new(3.0).unwrap()))
        );
    }
}
