pub mod builder;

use crate::properties::{Location, Object, Rectangle};

#[derive(Debug, Eq, PartialEq)]
struct Bucket<'a> {
    rectangle: Rectangle,
    location: Location,
    objects: Vec<&'a Object>,
    buckets: Vec<Bucket<'a>>,
}

#[derive(Debug)]
pub struct QuadTree<'a> {
    root_bucket: Bucket<'a>,
}

#[cfg(test)]
mod test {
    use super::{builder::QuadTreeBuilder, Bucket, Object};
    use crate::properties::{Kind, Location, MovementVector, Rectangle};

    fn object_in_quadrant_one() -> Object {
        Object {
            location: Location::new(10, 10),
            rectangle: Rectangle::new(14, 14),
            movement: MovementVector::default(),
            kind: Kind::Organism,
        }
    }

    fn object_in_quadrant_two() -> Object {
        Object {
            location: Location::new(80, 30),
            rectangle: Rectangle::new(12, 12),
            movement: MovementVector::default(),
            kind: Kind::Organism,
        }
    }

    fn object_in_quadrant_three() -> Object {
        Object {
            location: Location::new(10, 60),
            rectangle: Rectangle::new(14, 14),
            movement: MovementVector::default(),
            kind: Kind::Organism,
        }
    }

    fn object_in_quadrant_four() -> Object {
        Object {
            location: Location::new(70, 70),
            rectangle: Rectangle::new(14, 14),
            movement: MovementVector::default(),
            kind: Kind::Organism,
        }
    }

    #[test]
    fn test_quadtree_splits_on_bucket_capacity() {
        const WORLD_SIZE: u32 = 100;
        const FIRST_SUBBUCKET_SIZE: u32 = 50;

        let obj1 = object_in_quadrant_one();
        let obj2 = object_in_quadrant_two();
        let obj3 = object_in_quadrant_three();
        let obj4 = object_in_quadrant_four();

        let expected = Bucket {
            rectangle: Rectangle::new(WORLD_SIZE, WORLD_SIZE),
            location: Location::new(0, 0),
            objects: vec![],
            buckets: vec![
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(0, 0),
                    objects: vec![&obj1],
                    buckets: vec![],
                },
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(FIRST_SUBBUCKET_SIZE, 0),
                    objects: vec![&obj2],
                    buckets: vec![],
                },
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(0, FIRST_SUBBUCKET_SIZE),
                    objects: vec![&obj3],
                    buckets: vec![],
                },
                Bucket {
                    rectangle: Rectangle::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    location: Location::new(FIRST_SUBBUCKET_SIZE, FIRST_SUBBUCKET_SIZE),
                    objects: vec![&obj4],
                    buckets: vec![],
                },
            ],
        };

        let container = vec![
            object_in_quadrant_one(),
            object_in_quadrant_two(),
            object_in_quadrant_three(),
            object_in_quadrant_four(),
        ];

        let quad_tree = QuadTreeBuilder::default().split_at(4).build(&container);

        assert_eq!(expected, quad_tree.root_bucket);
    }

    #[test]
    fn test_quadtree_doesnt_split_on_bucket_capacity_minus_one() {
        const WORLD_SIZE: u32 = 100;

        let obj1 = object_in_quadrant_one();
        let obj2 = object_in_quadrant_two();
        let obj3 = object_in_quadrant_three();

        let expected = Bucket {
            rectangle: Rectangle::new(WORLD_SIZE, WORLD_SIZE),
            location: Location::new(0, 0),
            objects: vec![&obj1, &obj2, &obj3],
            buckets: vec![],
        };

        let container = vec![
            object_in_quadrant_one(),
            object_in_quadrant_two(),
            object_in_quadrant_three(),
        ];

        let quad_tree = QuadTreeBuilder::default().split_at(4).build(&container);

        assert_eq!(expected, quad_tree.root_bucket);
    }

    #[test]
    fn test_quadtree_only_splits_container_with_contents() {}
}
