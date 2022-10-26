mod amount_entity;
mod ledger_entity;
mod transaction_entity;

pub use amount_entity::AmountEntity;
pub use ledger_entity::AccountEntity;
pub use ledger_entity::AccountsReader;
pub use ledger_entity::LedgerEntity;
use serde::Deserialize;
use serde::Serialize;
pub use transaction_entity::TransactionEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    name: String,
    pub definition: LedgerEntity,
    pub transactions: Vec<TransactionEntity>,
}

#[cfg(test)]
pub use ledger_entity::MockLedgerEntity;
