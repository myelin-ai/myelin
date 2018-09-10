use crate::object::{Polygon, Vertex};

#[derive(Default, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<Vertex>,
}

impl PolygonBuilder {
    pub fn vertex(mut self, x: u32, y: u32) -> Self {
        self.vertices.push(Vertex { x, y });
        self
    }

    pub fn build(self) -> Polygon {
        Polygon {
            vertices: self.vertices,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_polygon_builder_works() {
        let polygon = PolygonBuilder::default()
            .vertex(0, 0)
            .vertex(0, 1)
            .vertex(1, 0)
            .vertex(1, 1)
            .build();

        let expected = Polygon {
            vertices: vec![
                Vertex { x: 0, y: 0 },
                Vertex { x: 0, y: 1 },
                Vertex { x: 1, y: 0 },
                Vertex { x: 1, y: 1 },
            ],
        };

        assert_eq!(expected, polygon);
    }
}
