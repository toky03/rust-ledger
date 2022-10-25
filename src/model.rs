use std::fs::File;

use self::definition_control::from_ledger_definition;

mod definition_control;
mod ledger;
mod transaction_control;

pub fn read_control(file: File) {
    let ledger: ledger::Ledger =
        serde_yaml::from_reader(file).expect("could not deserialize ledger");
    let accounts = from_ledger_definition(&ledger.definition);
    transaction_control::check_transactins(&ledger.transactions, &accounts);
}

pub fn read_ledger(file: File) -> ledger::Ledger {
    serde_yaml::from_reader(file).expect("could not deserialize ledger")
}
