use crate::simulation::Presenter;
use crate::view_model::{self, ViewModel};
use myelin_environment::object as business_object;
use myelin_environment::polygon_translator::PolygonTranslator;

pub(crate) trait View {
    fn draw_objects(&self, view_model: &ViewModel);
}

pub(crate) struct CanvasPresenter {
    view: Box<View>,
    polygon_translator: Box<PolygonTranslator>,
}

impl Presenter for CanvasPresenter {
    fn present_objects(&self, objects: &[business_object::Object]) {
        let view_model = ViewModel {
            objects: objects
                .iter()
                .map(|object| self.business_objects_to_view_model_object(object))
                .collect(),
        };

        self.view.draw_objects(&view_model);
    }
}

fn map_kind(kind: &business_object::Kind) -> view_model::Kind {
    match *kind {
        business_object::Kind::Organism => view_model::Kind::Organism,
        business_object::Kind::Plant => view_model::Kind::Plant,
        business_object::Kind::Water => view_model::Kind::Water,
        business_object::Kind::Terrain => view_model::Kind::Terrain,
    }
}

impl CanvasPresenter {
    pub(crate) fn new(view: Box<View>, polygon_translator: Box<PolygonTranslator>) -> Self {
        Self {
            view,
            polygon_translator,
        }
    }

    fn business_objects_to_view_model_object(
        &self,
        object: &business_object::Object,
    ) -> view_model::Object {
        view_model::Object {
            body: view_model::Polygon {
                vertices: self.polygon_translator.global_polygon(object),
            },
            kind: map_kind(&object.kind),
        }
    }
}
