use crate::NameProvider;
use myelin_object_data::Kind;
use nameof::name_of;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::{self, Debug};

/// A [`NameProvider`] that uses names only once
#[derive(Debug)]
pub struct NameProviderImpl {
    names: HashMap<Kind, Vec<String>>,
}

impl NameProviderImpl {
    /// Creates a new [`NameProviderImpl`]
    pub fn new(names: HashMap<Kind, Vec<String>>) -> Self {
        Self { names }
    }
}

/// Creates a new [`NameProvider`] from a list of names
pub trait NameProviderFactory {
    /// Creates a new [`NameProvider`] from a list of names
    fn create(&self, names: HashMap<Kind, Vec<String>>) -> Box<dyn NameProvider>;
}

impl<T> NameProviderFactory for T
where
    T: Fn(HashMap<Kind, Vec<String>>) -> Box<dyn NameProvider>,
{
    fn create(&self, names: HashMap<Kind, Vec<String>>) -> Box<dyn NameProvider> {
        (self)(names)
    }
}

/// Shuffles the names before creating a [`NameProvider`]
#[derive(Debug, Default)]
pub struct ShuffledNameProviderFactory;

impl NameProviderFactory for ShuffledNameProviderFactory {
    fn create(&self, mut names: HashMap<Kind, Vec<String>>) -> Box<dyn NameProvider> {
        let mut rng = thread_rng();

        for name_list in names.values_mut() {
            name_list.shuffle(&mut rng);
        }

        box NameProviderImpl::new(names)
    }
}

/// Provides names read from filess
pub struct NameProviderBuilder {
    names: HashMap<Kind, Vec<String>>,
    name_provider_factory: Box<dyn NameProviderFactory>,
}

impl Debug for NameProviderBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type NameProviderBuilder))
            .field(name_of!(names in Self), &self.names)
            .finish()
    }
}

impl NameProviderBuilder {
    /// Creates a new [`NameProviderBuilder`]
    pub fn new(name_provider_factory: Box<dyn NameProviderFactory>) -> Self {
        Self {
            name_provider_factory,
            names: HashMap::new(),
        }
    }

    /// Add names from a file for a certain kind of object
    pub fn add_names(&mut self, names: &[String], kind: Kind) {
        self.names.entry(kind).or_default().extend_from_slice(names);
    }

    /// Build
    pub fn build(self) -> Box<dyn NameProvider> {
        self.name_provider_factory.create(self.names)
    }
}

impl NameProvider for NameProviderImpl {
    fn get_name(&mut self, kind: Kind) -> Option<String> {
        self.names.get_mut(&kind)?.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_file_for_kind_works_with_one_name() {
        let mut builder = create_builder();

        let names = vec![String::from("Malus domestica")];
        builder.add_names(&names, Kind::Plant);

        let mut name_provider = builder.build();

        assert_eq!(None, name_provider.get_name(Kind::Organism));
        assert_eq!(None, name_provider.get_name(Kind::Terrain));
        assert_eq!(None, name_provider.get_name(Kind::Water));
        assert_eq!(
            Some(String::from("Malus domestica")),
            name_provider.get_name(Kind::Plant)
        );
        assert_eq!(None, name_provider.get_name(Kind::Plant));
    }

    fn create_builder() -> NameProviderBuilder {
        NameProviderBuilder::new(box |names| {
            box NameProviderImpl::new(names) as Box<dyn NameProvider>
        })
    }
}
