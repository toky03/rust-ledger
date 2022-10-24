use std::fs::File;

mod control;
mod ledger;

pub fn read_ledger(file: File) -> ledger::Ledger {
    serde_yaml::from_reader(file).expect("could not deserialize ledger")
}
