use std::fs::read_to_string;
use std::path::Path;
use myelin_environment::object::Kind;
use myelin_worldgen::generator::NameProvider;
use std::collections::HashMap;
use std::error::Error;
use std::io;

struct FileSystemNameProvider {
    names: HashMap<Kind, Vec<String>>,
}

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

    fn build_randomized(self) -> Box<dyn NameProvider> {
        unimplemented!()
    }
}

impl NameProvider for FileSystemNameProvider {
    fn get_name(&mut self, kind: Kind) -> Option<String> {
        self.names.get_mut(&kind)?.pop()
    }
}
