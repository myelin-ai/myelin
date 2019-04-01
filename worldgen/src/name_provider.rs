use crate::NameProvider;
use myelin_object_data::Kind;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

struct NameProviderImpl {
    names: HashMap<Kind, Vec<String>>,
}

/// Provides names read from files
#[derive(Debug, Default)]
pub struct NameProviderBuilder {
    names: HashMap<Kind, Vec<String>>,
}

impl NameProviderBuilder {
    /// Add names from a file for a certain kind of object
    pub fn add_names(&mut self, names: &[String], kind: Kind) {
        self.names.entry(kind).or_default().extend_from_slice(names);
    }

    /// Build
    pub fn build(self) -> Box<dyn NameProvider> {
        box NameProviderImpl { names: self.names }
    }

    /// Build, but shuffle the names beforehand
    pub fn build_randomized(mut self) -> Box<dyn NameProvider> {
        let mut rng = thread_rng();

        for name_list in self.names.values_mut() {
            name_list.shuffle(&mut rng);
        }

        self.build()
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
        let mut builder = NameProviderBuilder::default();

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
}
