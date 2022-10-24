use std::fs::File;

mod control;
mod ledger;

pub fn read_definition(file: File) -> ledger::LedgerDefinition {
    serde_yaml::from_reader(file).expect("could not deserialize account definition")
}
