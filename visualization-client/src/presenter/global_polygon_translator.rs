use crate::view_model;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::translate::Translate;
use geo_types::Polygon as GeoPolygon;
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
                .map(|vertex| (vertex.x as f64, vertex.y as f64))
                .collect::<Vec<_>>()
                .into(),
            vec![],
        );
        let geo_polygon =
            geo_polygon.translate(position.location.x as f64, position.location.y as f64);
        let geo_polygon = geo_polygon.rotate(position.rotation.value());
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
                        x: 40.0 - 2.0,
                        y: 50.0 + 1.0,
                    },
                    view_model::Vertex {
                        x: 20.0 - 1.0,
                        y: 50.0 - 2.0,
                    },
                    view_model::Vertex {
                        x: 20.0 + 2.0,
                        y: 30.0 - 1.0,
                    },
                    view_model::Vertex {
                        x: 40.0 + 1.0,
                        y: 30.0 + 2.0,
                    },
                ],
            },
            translator.to_global_polygon(&polygon(), &position(Radians::try_new(3.0).unwrap()))
        );
    }
}
