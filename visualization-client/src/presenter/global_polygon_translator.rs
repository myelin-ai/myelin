use crate::view_model;
use myelin_geometry;
use myelin_geometry::polygon::{Polygon, PolygonBuilder};
use std::fmt::Debug;
use std::marker::PhantomData;

pub(crate) trait GlobalPolygonTranslator: Debug {
    fn to_global_polygon(
        &self,
        polygon: &myelin_geometry::polygon::Polygon,
        position: &myelin_geometry::Position,
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
        polygon: &myelin_geometry::polygon::Polygon,
        position: &myelin_geometry::Position,
    ) -> view_model::Polygon {
        let global_polygon = polygon
            .translate(position.location)
            .rotate_around_point(position.rotation, position.location);
        let view_model_vertices = global_polygon
            .vertices
            .iter()
            .map(|vertex| view_model::Point {
                x: vertex.x,
                y: vertex.y,
            })
            .collect();

        view_model::Polygon {
            vertices: view_model_vertices,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use myelin_geometry::polygon::*;
    use myelin_geometry::*;
    use std::f64::consts::PI;

    fn polygon() -> Polygon {
        PolygonBuilder::default()
            .vertex(-10.0, -10.0)
            .vertex(10.0, -10.0)
            .vertex(10.0, 10.0)
            .vertex(-10.0, 10.0)
            .build()
            .unwrap()
    }

    fn position(rotation: Radians) -> Position {
        Position {
            location: Point { x: 30.0, y: 40.0 },
            rotation,
        }
    }

    #[test]
    fn converts_to_global_object_with_no_orientation() {
        let translator = GlobalPolygonTranslatorImpl::new();

        assert_eq!(
            view_model::Polygon {
                vertices: vec![
                    view_model::Point { x: 20.0, y: 30.0 },
                    view_model::Point { x: 40.0, y: 30.0 },
                    view_model::Point { x: 40.0, y: 50.0 },
                    view_model::Point { x: 20.0, y: 50.0 },
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
                    view_model::Point { x: 40.0, y: 50.0 },
                    view_model::Point { x: 20.0, y: 50.0 },
                    view_model::Point { x: 20.0, y: 30.0 },
                    view_model::Point { x: 40.0, y: 30.0 },
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
                    view_model::Point {
                        x: 41.311125046603124,
                        y: 48.48872488540578
                    },
                    view_model::Point {
                        x: 21.51127511459422,
                        y: 51.311125046603124
                    },
                    view_model::Point {
                        x: 18.688874953396876,
                        y: 31.51127511459422
                    },
                    view_model::Point {
                        x: 38.48872488540578,
                        y: 28.688874953396876
                    },
                ],
            },
            translator.to_global_polygon(&polygon(), &position(Radians::try_new(3.0).unwrap()))
        );
    }
}
