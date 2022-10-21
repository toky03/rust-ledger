use std::fs::File;

mod amount;
mod control;
mod definition;
#[cfg(test)]
mod mock;
mod transaction;

pub fn read_definition(file: File) -> definition::LedgerDefinition {
    serde_yaml::from_reader(file).expect("could not deserialize account definition")
}
