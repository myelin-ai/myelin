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
                x: point.x().round() as u32,
                y: point.y().round() as u32,
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
