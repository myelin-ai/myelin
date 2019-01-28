use crate::Point;

pub(super) struct ConvexHull<'a> {
    vertices: &'a [Point],
    leftmost_point: &'a Point,
    current_point: &'a Point,
    state: ConvexHullState,
}

enum ConvexHullState {
    Initial,
    FindingNextVertex,
}

impl<'a> ConvexHull<'a> {
    pub(super) fn new(vertices: &'a [Point]) -> Result<Self, ()> {
        if vertices.is_empty() {
            Err(())
        } else {
            // Safe unwrap: A polygon's vertex should not be baloney like NaN
            let leftmost_point = vertices
                .iter()
                .min_by(|a, b| a.partial_cmp(&b).unwrap())
                .expect("At least one vertex must be given");
            Ok(Self {
                vertices,
                leftmost_point,
                current_point: leftmost_point,
                state: ConvexHullState::Initial,
            })
        }
    }
}

impl<'a> Iterator for ConvexHull<'a> {
    type Item = &'a Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ConvexHullState::Initial => {
                self.state = ConvexHullState::FindingNextVertex;
                Some(self.leftmost_point)
            }
            ConvexHullState::FindingNextVertex => self.find_next_vertex(),
        }
    }
}

impl<'a> ConvexHull<'a> {
    fn find_next_vertex(&mut self) -> Option<&'a Point> {
        let mut endpoint = self.vertices.first().unwrap();

        for vertex in self.vertices.iter().skip(1) {
            if endpoint == self.current_point
                || !is_counter_clockwise_turn(vertex, self.current_point, endpoint)
            {
                endpoint = vertex;
            }
        }

        self.current_point = endpoint;

        if self.leftmost_point == endpoint {
            None
        } else {
            Some(self.current_point)
        }
    }
}

fn is_counter_clockwise_turn(p1: &Point, p2: &Point, p3: &Point) -> bool {
    (p3.y - p1.y) * (p2.x - p1.x) >= (p2.y - p1.y) * (p3.x - p1.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_fails_with_zero_vertices() {
        assert!(ConvexHull::new(&[]).is_err());
    }

    #[test]
    fn convex_hull_iterator_works() {
        let vertices = vec![
            Point { x: 5.0, y: 5.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 15.0, y: 0.0 },
            Point { x: 20.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 },
        ];

        let expected_hull: Vec<_> = vertices.iter().collect();
        let hull: Vec<_> = ConvexHull::new(&vertices).unwrap().collect();

        assert_eq!(&expected_hull, &hull);
    }

    #[test]
    fn convex_hull_iterator_works_when_items_are_not_sorted() {
        let vertices = vec![
            Point { x: 20.0, y: 5.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 15.0, y: 0.0 },
            Point { x: 5.0, y: 5.0 },
        ];

        let expected_hull = vec![
            Point { x: 5.0, y: 5.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 15.0, y: 0.0 },
            Point { x: 20.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 },
        ];

        let hull: Vec<_> = ConvexHull::new(&vertices).unwrap().collect();

        assert_eq!(&expected_hull.iter().collect::<Vec<_>>(), &hull);
    }
}
