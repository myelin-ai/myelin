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
