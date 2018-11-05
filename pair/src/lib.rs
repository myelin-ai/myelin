//! This crate provides a tuple struct for an unordered pair

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A tuple struct representing an unordered pair
#[derive(Debug, Clone, Eq)]
pub struct UnorderedPair<T>(pub T, pub T);

impl<T> From<(T, T)> for UnorderedPair<T> {
    fn from(tuple: (T, T)) -> UnorderedPair<T> {
        UnorderedPair(tuple.0, tuple.1)
    }
}

impl<T> Into<(T, T)> for UnorderedPair<T> {
    fn into(self) -> (T, T) {
        (self.0, self.1)
    }
}

/// Compares two pairs while disregarding the order of the contained items
impl<T> PartialEq for UnorderedPair<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &UnorderedPair<T>) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

/// Computes the same hash regardless of the order of the contained items
impl<T> Hash for UnorderedPair<T>
where
    T: Ord + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let UnorderedPair(first, second) = self;

        match first.cmp(second) {
            Ordering::Greater => {
                second.hash(state);
                first.hash(state);
            }
            _ => {
                first.hash(state);
                second.hash(state);
            }
        }
    }
}
