use crate::model::entity::TransactionAccountReader;
use std::fs::File;

mod core;
mod entity;
pub mod error;

pub fn read_control(file: File) {
    let ledger: entity::Entity =
        serde_yaml::from_reader(file).expect("could not deserialize entity");
    let accounts = core::from_ledger_definition(&ledger.definition);
    let transactions = ledger
        .transactions
        .into_iter()
        .map(|transaction| Box::new(transaction) as Box<dyn TransactionAccountReader>)
        .collect();
    core::check_transactions(&transactions, &accounts);
}

pub fn read_ledger(file: File) -> entity::Entity {
    serde_yaml::from_reader(file).expect("could not deserialize entity")
}
