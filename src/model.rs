use std::fs::File;

pub mod control;
mod definition;

pub fn read_definition(file: File) -> definition::AccountDefinition {
    serde_yaml::from_reader(file).expect("could not deserialize account definition")
}
