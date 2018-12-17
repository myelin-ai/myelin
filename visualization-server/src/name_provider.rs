use myelin_environment::object::Kind;
use myelin_worldgen::generator::NameProvider;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::io;
use rand::thread_rng;
use std::path::Path;
use rand::seq::SliceRandom;

struct FileSystemNameProvider {
    names: HashMap<Kind, Vec<String>>,
}

#[derive(Default)]
pub struct FileSystemNameProviderBuilder {
    names: HashMap<Kind, Vec<String>>,
}

impl FileSystemNameProviderBuilder {
    fn add_file_for_kind(&mut self, path: &Path, kind: Kind) -> io::Result<()> {
        let contents = read_to_string(&path)?;
        let new_names = contents.lines().map(String::from);
        self.names.entry(kind).or_default().extend(new_names);
        Ok(())
    }

    fn build(self) -> Box<dyn NameProvider> {
        Box::new(FileSystemNameProvider { names: self.names })
    }

    fn build_randomized(mut self) -> Box<dyn NameProvider> {
        let mut rng = thread_rng();
        self.names.values_mut().map(|e| e.shuffle(&mut rng));
        self.build()
    }
}

impl NameProvider for FileSystemNameProvider {
    fn get_name(&mut self, kind: Kind) -> Option<String> {
        self.names.get_mut(&kind)?.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_file_for_kind_works_with_one_name() {
        let mut builder = FileSystemNameProviderBuilder::default();

        let path = Path::new("./tests/object_names/plants.txt");
        builder.add_file_for_kind(path, Kind::Plant).expect("Error while reading file");

        let mut name_provider = builder.build();

        assert_eq!(None, name_provider.get_name(Kind::Organism));
        assert_eq!(None, name_provider.get_name(Kind::Terrain));
        assert_eq!(None, name_provider.get_name(Kind::Water));
        assert_eq!(Some(String::from("Malus domestica")), name_provider.get_name(Kind::Plant));
        assert_eq!(None, name_provider.get_name(Kind::Plant));
    }
}
