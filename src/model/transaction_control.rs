use crate::model::definition_control::{AccountType, LedgerAccount};
use crate::model::entity::{AmountEntity, LedgerEntity, TransactionEntity};
use std::collections::HashMap;

struct CalculatedAccount {
    name: String,
    current_ammount: AmountEntity,
    account_type: AccountType,
}

struct TransactionResult {
    accounts: HashMap<String, CalculatedAccount>,
    ledger_definition: LedgerEntity,
}

pub fn check_transactins(
    transactions: &Vec<TransactionEntity>,
    accounts: &HashMap<String, LedgerAccount>,
) {
}
