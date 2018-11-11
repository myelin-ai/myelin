use crate::view_model;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::rotate::RotatePoint;
use geo::algorithm::translate::Translate;
use geo_types::{Point, Polygon as GeoPolygon};
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
        let geo_polygon = GeoPolygon::new(
            polygon
                .vertices
                .iter()
                .map(|vertex| (vertex.x, vertex.y))
                .collect::<Vec<_>>()
                .into(),
            vec![],
        );
        let geo_polygon = geo_polygon.translate(position.location.x, position.location.y);
        let rotation_angle_in_degrees = position.rotation.value().to_degrees();
        let center = Point::new(position.location.x, position.location.y);
        let geo_polygon = geo_polygon.rotate_around_point(rotation_angle_in_degrees, center);
        let vertices = geo_polygon
            .exterior
            .into_points()
            .iter()
            .map(|point| view_model::Vertex {
                x: point.x(),
                y: point.y(),
            })
            .collect();
        view_model::Polygon { vertices }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::PolygonBuilder;
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
            location: Location { x: 30.0, y: 40.0 },
            rotation,
        }
    }

    #[test]
    fn converts_to_global_object_with_no_orientation() {
        let translator = GlobalPolygonTranslatorImpl::new();

        assert_eq!(
            view_model::Polygon {
                vertices: vec![
                    view_model::Vertex { x: 20.0, y: 30.0 },
                    view_model::Vertex { x: 40.0, y: 30.0 },
                    view_model::Vertex { x: 40.0, y: 50.0 },
                    view_model::Vertex { x: 20.0, y: 50.0 },
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
                    view_model::Vertex { x: 40.0, y: 50.0 },
                    view_model::Vertex { x: 20.0, y: 50.0 },
                    view_model::Vertex { x: 20.0, y: 30.0 },
                    view_model::Vertex { x: 40.0, y: 30.0 },
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
                        x: 41.311125046603124,
                        y: 48.48872488540578
                    },
                    view_model::Vertex {
                        x: 21.51127511459422,
                        y: 51.311125046603124
                    },
                    view_model::Vertex {
                        x: 18.688874953396876,
                        y: 31.51127511459422
                    },
                    view_model::Vertex {
                        x: 38.48872488540578,
                        y: 28.688874953396876
                    },
                ],
            },
            translator.to_global_polygon(&polygon(), &position(Radians::try_new(3.0).unwrap()))
        );
    }
}
