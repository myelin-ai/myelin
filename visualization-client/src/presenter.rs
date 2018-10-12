use crate::view_model;
use myelin_environment::object as business_object;

struct Presenter {}

fn to_global_object(object: &business_object::ObjectDescription) -> view_model::Object {
    view_model::Object {
        shape: view_model::Polygon {
            vertices: object
                .shape
                .vertices
                .iter()
                .map(|vertex| to_global_rotated_vertex(vertex, object))
                .collect(),
        },
        kind: map_kind(object.kind),
    }
}

fn to_global_rotated_vertex(
    vertex: &business_object::Vertex,
    object: &business_object::ObjectDescription,
) -> view_model::Vertex {
    // algorithm source: https://stackoverflow.com/questions/786472/rotate-a-point-by-another-point-in-2d/786508#786508
    let center_x = f64::from(object.position.location.x);
    let center_y = f64::from(object.position.location.y);
    let rotation = object.position.rotation.0;
    let global_x = center_x + f64::from(vertex.x);
    let global_y = center_y + f64::from(vertex.y);
    let rotated_global_x =
        rotation.cos() * (global_x - center_x) - rotation.sin() * (global_y - center_y) + center_x;
    let rotated_global_y =
        rotation.sin() * (global_x - center_x) + rotation.cos() * (global_y - center_y) + center_y;

    view_model::Vertex {
        x: rotated_global_x.round() as u32,
        y: rotated_global_y.round() as u32,
    }
}

fn map_kind(kind: business_object::Kind) -> view_model::Kind {
    match kind {
        business_object::Kind::Organism => view_model::Kind::Organism,
        business_object::Kind::Plant => view_model::Kind::Plant,
        business_object::Kind::Water => view_model::Kind::Water,
        business_object::Kind::Terrain => view_model::Kind::Terrain,
    }
}
