use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
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

impl<T> PartialEq for UnorderedPair<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &UnorderedPair<T>) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

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
