use crate::object::{Object, Polygon};

pub trait PolygonTranslator {
    fn global_vertices(&self, object: &Object) -> Polygon;
}

pub struct NAlgebraPolygonTranslator;

impl PolygonTranslator for NAlgebraPolygonTranslator {
    fn global_vertices(&self, object: &Object) -> Polygon {
        let vertices: Vec<_> = object
            .shape
            .vertices
            .iter()
            .map(|vertex| vertex.clone())
            .collect();
        Polygon { vertices }
    }
}
