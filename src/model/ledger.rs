mod amount;
mod definition;
mod transaction;

pub use amount::Amount;
pub use definition::Account;
pub use definition::AccountsReader;
pub use definition::LedgerDefinition;
use serde::Deserialize;
use serde::Serialize;
pub use transaction::TransactionLedger;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ledger {
    name: String,
    definition: LedgerDefinition,
    transactions: Vec<Transaction>,
}

#[cfg(test)]
pub use definition::MockLedgerDefinition;

use self::transaction::Transaction;
